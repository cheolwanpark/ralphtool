## 1. Project Setup

- [x] 1.1 Create Cargo.toml with project metadata (name: ralphtool, edition: 2021)
- [x] 1.2 Add dependencies: ratatui, crossterm, anyhow
- [x] 1.3 Create src/ directory structure

## 2. Core Modules

- [x] 2.1 Create src/main.rs with module declarations and entry point
- [x] 2.2 Create src/app.rs with App struct holding application state
- [x] 2.3 Create src/event.rs with event polling and handling
- [x] 2.4 Create src/ui.rs with render function

## 3. Terminal Management

- [x] 3.1 Implement terminal initialization (raw mode, alternate screen)
- [x] 3.2 Implement terminal restoration function
- [x] 3.3 Install panic hook for terminal restoration on panic

## 4. Main Loop

- [x] 4.1 Implement main event loop with poll timeout
- [x] 4.2 Handle 'q' key press to exit
- [x] 4.3 Call render function each loop iteration

## 5. UI Rendering

- [x] 5.1 Implement basic frame rendering with Terminal::draw
- [x] 5.2 Display welcome message/title
- [x] 5.3 Display quit instructions ("Press q to quit")

## 6. Verification

- [x] 6.1 Verify project builds with `cargo build`
- [x] 6.2 Verify application runs and displays UI
- [x] 6.3 Verify clean exit on 'q' press
