//! OpenSpec adapter implementing spec abstraction traits.
//!
//! This adapter reads completed OpenSpec changes and converts them
//! to spec domain types (Story, Task, UserStory, Scenario).

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use fs2::FileExt;
use serde::Deserialize;

use crate::spec::{
    ContextProvider, Learning, Pattern, Priority, Scenario, ScenarioSource, SpecWriter, Story,
    StorySource, Task, TaskSource, UserStory, VerifyCommands, WorkContext, WorkStatus,
};

/// Information about an OpenSpec change.
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeInfo {
    pub name: String,
    #[serde(rename = "completedTasks")]
    pub completed_tasks: usize,
    #[serde(rename = "totalTasks")]
    pub total_tasks: usize,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub status: String,
}

/// Response from `openspec list --json`.
#[derive(Debug, Deserialize)]
struct ListResponse {
    changes: Vec<ChangeInfo>,
}

/// Response from `openspec status --change <name> --json`.
#[derive(Debug, Deserialize)]
struct StatusResponse {
    #[serde(rename = "changeName")]
    change_name: String,
    #[serde(rename = "isComplete")]
    is_complete: bool,
}

/// OpenSpec adapter that provides spec domain types from OpenSpec change data.
pub struct OpenSpecAdapter {
    change_name: String,
    change_dir: PathBuf,
    stories: Vec<Story>,
    user_stories: Vec<UserStory>,
    scenarios: Vec<Scenario>,
    /// Maps scenario name to the story_id it belongs to
    scenario_to_story: HashMap<String, String>,
}

impl OpenSpecAdapter {
    /// Creates a new OpenSpecAdapter for the given change.
    ///
    /// Loads and parses all task and spec data from the change directory.
    pub fn new(change_name: &str) -> Result<Self> {
        // Verify the change exists by getting its status
        let _status = Self::get_status(change_name)?;

        // Determine change directory
        let change_dir = Self::get_change_dir(change_name)?;

        // Parse tasks.md
        let tasks_path = change_dir.join("tasks.md");
        let stories = if tasks_path.exists() {
            Self::parse_tasks_file(&tasks_path)?
        } else {
            Vec::new()
        };

        // Parse specs
        let specs_dir = change_dir.join("specs");
        let (user_stories, scenarios, scenario_to_story) = if specs_dir.exists() {
            Self::parse_specs_dir(&specs_dir)?
        } else {
            (Vec::new(), Vec::new(), HashMap::new())
        };

        Ok(Self {
            change_name: change_name.to_string(),
            change_dir,
            stories,
            user_stories,
            scenarios,
            scenario_to_story,
        })
    }

    /// Returns the change name.
    pub fn change_name(&self) -> &str {
        &self.change_name
    }

    /// Returns the change directory path.
    pub fn change_dir(&self) -> &Path {
        &self.change_dir
    }

    /// Returns scenarios that belong to a specific user story.
    pub fn scenarios_for_story(&self, story_id: &str) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|scenario| {
                self.scenario_to_story
                    .get(&scenario.name)
                    .is_some_and(|sid| sid == story_id)
            })
            .collect()
    }

    /// Returns the scenario-to-story mapping.
    pub fn scenario_to_story_map(&self) -> &HashMap<String, String> {
        &self.scenario_to_story
    }

    /// Lists all available changes.
    pub fn list_changes() -> Result<Vec<ChangeInfo>> {
        let output = run_openspec_command(&["list", "--json"])?;
        let response: ListResponse =
            serde_json::from_str(&output).context("Failed to parse openspec list output")?;
        Ok(response.changes)
    }

    /// Checks if a change is complete.
    pub fn is_complete(change_name: &str) -> Result<bool> {
        let status = Self::get_status(change_name)?;
        Ok(status.is_complete)
    }

    fn get_status(change_name: &str) -> Result<StatusResponse> {
        let output = run_openspec_command(&["status", "--change", change_name, "--json"])?;
        let response: StatusResponse =
            serde_json::from_str(&output).context("Failed to parse openspec status output")?;
        Ok(response)
    }

    fn get_change_dir(change_name: &str) -> Result<PathBuf> {
        // OpenSpec stores changes in openspec/changes/<name>/
        let cwd = std::env::current_dir()?;
        let change_dir = cwd.join("openspec").join("changes").join(change_name);
        if !change_dir.exists() {
            return Err(anyhow!("Change directory not found: {}", change_dir.display()));
        }
        Ok(change_dir)
    }

    fn parse_tasks_file(path: &Path) -> Result<Vec<Story>> {
        let content = fs::read_to_string(path).context("Failed to read tasks.md")?;
        parse_tasks_md(&content)
    }

    fn parse_specs_dir(
        specs_dir: &Path,
    ) -> Result<(Vec<UserStory>, Vec<Scenario>, HashMap<String, String>)> {
        let mut user_stories = Vec::new();
        let mut scenarios = Vec::new();
        let mut scenario_to_story = HashMap::new();

        // Read all spec.md files in subdirectories
        if let Ok(entries) = fs::read_dir(specs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let spec_file = path.join("spec.md");
                    if spec_file.exists() {
                        let content = fs::read_to_string(&spec_file)?;
                        let (stories, scene, mapping) = parse_spec_md(&content)?;
                        user_stories.extend(stories);
                        scenarios.extend(scene);
                        scenario_to_story.extend(mapping);
                    }
                }
            }
        }

        Ok((user_stories, scenarios, scenario_to_story))
    }
}

/// Runs an openspec CLI command and returns stdout.
fn run_openspec_command(args: &[&str]) -> Result<String> {
    let output = Command::new("openspec")
        .args(args)
        .output()
        .context("Failed to execute openspec command. Is OpenSpec CLI installed and in PATH?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "openspec command failed: {}",
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8(output.stdout).context("Invalid UTF-8 in command output")?;
    Ok(stdout)
}

/// Parses tasks.md content into Story hierarchy.
///
/// Format:
/// - `## N. Title` → Story with id "N" and title "Title"
/// - `- [ ] N.M Description` → Incomplete task with id "N.M"
/// - `- [x] N.M Description` → Complete task with id "N.M"
fn parse_tasks_md(content: &str) -> Result<Vec<Story>> {
    let mut stories: Vec<Story> = Vec::new();
    let mut current_story: Option<Story> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        // Parse story headers: ## N. Title
        if let Some(rest) = trimmed.strip_prefix("## ") {
            // Save current story if any
            if let Some(story) = current_story.take() {
                stories.push(story);
            }

            // Parse "N. Title" format
            if let Some((id, title)) = parse_story_header(rest) {
                current_story = Some(Story {
                    id,
                    title,
                    tasks: Vec::new(),
                });
            }
        }
        // Parse task checkboxes: - [ ] N.M Description or - [x] N.M Description
        else if let Some(task) = parse_task_line(trimmed) {
            if let Some(ref mut story) = current_story {
                story.tasks.push(task);
            }
        }
    }

    // Save final story
    if let Some(story) = current_story {
        stories.push(story);
    }

    Ok(stories)
}

/// Parses a story header like "1. Project Setup" into (id, title).
fn parse_story_header(text: &str) -> Option<(String, String)> {
    let mut parts = text.splitn(2, ". ");
    let id = parts.next()?.trim().to_string();
    let title = parts.next()?.trim().to_string();

    // Verify id is numeric
    if id.chars().all(|c| c.is_ascii_digit()) {
        Some((id, title))
    } else {
        None
    }
}

/// Parses a task line like "- [ ] 1.1 Description" into a Task.
fn parse_task_line(line: &str) -> Option<Task> {
    let (complete, rest) = if let Some(rest) = line.strip_prefix("- [x] ") {
        (true, rest)
    } else if let Some(rest) = line.strip_prefix("- [X] ") {
        (true, rest)
    } else if let Some(rest) = line.strip_prefix("- [ ] ") {
        (false, rest)
    } else {
        return None;
    };

    // Parse "N.M Description" format
    let mut parts = rest.splitn(2, ' ');
    let id = parts.next()?.trim().to_string();
    let description = parts.next().unwrap_or("").trim().to_string();

    Some(Task {
        id,
        description,
        complete,
    })
}

/// Parses a spec.md file into UserStories and Scenarios.
///
/// Format:
/// - `### Requirement: Name` → UserStory
/// - `#### Scenario: Name` → Scenario (belongs to preceding requirement)
fn parse_spec_md(
    content: &str,
) -> Result<(Vec<UserStory>, Vec<Scenario>, HashMap<String, String>)> {
    let mut user_stories = Vec::new();
    let mut scenarios = Vec::new();
    let mut scenario_to_story = HashMap::new();

    let mut current_story: Option<UserStory> = None;
    let mut current_scenario: Option<(String, Vec<String>, String, Vec<String>)> = None;
    let mut in_scenario = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Parse requirement headers: ### Requirement: Name
        if let Some(rest) = trimmed.strip_prefix("### Requirement: ") {
            // Save current scenario if any
            if let Some((name, given, when, then)) = current_scenario.take() {
                let scenario = Scenario { name: name.clone(), given, when, then };
                if let Some(ref story) = current_story {
                    scenario_to_story.insert(name, story.id.clone());
                }
                scenarios.push(scenario);
            }

            // Save current story if any
            if let Some(story) = current_story.take() {
                user_stories.push(story);
            }

            let title = rest.trim().to_string();
            let id = title
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect();

            current_story = Some(UserStory {
                id,
                title,
                description: String::new(),
                acceptance_criteria: Vec::new(),
                priority: Priority::Medium,
                passed: false,
            });
            in_scenario = false;
        }
        // Parse scenario headers: #### Scenario: Name
        else if let Some(rest) = trimmed.strip_prefix("#### Scenario: ") {
            // Save previous scenario if any
            if let Some((name, given, when, then)) = current_scenario.take() {
                let scenario = Scenario { name: name.clone(), given, when, then };
                if let Some(ref story) = current_story {
                    scenario_to_story.insert(name.clone(), story.id.clone());
                    // Add scenario as acceptance criteria
                    current_story.as_mut().unwrap().acceptance_criteria.push(name);
                }
                scenarios.push(scenario);
            }

            let name = rest.trim().to_string();
            current_scenario = Some((name, Vec::new(), String::new(), Vec::new()));
            in_scenario = true;
        }
        // Parse Given/When/Then within scenarios
        else if in_scenario {
            if let Some((ref _name, ref mut given, ref mut when, ref mut then)) = current_scenario {
                let upper = trimmed.to_uppercase();
                if upper.starts_with("- **GIVEN") || upper.starts_with("- GIVEN") {
                    let step = extract_step(trimmed);
                    if !step.is_empty() {
                        given.push(step);
                    }
                } else if upper.starts_with("- **WHEN") || upper.starts_with("- WHEN") {
                    *when = extract_step(trimmed);
                } else if upper.starts_with("- **THEN") || upper.starts_with("- THEN") {
                    let step = extract_step(trimmed);
                    if !step.is_empty() {
                        then.push(step);
                    }
                } else if upper.starts_with("- **AND") || upper.starts_with("- AND") {
                    // AND after THEN goes to then
                    let step = extract_step(trimmed);
                    if !step.is_empty() {
                        then.push(step);
                    }
                }
            }
        }
        // Collect description lines for current story
        else if let Some(ref mut story) = current_story {
            if trimmed.starts_with("The ") && story.description.is_empty() {
                story.description = trimmed.to_string();
            }
        }
    }

    // Save final scenario
    if let Some((name, given, when, then)) = current_scenario.take() {
        let scenario = Scenario { name: name.clone(), given, when, then };
        if let Some(ref mut story) = current_story {
            scenario_to_story.insert(name.clone(), story.id.clone());
            story.acceptance_criteria.push(name);
        }
        scenarios.push(scenario);
    }

    // Save final story
    if let Some(story) = current_story.take() {
        user_stories.push(story);
    }

    Ok((user_stories, scenarios, scenario_to_story))
}

/// Extracts the step text from a Given/When/Then line.
fn extract_step(line: &str) -> String {
    // Remove leading "- **GIVEN**" or similar markers
    let line = line.trim_start_matches('-').trim();
    let line = line.trim_start_matches("**").trim();

    // Find the actual content after the keyword
    let keywords = ["GIVEN", "WHEN", "THEN", "AND"];
    for kw in keywords {
        if let Some(rest) = line.strip_prefix(kw) {
            let rest = rest.trim_start_matches("**").trim();
            return rest.to_string();
        }
        let lower = kw.chars().next().unwrap().to_string() + &kw[1..].to_lowercase();
        if let Some(rest) = line.strip_prefix(&lower) {
            let rest = rest.trim_start_matches("**").trim();
            return rest.to_string();
        }
    }

    line.to_string()
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl TaskSource for OpenSpecAdapter {
    type Error = anyhow::Error;

    fn list_tasks(&self) -> Result<Vec<Story>, Self::Error> {
        Ok(self.stories.clone())
    }

    fn next_task(&self) -> Result<Option<Task>, Self::Error> {
        for story in &self.stories {
            for task in &story.tasks {
                if !task.complete {
                    return Ok(Some(task.clone()));
                }
            }
        }
        Ok(None)
    }

    fn mark_complete(&mut self, task_id: &str) -> Result<(), Self::Error> {
        // First, update in-memory state
        let mut found = false;
        for story in &mut self.stories {
            for task in &mut story.tasks {
                if task.id == task_id {
                    task.complete = true;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return Err(anyhow!("Task not found: {}", task_id));
        }

        // Persist to tasks.md
        let tasks_path = self.change_dir.join("tasks.md");
        if !tasks_path.exists() {
            return Err(anyhow!("tasks.md not found at: {}", tasks_path.display()));
        }

        // Open file with locking
        let file = File::options()
            .read(true)
            .write(true)
            .open(&tasks_path)
            .with_context(|| format!("Failed to open {}", tasks_path.display()))?;

        file.lock_exclusive()
            .with_context(|| "Failed to acquire file lock")?;

        // Read content
        let mut content = String::new();
        {
            let mut reader = std::io::BufReader::new(&file);
            reader.read_to_string(&mut content)?;
        }

        // Find and replace the task checkbox
        let unchecked = format!("- [ ] {}", task_id);
        let checked = format!("- [x] {}", task_id);

        if !content.contains(&unchecked) {
            // Task might already be complete (idempotent)
            if content.contains(&checked) {
                file.unlock()?;
                return Ok(());
            }
            file.unlock()?;
            return Err(anyhow!("Task '{}' not found in tasks.md", task_id));
        }

        let new_content = content.replace(&unchecked, &checked);

        file.unlock()?;
        fs::write(&tasks_path, new_content)?;

        Ok(())
    }
}

impl StorySource for OpenSpecAdapter {
    type Error = anyhow::Error;

    fn list_stories(&self) -> Result<Vec<UserStory>, Self::Error> {
        Ok(self.user_stories.clone())
    }

    fn next_story(&self) -> Result<Option<UserStory>, Self::Error> {
        for story in &self.user_stories {
            if !story.passed {
                return Ok(Some(story.clone()));
            }
        }
        Ok(None)
    }

    fn mark_passed(&mut self, story_id: &str) -> Result<(), Self::Error> {
        for story in &mut self.user_stories {
            if story.id == story_id {
                story.passed = true;
                return Ok(());
            }
        }
        Err(anyhow!("Story not found: {}", story_id))
    }
}

impl ScenarioSource for OpenSpecAdapter {
    type Error = anyhow::Error;

    fn scenarios_for(&self, story_id: &str) -> Result<Vec<Scenario>, Self::Error> {
        let mut result = Vec::new();
        for scenario in &self.scenarios {
            if let Some(sid) = self.scenario_to_story.get(&scenario.name) {
                if sid == story_id {
                    result.push(scenario.clone());
                }
            }
        }
        Ok(result)
    }

    fn list_scenarios(&self) -> Result<Vec<Scenario>, Self::Error> {
        Ok(self.scenarios.clone())
    }
}

impl SpecWriter for OpenSpecAdapter {
    type Error = anyhow::Error;

    fn write_learnings(&mut self, learnings: &[Learning]) -> Result<(), Self::Error> {
        if learnings.is_empty() {
            return Ok(());
        }

        let design_path = self.change_dir.join("design.md");
        if !design_path.exists() {
            return Err(anyhow!("design.md not found at: {}", design_path.display()));
        }

        // Open file with locking
        let file = File::options()
            .read(true)
            .write(true)
            .open(&design_path)
            .with_context(|| format!("Failed to open {}", design_path.display()))?;

        file.lock_exclusive()
            .with_context(|| "Failed to acquire file lock")?;

        // Read current content
        let mut content = String::new();
        {
            let mut reader = std::io::BufReader::new(&file);
            reader.read_to_string(&mut content)?;
        }

        // Check if Learnings section exists
        if !content.contains("## Learnings") {
            content.push_str("\n## Learnings\n");
        }

        // Group learnings by story
        use std::collections::BTreeMap;
        let mut by_story: BTreeMap<String, Vec<&Learning>> = BTreeMap::new();

        for learning in learnings {
            let story_id = learning
                .story_id
                .clone()
                .unwrap_or_else(|| "General".to_string());
            by_story.entry(story_id).or_default().push(learning);
        }

        // Format learnings
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let mut learnings_text = String::new();
        for (story_id, items) in by_story {
            learnings_text.push_str(&format!("\n### {} - Story {}\n", today, story_id));
            for learning in items {
                let task_ref = learning
                    .task_id
                    .as_ref()
                    .map(|id| format!(" (Task {})", id))
                    .unwrap_or_default();
                learnings_text.push_str(&format!("- {}{}\n", learning.description, task_ref));
            }
        }

        // Append learnings
        content.push_str(&learnings_text);

        file.unlock()?;
        fs::write(&design_path, content)?;

        Ok(())
    }

    fn write_patterns(&mut self, patterns: &[Pattern]) -> Result<(), Self::Error> {
        if patterns.is_empty() {
            return Ok(());
        }

        let design_path = self.change_dir.join("design.md");
        if !design_path.exists() {
            return Err(anyhow!("design.md not found at: {}", design_path.display()));
        }

        // Open file with locking
        let file = File::options()
            .read(true)
            .write(true)
            .open(&design_path)
            .with_context(|| format!("Failed to open {}", design_path.display()))?;

        file.lock_exclusive()
            .with_context(|| "Failed to acquire file lock")?;

        // Read current content
        let mut content = String::new();
        {
            let mut reader = std::io::BufReader::new(&file);
            reader.read_to_string(&mut content)?;
        }

        // Check if Patterns section exists
        if !content.contains("## Patterns") {
            content.push_str("\n## Patterns\n");
        }

        // Format patterns
        let mut patterns_text = String::new();
        for pattern in patterns {
            patterns_text.push_str(&format!("\n### {}\n\n{}\n", pattern.name, pattern.description));
        }

        // Append patterns
        content.push_str(&patterns_text);

        file.unlock()?;
        fs::write(&design_path, content)?;

        Ok(())
    }
}

impl ContextProvider for OpenSpecAdapter {
    type Error = anyhow::Error;

    fn get_context(&self, story_id: &str) -> Result<WorkContext, Self::Error> {
        // Find the story
        let story = self
            .stories
            .iter()
            .find(|s| s.id == story_id)
            .ok_or_else(|| anyhow!("Story '{}' not found", story_id))?
            .clone();

        // Get tasks for this story
        let tasks = story.tasks.clone();

        // Read proposal
        let proposal_path = self.change_dir.join("proposal.md");
        let proposal = fs::read_to_string(&proposal_path).unwrap_or_default();

        // Read design
        let design_path = self.change_dir.join("design.md");
        let design = fs::read_to_string(&design_path).unwrap_or_default();

        // Get scenarios for this story
        let scenarios = self.scenarios_for(story_id)?;

        // Infer verification commands from project type
        let verify = infer_verify_commands()?;

        Ok(WorkContext {
            story,
            tasks,
            proposal,
            design,
            scenarios,
            verify,
        })
    }

    fn get_status(&self) -> Result<WorkStatus, Self::Error> {
        // Find current story (first incomplete one)
        let mut story_id = String::new();
        let mut story_tasks_done = 0;
        let mut story_tasks_total = 0;
        let mut change_stories_done = 0;
        let change_stories_total = self.stories.len();

        for story in &self.stories {
            let tasks_done = story.tasks.iter().filter(|t| t.complete).count();
            let all_done = tasks_done == story.tasks.len() && !story.tasks.is_empty();

            if all_done {
                change_stories_done += 1;
            } else if story_id.is_empty() {
                // First incomplete story is the current one
                story_id = story.id.clone();
                story_tasks_done = tasks_done;
                story_tasks_total = story.tasks.len();
            }
        }

        Ok(WorkStatus {
            story_id,
            story_tasks_done,
            story_tasks_total,
            change_stories_done,
            change_stories_total,
        })
    }
}

/// Infers verification commands from project type.
fn infer_verify_commands() -> Result<VerifyCommands> {
    let cwd = std::env::current_dir()?;

    // Check for Cargo.toml (Rust project)
    if cwd.join("Cargo.toml").exists() {
        return Ok(VerifyCommands {
            checks: vec![
                "cargo check".to_string(),
                "cargo clippy -- -D warnings".to_string(),
            ],
            tests: "cargo test".to_string(),
        });
    }

    // Check for package.json (Node.js project)
    if cwd.join("package.json").exists() {
        return Ok(VerifyCommands {
            checks: vec!["npm run lint".to_string()],
            tests: "npm test".to_string(),
        });
    }

    // Check for pyproject.toml or setup.py (Python project)
    if cwd.join("pyproject.toml").exists() || cwd.join("setup.py").exists() {
        return Ok(VerifyCommands {
            checks: vec!["python -m mypy .".to_string()],
            tests: "python -m pytest".to_string(),
        });
    }

    // Default/fallback
    Ok(VerifyCommands {
        checks: Vec::new(),
        tests: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_story_header_valid() {
        let result = parse_story_header("1. Project Setup");
        assert_eq!(result, Some(("1".to_string(), "Project Setup".to_string())));
    }

    #[test]
    fn parse_story_header_two_digit() {
        let result = parse_story_header("12. Large Story");
        assert_eq!(result, Some(("12".to_string(), "Large Story".to_string())));
    }

    #[test]
    fn parse_story_header_invalid() {
        assert_eq!(parse_story_header("Not a story"), None);
        assert_eq!(parse_story_header("A. Invalid"), None);
    }

    #[test]
    fn parse_task_incomplete() {
        let task = parse_task_line("- [ ] 1.1 Add serde dependency").unwrap();
        assert_eq!(task.id, "1.1");
        assert_eq!(task.description, "Add serde dependency");
        assert!(!task.complete);
    }

    #[test]
    fn parse_task_complete() {
        let task = parse_task_line("- [x] 2.3 Implement feature").unwrap();
        assert_eq!(task.id, "2.3");
        assert_eq!(task.description, "Implement feature");
        assert!(task.complete);
    }

    #[test]
    fn parse_task_invalid() {
        assert!(parse_task_line("Not a task").is_none());
        assert!(parse_task_line("- Regular list item").is_none());
    }

    #[test]
    fn parse_tasks_md_basic() {
        let content = r#"
## 1. Setup

- [ ] 1.1 First task
- [x] 1.2 Second task

## 2. Implementation

- [ ] 2.1 Another task
"#;
        let stories = parse_tasks_md(content).unwrap();
        assert_eq!(stories.len(), 2);

        assert_eq!(stories[0].id, "1");
        assert_eq!(stories[0].title, "Setup");
        assert_eq!(stories[0].tasks.len(), 2);
        assert!(!stories[0].tasks[0].complete);
        assert!(stories[0].tasks[1].complete);

        assert_eq!(stories[1].id, "2");
        assert_eq!(stories[1].title, "Implementation");
        assert_eq!(stories[1].tasks.len(), 1);
    }

    #[test]
    fn extract_step_given() {
        assert_eq!(extract_step("- **GIVEN** the user exists"), "the user exists");
        assert_eq!(extract_step("- GIVEN the user exists"), "the user exists");
    }

    #[test]
    fn extract_step_when() {
        assert_eq!(extract_step("- **WHEN** user clicks button"), "user clicks button");
    }

    #[test]
    fn extract_step_then() {
        assert_eq!(extract_step("- **THEN** result is shown"), "result is shown");
    }
}
