//! Domain types for spec abstraction concepts.

// ============================================================================
// Task Hierarchy Types
// ============================================================================

/// A single actionable task within the Ralph workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// Unique identifier for the task (e.g., "1.1").
    pub id: String,
    /// Description of what needs to be done.
    pub description: String,
    /// Whether the task has been completed.
    pub done: bool,
}

/// A story containing related tasks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Story {
    /// Unique identifier for the story (e.g., "1").
    pub id: String,
    /// Title of the story.
    pub title: String,
    /// Tasks that belong to this story.
    pub tasks: Vec<Task>,
}

#[allow(dead_code)]
impl Story {
    /// Returns true if all tasks in this story are complete.
    pub fn is_complete(&self) -> bool {
        !self.tasks.is_empty() && self.tasks.iter().all(|t| t.done)
    }

    /// Returns the next incomplete task, if any.
    #[allow(dead_code)]
    pub fn next_task(&self) -> Option<&Task> {
        self.tasks.iter().find(|t| !t.done)
    }
}

// ============================================================================
// Verification Types
// ============================================================================

/// A verification scenario with Given/When/Then steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    /// Name of the scenario.
    pub name: String,
    /// Capability this scenario belongs to (spec folder name, e.g., "ralph-loop").
    pub capability: String,
    /// Requirement ID this scenario belongs to (slugified requirement name).
    pub requirement_id: String,
    /// Preconditions (Given steps).
    pub given: Vec<String>,
    /// Action being taken (When step).
    pub when: String,
    /// Expected outcomes (Then steps).
    pub then: Vec<String>,
}

// ============================================================================
// Context Type
// ============================================================================

/// Verification commands for a project.
#[derive(Debug, Clone)]
pub struct VerifyCommands {
    /// Static check commands (e.g., cargo check, cargo clippy).
    pub checks: Vec<String>,
    /// Test command pattern.
    pub tests: String,
}

/// Complete context needed by an agent to work on a story.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Context {
    /// Current story information.
    pub story: Story,
    /// Proposal content.
    pub proposal: String,
    /// Design content.
    pub design: String,
    /// Scenarios relevant to this story.
    pub scenarios: Vec<Scenario>,
    /// Verification commands.
    pub verify: VerifyCommands,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_has_required_fields() {
        let task = Task {
            id: "1.1".to_string(),
            description: "Implement feature X".to_string(),
            done: false,
        };
        assert_eq!(task.id, "1.1");
        assert_eq!(task.description, "Implement feature X");
        assert!(!task.done);
    }

    #[test]
    fn task_completion_state() {
        let incomplete = Task {
            id: "t1".to_string(),
            description: "Do something".to_string(),
            done: false,
        };
        let complete = Task {
            id: "t2".to_string(),
            description: "Done thing".to_string(),
            done: true,
        };
        assert!(!incomplete.done);
        assert!(complete.done);
    }

    #[test]
    fn story_contains_tasks() {
        let story = Story {
            id: "1".to_string(),
            title: "User can login".to_string(),
            tasks: vec![
                Task {
                    id: "1.1".to_string(),
                    description: "Create login form".to_string(),
                    done: false,
                },
                Task {
                    id: "1.2".to_string(),
                    description: "Validate credentials".to_string(),
                    done: false,
                },
            ],
        };
        assert_eq!(story.id, "1");
        assert_eq!(story.title, "User can login");
        assert_eq!(story.tasks.len(), 2);
    }

    #[test]
    fn story_is_complete_when_all_tasks_done() {
        let story = Story {
            id: "1".to_string(),
            title: "Complete story".to_string(),
            tasks: vec![
                Task {
                    id: "1.1".to_string(),
                    description: "Task 1".to_string(),
                    done: true,
                },
                Task {
                    id: "1.2".to_string(),
                    description: "Task 2".to_string(),
                    done: true,
                },
            ],
        };
        assert!(story.is_complete());
    }

    #[test]
    fn story_is_not_complete_when_tasks_pending() {
        let story = Story {
            id: "1".to_string(),
            title: "Incomplete story".to_string(),
            tasks: vec![
                Task {
                    id: "1.1".to_string(),
                    description: "Task 1".to_string(),
                    done: true,
                },
                Task {
                    id: "1.2".to_string(),
                    description: "Task 2".to_string(),
                    done: false,
                },
            ],
        };
        assert!(!story.is_complete());
    }

    #[test]
    fn story_is_not_complete_when_empty() {
        let story = Story {
            id: "1".to_string(),
            title: "Empty story".to_string(),
            tasks: vec![],
        };
        assert!(!story.is_complete());
    }

    #[test]
    fn story_next_task_returns_first_incomplete() {
        let story = Story {
            id: "1".to_string(),
            title: "Story".to_string(),
            tasks: vec![
                Task {
                    id: "1.1".to_string(),
                    description: "Done task".to_string(),
                    done: true,
                },
                Task {
                    id: "1.2".to_string(),
                    description: "Pending task".to_string(),
                    done: false,
                },
                Task {
                    id: "1.3".to_string(),
                    description: "Another pending".to_string(),
                    done: false,
                },
            ],
        };
        let next = story.next_task().unwrap();
        assert_eq!(next.id, "1.2");
    }

    #[test]
    fn story_next_task_returns_none_when_all_complete() {
        let story = Story {
            id: "1".to_string(),
            title: "Story".to_string(),
            tasks: vec![Task {
                id: "1.1".to_string(),
                description: "Done task".to_string(),
                done: true,
            }],
        };
        assert!(story.next_task().is_none());
    }

    #[test]
    fn scenario_has_capability_and_requirement_id() {
        let scenario = Scenario {
            name: "Successful login".to_string(),
            capability: "auth".to_string(),
            requirement_id: "auth-flow".to_string(),
            given: vec!["User exists".to_string()],
            when: "User enters valid credentials".to_string(),
            then: vec!["User is logged in".to_string()],
        };
        assert_eq!(scenario.capability, "auth");
        assert_eq!(scenario.requirement_id, "auth-flow");
        assert_eq!(scenario.given.len(), 1);
        assert!(!scenario.when.is_empty());
        assert_eq!(scenario.then.len(), 1);
    }
}
