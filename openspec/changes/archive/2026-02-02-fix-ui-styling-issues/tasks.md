## 1. Result Screen Changed Files Coloring

- [x] 1.1 Modify `render_changed_files` in `src/ui/result_screen.rs` to split each line into status character and filename
- [x] 1.2 Apply color style only to status character Span, use default style for filename Span

## 2. Loop Screen Story Progress Ellipsis

- [x] 2.1 Remove ellipsis span generation before story window in `render_story_progress` in `src/ui/loop_screen.rs`
- [x] 2.2 Remove ellipsis span generation after story window in `render_story_progress`
