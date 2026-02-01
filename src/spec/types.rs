//! Domain types for spec abstraction concepts.

// ============================================================================
// Task Hierarchy Types
// ============================================================================

/// A single actionable task within the Ralph workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// Unique identifier for the task.
    pub id: String,
    /// Description of what needs to be done.
    pub description: String,
    /// Whether the task has been completed.
    pub complete: bool,
}

/// A story containing related tasks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Story {
    /// Unique identifier for the story.
    pub id: String,
    /// Title of the story.
    pub title: String,
    /// Tasks that belong to this story.
    pub tasks: Vec<Task>,
}


// ============================================================================
// Story Types
// ============================================================================

/// Priority level for user stories.
///
/// Variants are ordered so that `High > Medium > Low`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Lowest priority - can be deferred.
    Low,
    /// Normal priority.
    Medium,
    /// Highest priority - must be done first.
    High,
}

/// A user story with acceptance criteria and priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserStory {
    /// Unique identifier for the story.
    pub id: String,
    /// Title of the story.
    pub title: String,
    /// Detailed description of the story.
    pub description: String,
    /// List of acceptance criteria that must be satisfied.
    pub acceptance_criteria: Vec<String>,
    /// Priority level of this story.
    pub priority: Priority,
    /// Whether the story has passed verification.
    pub passed: bool,
}

// ============================================================================
// Verification Types
// ============================================================================

/// A verification scenario with Given/When/Then steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    /// Name of the scenario.
    pub name: String,
    /// Preconditions (Given steps).
    pub given: Vec<String>,
    /// Action being taken (When step).
    pub when: String,
    /// Expected outcomes (Then steps).
    pub then: Vec<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Task hierarchy tests
    #[test]
    fn task_has_required_fields() {
        let task = Task {
            id: "task-1".to_string(),
            description: "Implement feature X".to_string(),
            complete: false,
        };
        assert_eq!(task.id, "task-1");
        assert_eq!(task.description, "Implement feature X");
        assert!(!task.complete);
    }

    #[test]
    fn task_completion_state() {
        let incomplete = Task {
            id: "t1".to_string(),
            description: "Do something".to_string(),
            complete: false,
        };
        let complete = Task {
            id: "t2".to_string(),
            description: "Done thing".to_string(),
            complete: true,
        };
        assert!(!incomplete.complete);
        assert!(complete.complete);
    }

    #[test]
    fn story_contains_tasks() {
        let story = Story {
            id: "story-1".to_string(),
            title: "User can login".to_string(),
            tasks: vec![
                Task {
                    id: "t1".to_string(),
                    description: "Create login form".to_string(),
                    complete: false,
                },
                Task {
                    id: "t2".to_string(),
                    description: "Validate credentials".to_string(),
                    complete: false,
                },
            ],
        };
        assert_eq!(story.id, "story-1");
        assert_eq!(story.title, "User can login");
        assert_eq!(story.tasks.len(), 2);
    }

    #[test]
    fn story_can_have_no_tasks() {
        let story = Story {
            id: "empty".to_string(),
            title: "Empty story".to_string(),
            tasks: vec![],
        };
        assert!(story.tasks.is_empty());
    }

    // Story types tests
    #[test]
    fn priority_ordering() {
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::High > Priority::Low);
    }

    #[test]
    fn user_story_has_all_fields() {
        let story = UserStory {
            id: "us-1".to_string(),
            title: "User registration".to_string(),
            description: "As a user, I want to register".to_string(),
            acceptance_criteria: vec![
                "Email is validated".to_string(),
                "Password meets requirements".to_string(),
            ],
            priority: Priority::High,
            passed: false,
        };
        assert_eq!(story.id, "us-1");
        assert_eq!(story.title, "User registration");
        assert!(!story.description.is_empty());
        assert_eq!(story.acceptance_criteria.len(), 2);
        assert_eq!(story.priority, Priority::High);
        assert!(!story.passed);
    }

    // Verification types tests
    #[test]
    fn scenario_has_given_when_then() {
        let scenario = Scenario {
            name: "Successful login".to_string(),
            given: vec![
                "User exists".to_string(),
                "User is on login page".to_string(),
            ],
            when: "User enters valid credentials".to_string(),
            then: vec![
                "User is redirected to dashboard".to_string(),
                "Session is created".to_string(),
            ],
        };
        assert_eq!(scenario.name, "Successful login");
        assert_eq!(scenario.given.len(), 2);
        assert!(!scenario.when.is_empty());
        assert_eq!(scenario.then.len(), 2);
    }
}
