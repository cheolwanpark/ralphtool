## ADDED Requirements

### Requirement: ContextProvider trait

The spec abstraction SHALL provide a `ContextProvider` trait for retrieving unified work context.

#### Scenario: Get context for story

- **WHEN** `get_context(story_id)` is called
- **THEN** the adapter SHALL return a `WorkContext` containing: story info, tasks, proposal content, design content, scenarios, and verification commands

#### Scenario: Invalid story ID

- **WHEN** `get_context(story_id)` is called with non-existent story ID
- **THEN** the adapter SHALL return an error indicating story not found

### Requirement: WorkContext structure

The `WorkContext` type SHALL contain all information needed by an agent to work on a story.

#### Scenario: WorkContext fields

- **WHEN** a WorkContext is returned
- **THEN** it SHALL contain: story (Story), tasks (Vec<Task>), proposal (String), design (String), scenarios (Vec<Scenario>), verify (VerifyCommands)

### Requirement: WorkStatus structure

The spec abstraction SHALL provide work status information.

#### Scenario: Get status

- **WHEN** `get_status()` is called
- **THEN** the adapter SHALL return a `WorkStatus` containing: current story progress and overall change progress

#### Scenario: WorkStatus fields

- **WHEN** a WorkStatus is returned
- **THEN** it SHALL contain: story_id, story_tasks_done, story_tasks_total, change_stories_done, change_stories_total

### Requirement: VerifyCommands inference

The context provider SHALL infer verification commands from project type.

#### Scenario: Rust project

- **WHEN** `Cargo.toml` exists in project root
- **THEN** verify commands SHALL include `cargo check`, `cargo clippy`, and tests SHALL be `cargo test`

#### Scenario: Node.js project

- **WHEN** `package.json` exists in project root
- **THEN** verify commands SHALL include `npm run lint` and tests SHALL be `npm test`

#### Scenario: Unknown project type

- **WHEN** no recognized project file exists
- **THEN** verify commands SHALL be empty
