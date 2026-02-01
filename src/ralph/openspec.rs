//! OpenSpec adapter implementing Ralph traits.
//!
//! This adapter reads completed OpenSpec changes and converts them
//! to Ralph domain types (Story, Task, UserStory, Scenario).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::ralph::{
    Learning, Pattern, Priority, ProgressTracker, Scenario, Story, StoryProvider, Task,
    TaskSource, UserStory, VerificationSource,
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

/// OpenSpec adapter that provides Ralph domain types from OpenSpec change data.
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
        for story in &mut self.stories {
            for task in &mut story.tasks {
                if task.id == task_id {
                    task.complete = true;
                    return Ok(());
                }
            }
        }
        Err(anyhow!("Task not found: {}", task_id))
    }
}

impl StoryProvider for OpenSpecAdapter {
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

impl VerificationSource for OpenSpecAdapter {
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

impl ProgressTracker for OpenSpecAdapter {
    type Error = anyhow::Error;

    fn record_learning(&mut self, _learning: Learning) -> Result<(), Self::Error> {
        // No-op in preview mode
        Ok(())
    }

    fn record_pattern(&mut self, _pattern: Pattern) -> Result<(), Self::Error> {
        // No-op in preview mode
        Ok(())
    }

    fn list_patterns(&self) -> Result<Vec<Pattern>, Self::Error> {
        // Return empty in preview mode
        Ok(Vec::new())
    }
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
