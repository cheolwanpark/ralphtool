## Why

This project needs to be initialized as a Rust TUI application using ratatui. Setting up the proper project structure, dependencies, and boilerplate code now will establish a solid foundation for building the terminal user interface.

## What Changes

- Initialize a new Rust project with Cargo
- Add ratatui and crossterm dependencies for TUI rendering and terminal handling
- Create the basic application structure with main loop, event handling, and UI rendering
- Set up error handling with anyhow for cleaner error propagation

## Capabilities

### New Capabilities

- `app-scaffold`: Basic application structure including Cargo.toml, main entry point, and module organization
- `tui-core`: Terminal initialization, main loop, event handling, and graceful shutdown
- `ui-rendering`: Basic UI rendering setup with ratatui widgets and layout system

### Modified Capabilities

None - this is a greenfield project.

## Impact

- Creates new `Cargo.toml` with project metadata and dependencies
- Creates `src/` directory with application code
- No existing code affected (new project initialization)
