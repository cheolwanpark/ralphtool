## Why

The current TUI layout uses the full terminal width, making text hard to read on wide terminals and lacking visual hierarchy. The header section is cramped (5 lines fixed), mixing logo, title, and keybindings horizontally. We need a centered, max-width constrained layout with a dedicated header section (20% height) featuring an ASCII logo, screen title, description, and keybindings.

## What Changes

- Add centered layout container with max_width=100, centered on both X and Y axes
- Replace current horizontal header with vertical header section occupying 20% of height:
  - ASCII art logo "RALPH" (Slim Block style, 2 lines)
  - Screen title with icon (e.g., "◆ Change Selection")
  - Screen description
  - Keybindings help line
- Implement responsive behavior: hide logo when terminal height < 24 lines
- Apply new header layout consistently across all 4 screens (Selection, Preview, Loop, Result)

## Capabilities

### New Capabilities
- `tui-centered-layout`: Centered container with max-width constraint and XY centering
- `tui-header-section`: Unified header section with logo, title, description, and keybindings

### Modified Capabilities
<!-- No existing specs are being modified - this is new UI capability -->

## Impact

- `src/ui/mod.rs`: Add centered_rect helper, new header component, ASCII logo constant
- `src/ui/selection.rs`: Use new header section
- `src/ui/preview.rs`: Use new header section
- `src/ui/loop_screen.rs`: Use new header section
- `src/ui/result_screen.rs`: Use new header section
- All screens will have consistent visual appearance
- No breaking changes to functionality
