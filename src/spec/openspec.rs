//! OpenSpec adapter implementing the SpecAdapter trait.
//!
//! This adapter reads completed OpenSpec changes and converts them
//! to spec domain types (Story, Task, Scenario).

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use fs2::FileExt;
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::spec::{Context, Scenario, SpecAdapter, Story, Task, VerifyCommands};

/// Information about an OpenSpec change.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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
#[allow(dead_code)]
struct StatusResponse {
    #[serde(rename = "changeName")]
    change_name: String,
    #[serde(rename = "isComplete")]
    is_complete: bool,
}

/// OpenSpec adapter that provides spec domain types from OpenSpec change data.
pub struct OpenSpecAdapter {
    #[allow(dead_code)]
    change_name: String,
    change_dir: PathBuf,
    stories: Vec<Story>,
    scenarios: Vec<Scenario>,
}

impl OpenSpecAdapter {
    /// Creates a new OpenSpecAdapter for the given change.
    ///
    /// Loads and parses all task and spec data from the change directory.
    pub fn new(change_name: &str) -> Result<Self> {
        // Verify the change exists by getting its status
        Self::get_status(change_name)?;

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
        let scenarios = if specs_dir.exists() {
            Self::parse_specs_dir(&specs_dir)?
        } else {
            Vec::new()
        };

        Ok(Self {
            change_name: change_name.to_string(),
            change_dir,
            stories,
            scenarios,
        })
    }

    /// Returns the change directory path.
    #[allow(dead_code)]
    pub fn change_dir(&self) -> &Path {
        &self.change_dir
    }

    /// Lists all available changes.
    pub fn list_changes() -> Result<Vec<ChangeInfo>> {
        let output = run_openspec_command(&["list", "--json"])?;
        let response: ListResponse = serde_json::from_str(&output)?;
        Ok(response.changes)
    }

    /// Checks if a change is complete.
    pub fn is_complete(change_name: &str) -> Result<bool> {
        let status = Self::get_status(change_name)?;
        Ok(status.is_complete)
    }

    fn get_status(change_name: &str) -> Result<StatusResponse> {
        let output = run_openspec_command(&["status", "--change", change_name, "--json"])?;
        let response: StatusResponse = serde_json::from_str(&output)?;
        Ok(response)
    }

    fn get_change_dir(change_name: &str) -> Result<PathBuf> {
        // OpenSpec stores changes in openspec/changes/<name>/
        let cwd = std::env::current_dir()?;
        let change_dir = cwd.join("openspec").join("changes").join(change_name);
        if !change_dir.exists() {
            return Err(Error::ChangeNotFound(change_name.to_string()));
        }
        Ok(change_dir)
    }

    fn parse_tasks_file(path: &Path) -> Result<Vec<Story>> {
        let content = fs::read_to_string(path)?;
        parse_tasks_md(&content)
    }

    fn parse_specs_dir(specs_dir: &Path) -> Result<Vec<Scenario>> {
        let mut scenarios = Vec::new();

        // Read all spec.md files in subdirectories
        if let Ok(entries) = fs::read_dir(specs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let spec_file = path.join("spec.md");
                    if spec_file.exists() {
                        let content = fs::read_to_string(&spec_file)?;
                        let parsed = parse_spec_md(&content)?;
                        scenarios.extend(parsed);
                    }
                }
            }
        }

        Ok(scenarios)
    }
}

/// Runs an openspec CLI command and returns stdout.
fn run_openspec_command(args: &[&str]) -> Result<String> {
    let output = Command::new("openspec")
        .args(args)
        .output()
        .map_err(|e| Error::Command {
            cmd: format!("openspec {}", args.join(" ")),
            stderr: e.to_string(),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Command {
            cmd: format!("openspec {}", args.join(" ")),
            stderr: stderr.trim().to_string(),
        });
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| Error::Parse(format!("Invalid UTF-8 in command output: {}", e)))?;
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
    let (done, rest) = if let Some(rest) = line.strip_prefix("- [x] ") {
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
        done,
    })
}

/// Parses a spec.md file into Scenarios.
///
/// Format:
/// - `### Requirement: Name` → Story ID derived from name
/// - `#### Scenario: Name` → Scenario (belongs to preceding requirement)
fn parse_spec_md(content: &str) -> Result<Vec<Scenario>> {
    let mut scenarios = Vec::new();
    let mut current_story_id = String::new();
    let mut current_scenario: Option<(String, Vec<String>, String, Vec<String>)> = None;
    let mut in_scenario = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Parse requirement headers: ### Requirement: Name
        if let Some(rest) = trimmed.strip_prefix("### Requirement: ") {
            // Save current scenario if any
            if let Some((name, given, when, then)) = current_scenario.take() {
                scenarios.push(Scenario {
                    name,
                    story_id: current_story_id.clone(),
                    given,
                    when,
                    then,
                });
            }

            // Derive story_id from requirement name
            let title = rest.trim();
            current_story_id = title
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect();
            in_scenario = false;
        }
        // Parse scenario headers: #### Scenario: Name
        else if let Some(rest) = trimmed.strip_prefix("#### Scenario: ") {
            // Save previous scenario if any
            if let Some((name, given, when, then)) = current_scenario.take() {
                scenarios.push(Scenario {
                    name,
                    story_id: current_story_id.clone(),
                    given,
                    when,
                    then,
                });
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
    }

    // Save final scenario
    if let Some((name, given, when, then)) = current_scenario.take() {
        scenarios.push(Scenario {
            name,
            story_id: current_story_id,
            given,
            when,
            then,
        });
    }

    Ok(scenarios)
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

// ============================================================================
// SpecAdapter Implementation
// ============================================================================

impl SpecAdapter for OpenSpecAdapter {
    fn stories(&self) -> Result<Vec<Story>> {
        Ok(self.stories.clone())
    }

    fn scenarios(&self) -> Result<Vec<Scenario>> {
        Ok(self.scenarios.clone())
    }

    fn context(&self, story_id: &str) -> Result<Context> {
        // Find the story
        let story = self
            .stories
            .iter()
            .find(|s| s.id == story_id)
            .ok_or_else(|| Error::StoryNotFound(story_id.to_string()))?
            .clone();

        // Read proposal
        let proposal_path = self.change_dir.join("proposal.md");
        let proposal = fs::read_to_string(&proposal_path).unwrap_or_default();

        // Read design
        let design_path = self.change_dir.join("design.md");
        let design = fs::read_to_string(&design_path).unwrap_or_default();

        // Get all scenarios
        let scenarios = self.scenarios.clone();

        // Infer verification commands
        let verify = infer_verify_commands()?;

        Ok(Context {
            story,
            proposal,
            design,
            scenarios,
            verify,
        })
    }

    fn mark_done(&mut self, task_id: &str) -> Result<()> {
        // First, update in-memory state
        let mut found = false;
        for story in &mut self.stories {
            for task in &mut story.tasks {
                if task.id == task_id {
                    task.done = true;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return Err(Error::TaskNotFound(task_id.to_string()));
        }

        // Persist to tasks.md
        let tasks_path = self.change_dir.join("tasks.md");
        if !tasks_path.exists() {
            return Err(Error::Parse(format!(
                "tasks.md not found at: {}",
                tasks_path.display()
            )));
        }

        // Open file with locking
        let file = File::options()
            .read(true)
            .write(true)
            .open(&tasks_path)?;

        file.lock_exclusive()
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

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
                file.unlock()
                    .map_err(|e| Error::Io(std::io::Error::other(e)))?;
                return Ok(());
            }
            file.unlock()
                .map_err(|e| Error::Io(std::io::Error::other(e)))?;
            return Err(Error::TaskNotFound(format!(
                "Task '{}' not found in tasks.md",
                task_id
            )));
        }

        let new_content = content.replace(&unchecked, &checked);

        file.unlock()
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;
        fs::write(&tasks_path, new_content)?;

        Ok(())
    }

    fn append_learnings(&mut self, learnings: &[String]) -> Result<()> {
        if learnings.is_empty() {
            return Ok(());
        }

        let design_path = self.change_dir.join("design.md");
        if !design_path.exists() {
            return Err(Error::Parse(format!(
                "design.md not found at: {}",
                design_path.display()
            )));
        }

        // Open file with locking
        let file = File::options()
            .read(true)
            .write(true)
            .open(&design_path)?;

        file.lock_exclusive()
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;

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

        // Format learnings
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let mut learnings_text = format!("\n### {}\n", today);
        for learning in learnings {
            learnings_text.push_str(&format!("- {}\n", learning));
        }

        // Append learnings
        content.push_str(&learnings_text);

        file.unlock()
            .map_err(|e| Error::Io(std::io::Error::other(e)))?;
        fs::write(&design_path, content)?;

        Ok(())
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
        assert!(!task.done);
    }

    #[test]
    fn parse_task_complete() {
        let task = parse_task_line("- [x] 2.3 Implement feature").unwrap();
        assert_eq!(task.id, "2.3");
        assert_eq!(task.description, "Implement feature");
        assert!(task.done);
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
        assert!(!stories[0].tasks[0].done);
        assert!(stories[0].tasks[1].done);

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
        assert_eq!(
            extract_step("- **WHEN** user clicks button"),
            "user clicks button"
        );
    }

    #[test]
    fn extract_step_then() {
        assert_eq!(extract_step("- **THEN** result is shown"), "result is shown");
    }
}
