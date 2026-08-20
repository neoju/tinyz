# FEATURE COMPONENT KNOWLEDGE BASE

## OVERVIEW

This directory contains eight handwritten application components; `ui/` is a separately
scoped primitive adapter layer.

## WHERE TO LOOK

| Task                      | Location                                        | Notes                                        |
| ------------------------- | ----------------------------------------------- | -------------------------------------------- |
| Zoom/pan/split/fullscreen | `ComparisonSlider.svelte`                       | Largest interaction state machine            |
| Batch controls            | `Controls.svelte`                               | Quality/format props and recompress callback |
| Drag/drop input           | `DropZone.svelte`                               | Picker MIME accept list and busy state       |
| Selected preview          | `Preview.svelte`                                | Composes `ComparisonSlider`                  |
| Queue/results/actions     | `ResultList.svelte`                             | Status UI, download, clear confirmation      |
| Shell content             | `Header.svelte`, `Hero.svelte`, `Footer.svelte` | Branding and static page regions             |

## CONVENTIONS

- Define component contracts with typed `$props<{ ... }>()`; callbacks have explicit
  argument and return types.
- Keep queue ownership, worker scheduling, downloads, and object URL lifecycle in
  `src/routes/+page.svelte`; components receive state and callbacks.
- Keep interaction-local state with the owning component. `ComparisonSlider` owns pointer,
  keyboard, zoom, pan, split position, and fullscreen behavior.
- Compose generic controls from `ui/`; keep application labels, statuses, and workflows in
  this feature layer.
- Use utilities and domain types through `$lib/utils` and `$lib/types`.

## ANTI-PATTERNS

- Do not move domain orchestration into a visual component to avoid passing a typed callback.
- Do not put product-specific copy or image-result logic into `ui/` primitives.
- Do not bypass busy/disabled state while a batch is queued or compressing.
- Do not add pointer-only interactions; `ComparisonSlider` also supports keyboard use.
