## MODIFIED Requirements

### Requirement: Scenario parsing from specs

The adapter SHALL parse spec files to extract verification scenarios with capability tracking.

#### Scenario: Parse scenario blocks

- **WHEN** parsing a spec.md file in `specs/<capability>/`
- **THEN** blocks starting with `#### Scenario: <name>` SHALL be parsed as Scenario entries
- **AND** the `capability` field SHALL be set to the spec folder name (e.g., `"ralph-loop"`)

#### Scenario: Extract Given/When/Then steps

- **WHEN** parsing a scenario block
- **THEN** lines containing `GIVEN` SHALL populate the `given` field
- **AND** lines containing `WHEN` SHALL populate the `when` field
- **AND** lines containing `THEN` or `AND` after THEN SHALL populate the `then` field

#### Scenario: Derive requirement ID from requirement name

- **WHEN** parsing a scenario under a requirement heading
- **THEN** the `requirement_id` field SHALL be set to the slugified requirement name
- **AND** the slugification SHALL convert to lowercase and replace spaces with hyphens

### Requirement: ScenarioSource trait implementation

The adapter SHALL implement the `ScenarioSource` trait.

#### Scenario: List scenarios returns all

- **WHEN** `list_scenarios()` is called
- **THEN** the adapter SHALL return all Scenario entries extracted from specs
- **AND** each scenario SHALL include `capability` and `requirement_id` fields

#### Scenario: Scenarios for capability filters correctly

- **WHEN** `scenarios_for_capability(capability)` is called
- **THEN** the adapter SHALL return only scenarios where `capability` matches the specified value

## REMOVED Requirements

### Requirement: Story-based scenario filtering

**Reason**: The `scenarios_for(story_id)` method attempted to match scenarios to task stories using incompatible ID namespaces (numeric story IDs vs slugified requirement names). This never worked correctly.

**Migration**: Use `scenarios_for_capability(capability)` instead to filter scenarios by their source spec file.
