# rmcp-release-demo — project conventions

Cross-skill project facts are declared in `capabilities.toml` at the repository root.

## Invariants

- Do not split a change into vertical slices (tickets or TDD slices) unless it needs them.
- Suppress a lint only with `#[expect(<lint>, reason = "...")]`, never with `#[allow(...)]`.

## Agent skills

### Issue tracker

Tickets are GitHub Issues on `aloong-planet/rmcp-release-demo`. See `docs/agents/issue-tracker.md`.

### Documentation layout

Single context: one glossary at `docs/CONTEXT.md`, decisions in `docs/adr/`. See `docs/agents/docs-layout.md`.
