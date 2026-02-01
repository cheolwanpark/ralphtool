//! Verification commands for the verification agent phase of Ralph Loop.
//!
//! These commands provide context and control for a verification agent to
//! validate that implementation matches specifications.

use anyhow::Result;
use serde::Serialize;

use crate::agent::session::{get_session_id, get_story_id, load_session};
use crate::spec::openspec::OpenSpecAdapter;
use crate::spec::{ScenarioSource, StorySource, TaskSource};

use super::cli::VerifyCommand;
use super::context::VerifyCommands;

/// Response from the verify context command.
#[derive(Debug, Serialize)]
pub struct VerifyContextResponse {
    /// All user stories/requirements from specs.
    pub user_stories: Vec<UserStoryContext>,

    /// All scenarios from specs.
    pub scenarios: Vec<ScenarioContext>,

    /// Completed tasks from the current session.
    pub completed_tasks: Vec<CompletedTaskContext>,

    /// Proposal content.
    pub proposal: String,

    /// Design content.
    pub design: String,

    /// Verification commands.
    pub verify: VerifyCommands,
}

/// User story context for verification.
#[derive(Debug, Serialize)]
pub struct UserStoryContext {
    pub id: String,
    pub title: String,
    pub description: String,
    pub acceptance_criteria: Vec<String>,
    pub passed: bool,
}

/// Scenario context for verification.
#[derive(Debug, Serialize)]
pub struct ScenarioContext {
    pub name: String,
    pub given: Vec<String>,
    pub when: String,
    pub then: Vec<String>,
}

/// Completed task context.
#[derive(Debug, Serialize)]
pub struct CompletedTaskContext {
    pub id: String,
    pub description: String,
}

/// Response from the verify pass command.
#[derive(Debug, Serialize)]
pub struct VerifyPassResponse {
    pub success: bool,
    pub story_id: String,
}

/// Runs a verify subcommand.
pub fn run(command: VerifyCommand) -> Result<()> {
    match command {
        VerifyCommand::Context => run_verify_context(),
        VerifyCommand::Pass => run_verify_pass(),
    }
}

/// Runs the verify context command.
fn run_verify_context() -> Result<()> {
    let session_id = get_session_id()?;
    let session = load_session(&session_id)?;

    let adapter = OpenSpecAdapter::new(&session.change_name)?;

    // Get all user stories
    let user_stories: Vec<UserStoryContext> = adapter
        .list_stories()?
        .into_iter()
        .map(|s| UserStoryContext {
            id: s.id,
            title: s.title,
            description: s.description,
            acceptance_criteria: s.acceptance_criteria,
            passed: s.passed,
        })
        .collect();

    // Get all scenarios
    let scenarios: Vec<ScenarioContext> = adapter
        .list_scenarios()?
        .into_iter()
        .map(|s| ScenarioContext {
            name: s.name,
            given: s.given,
            when: s.when,
            then: s.then,
        })
        .collect();

    // Get completed tasks from session
    let all_stories = adapter.list_tasks()?;
    let completed_tasks: Vec<CompletedTaskContext> = all_stories
        .iter()
        .flat_map(|story| &story.tasks)
        .filter(|task| task.complete)
        .map(|task| CompletedTaskContext {
            id: task.id.clone(),
            description: task.description.clone(),
        })
        .collect();

    // Read proposal
    let proposal_path = adapter.change_dir().join("proposal.md");
    let proposal = std::fs::read_to_string(&proposal_path).unwrap_or_default();

    // Read design
    let design_path = adapter.change_dir().join("design.md");
    let design = std::fs::read_to_string(&design_path).unwrap_or_default();

    // Infer verify commands
    let verify = infer_verify_commands();

    let response = VerifyContextResponse {
        user_stories,
        scenarios,
        completed_tasks,
        proposal,
        design,
        verify,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

/// Runs the verify pass command.
fn run_verify_pass() -> Result<()> {
    let session_id = get_session_id()?;
    let session = load_session(&session_id)?;
    let story_id = get_story_id()?;

    let mut adapter = OpenSpecAdapter::new(&session.change_name)?;

    // Mark the story as passed
    adapter.mark_passed(&story_id)?;

    let response = VerifyPassResponse {
        success: true,
        story_id,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

/// Infers verification commands from project type.
fn infer_verify_commands() -> VerifyCommands {
    let cwd = std::env::current_dir().unwrap_or_default();

    // Check for Cargo.toml (Rust project)
    if cwd.join("Cargo.toml").exists() {
        return VerifyCommands {
            checks: vec![
                "cargo check".to_string(),
                "cargo clippy -- -D warnings".to_string(),
            ],
            tests: "cargo test".to_string(),
        };
    }

    // Check for package.json (Node.js project)
    if cwd.join("package.json").exists() {
        return VerifyCommands {
            checks: vec!["npm run lint".to_string()],
            tests: "npm test".to_string(),
        };
    }

    // Check for pyproject.toml or setup.py (Python project)
    if cwd.join("pyproject.toml").exists() || cwd.join("setup.py").exists() {
        return VerifyCommands {
            checks: vec!["python -m mypy .".to_string()],
            tests: "python -m pytest".to_string(),
        };
    }

    // Default/fallback
    VerifyCommands {
        checks: Vec::new(),
        tests: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_context_response_serializes_correctly() {
        let response = VerifyContextResponse {
            user_stories: vec![UserStoryContext {
                id: "auth-flow".to_string(),
                title: "Authentication Flow".to_string(),
                description: "User can log in".to_string(),
                acceptance_criteria: vec!["Login works".to_string()],
                passed: false,
            }],
            scenarios: vec![ScenarioContext {
                name: "Valid login".to_string(),
                given: vec!["user exists".to_string()],
                when: "user enters credentials".to_string(),
                then: vec!["user is logged in".to_string()],
            }],
            completed_tasks: vec![CompletedTaskContext {
                id: "1.1".to_string(),
                description: "Setup project".to_string(),
            }],
            proposal: "# Proposal".to_string(),
            design: "# Design".to_string(),
            verify: VerifyCommands {
                checks: vec!["cargo check".to_string()],
                tests: "cargo test".to_string(),
            },
        };

        let json = serde_json::to_string(&response).unwrap();

        // Verify all fields are present
        assert!(json.contains("user_stories"));
        assert!(json.contains("scenarios"));
        assert!(json.contains("completed_tasks"));
        assert!(json.contains("proposal"));
        assert!(json.contains("design"));
        assert!(json.contains("verify"));

        // Verify nested data
        assert!(json.contains("auth-flow"));
        assert!(json.contains("Authentication Flow"));
        assert!(json.contains("Valid login"));
        assert!(json.contains("1.1"));
        assert!(json.contains("cargo check"));
    }

    #[test]
    fn verify_pass_response_serializes_correctly() {
        let response = VerifyPassResponse {
            success: true,
            story_id: "auth-flow".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"story_id\":\"auth-flow\""));
    }

    #[test]
    fn user_story_context_has_all_fields() {
        let ctx = UserStoryContext {
            id: "test-id".to_string(),
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            acceptance_criteria: vec!["Criterion 1".to_string(), "Criterion 2".to_string()],
            passed: true,
        };

        assert_eq!(ctx.id, "test-id");
        assert_eq!(ctx.title, "Test Title");
        assert_eq!(ctx.description, "Test Description");
        assert_eq!(ctx.acceptance_criteria.len(), 2);
        assert!(ctx.passed);
    }

    #[test]
    fn scenario_context_has_all_fields() {
        let ctx = ScenarioContext {
            name: "Test Scenario".to_string(),
            given: vec!["precondition 1".to_string(), "precondition 2".to_string()],
            when: "action happens".to_string(),
            then: vec!["result 1".to_string(), "result 2".to_string()],
        };

        assert_eq!(ctx.name, "Test Scenario");
        assert_eq!(ctx.given.len(), 2);
        assert_eq!(ctx.when, "action happens");
        assert_eq!(ctx.then.len(), 2);
    }
}
