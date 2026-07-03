# Agent Guidelines

## Component Design

- Prefer existing Material 3 tokens from `crate::tokens` for color, shape, spacing, elevation, duration, and typography before adding local constants.
- Keep `src/widget.rs` as the shared constructor entrypoint. Move large or stateful components into focused files under `src/widget/` and re-export them from the entrypoint.
- Preserve existing public APIs unless the task explicitly calls for a breaking change.
- Match Material 3 component behavior, metrics, and interaction states before inventing new motion or layout parameters.
- Interactive overlays must keep hover, focus, and click paths stable between the anchor and surface, and must forward events to interactive children.
- Platform-specific behavior should be isolated behind the narrowest practical `cfg` or adapter so native and WASM behavior stay predictable.

## Verification

- Run `cargo fmt --check`, `cargo check`, `cargo check --examples`, and `cargo test` for shared widget changes.
- Add focused tests for geometry, animation state, hit testing, input, or overlay behavior when a fix depends on those details.
- Broaden verification when a change touches shared style tokens, navigation, overlays, input handling, or WASM behavior.

## Commit Discipline

- Every fix or new feature must be followed by a separate conventional commit.
- Use clear scopes when helpful, for example `fix: keep rich tooltips interactive` or `feat: add expressive loading indicator`.
- Keep unrelated changes in separate commits so each commit explains one behavior change.
- Sign commits when the repository or user workflow requires signed history.
