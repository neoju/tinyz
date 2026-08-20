# tinyz Design System

## 1. Atmosphere & Identity

tinyz feels like a calm utility bench with a little warmth in the surface treatment. The interface is quiet, tactile, and direct: soft paper backgrounds, muted olive accents, and compact controls that get out of the way until the user needs them. The signature is the comparison surface itself, framed like a precise work surface with lightweight guidance layered on top instead of heavy chrome.

## 2. Color

### Palette

| Role | Token | Light | Dark | Usage |
|------|-------|-------|------|-------|
| Surface/background | --color-background | #f4f1eb | #f4f1eb | App shell background |
| Surface/popover | --color-popover | #fbfaf7 | #fbfaf7 | Overlays, tooltip panels |
| Surface/muted | --color-muted | #e8e5de | #e8e5de | Soft panels, subtle surfaces |
| Text/primary | --color-foreground | #1c1d1b | #1c1d1b | Main copy |
| Text/secondary | --color-muted-foreground | #7b7e76 | #7b7e76 | Supporting labels |
| Border/default | --color-border | #d4d3ca | #d4d3ca | Rules, frames |
| Border/input | --color-input | #b9bbb1 | #b9bbb1 | Controls, range tracks |
| Accent/primary | --color-primary | #798d2e | #798d2e | Primary actions, progress, focus |
| Accent/foreground | --color-primary-foreground | #f4f1eb | #f4f1eb | Text on primary surfaces |
| Accent/secondary | --color-secondary | #e8e5de | #e8e5de | Secondary fills |
| Accent/secondary text | --color-secondary-foreground | #252621 | #252621 | Text on secondary fills |
| Status/destructive | --color-destructive | #a34d3e | #a34d3e | Errors and destructive actions |
| Focus ring | --color-ring | #a8b77a | #a8b77a | Keyboard focus and active hints |

### Rules

- Keep the palette restrained and derived from the existing shell tokens in `src/routes/layout.css`.
- Use `--color-popover` for guidance surfaces such as tooltips and helper callouts.
- Accent green is reserved for interactive affordances, active state, and progress.
- Do not introduce new colors without updating this table first.

## 3. Typography

### Scale

| Level | Size | Weight | Line Height | Tracking | Usage |
|-------|------|--------|-------------|----------|-------|
| Display | 48px / 3rem | 700 | 1.1 | -0.02em | Hero headline |
| H1 | 36px / 2.25rem | 700 | 1.2 | -0.015em | Major section title |
| H2 | 28px / 1.75rem | 600 | 1.3 | -0.01em | Subsection title |
| H3 | 22px / 1.375rem | 600 | 1.4 | 0 | Card title |
| Body | 16px / 1rem | 400 | 1.6 | 0 | Default copy |
| Body/sm | 14px / 0.875rem | 400 | 1.5 | 0 | Secondary copy |
| Caption | 12px / 0.75rem | 500 | 1.4 | 0.02em | Labels, metadata |
| Overline | 10px / 0.625rem | 600 | 1.3 | 0.08em | Uppercase section labels |

### Font Stack

- Primary: system UI sans stack
- Mono: inherit from system defaults when needed for code-like labels

### Rules

- Keep label copy compact and uppercase where the current UI already does that.
- Use heavier weight only for emphasis and numeric readouts.
- Body text stays readable on small screens; no tiny helper copy below 12px.

## 4. Spacing & Layout

### Base Unit

All spacing derives from a 4px grid.

| Token | Value | Usage |
|-------|-------|-------|
| --space-1 | 4px | Tight icon-to-label gaps |
| --space-2 | 8px | Compact control gaps |
| --space-3 | 12px | Inline group spacing |
| --space-4 | 16px | Standard padding |
| --space-5 | 20px | Comfortable section rhythm |
| --space-6 | 24px | Larger control clusters |
| --space-8 | 32px | Section separation |
| --space-10 | 40px | Page rhythm |

### Grid

- Content stays centered within the app shell and compresses down to a single readable column on small screens.
- Comparison content owns the wide surface; controls and metadata remain compact and aligned to the right when space allows.
- The comparison helper lives below the section header as a short dismissible banner; when closed, a compact question-mark button appears beside the header to reopen it. The dismissed state persists in local storage.
- The output queue helper mirrors that pattern: a short dismissible banner below the section header, with a small reopen control beside the heading when collapsed. The dismissed state persists in local storage.

### Rules

- Use tokenized spacing values for authored gaps and margins.
- Keep browser mechanics raw when the layout needs intrinsic sizing or viewport behavior.

## 5. Components

### Button
- **Structure**: primary/secondary control button, icon or text+icon
- **Variants**: default, outline, ghost, destructive, link
- **Spacing**: 8px cluster gaps, 32px icon buttons
- **States**: default, hover, active, focus, disabled
- **Accessibility**: native button semantics, visible focus ring
- **Motion**: subtle press translation and state transitions
- **Layout**: inline cluster

### Select
- **Structure**: trigger + portal content + items
- **Variants**: compact form control
- **Spacing**: 8-12px internal gaps
- **States**: default, open, hover, focus, disabled
- **Accessibility**: keyboard navigation, portal-based menu, trigger labeling
- **Motion**: lightweight open/close transition
- **Layout**: form control

### Comparison Slider
- **Structure**: layered image comparison frame, draggable split handle, zoom controls, fullscreen action, inline metadata
- **Variants**: normal, zoomed, fullscreen fallback
- **Spacing**: 12px internal control gaps, 15px section gaps, 35px top margin
- **States**: default, dragging, split-adjusting, zoomed, fullscreen fallback, disabled controls while busy upstream
- **Accessibility**: ARIA slider handle, descriptive labels, keyboard reach for controls that exist on desktop
- **Motion**: pointer-following drag only
- **Layout**: comparison surface with anchored controls

### Helper Banner
- **Structure**: short dismissible note below the section header with body copy and an X button, plus a question-mark reopen control beside the header when collapsed
- **Variants**: expanded, dismissed
- **Spacing**: 12px gap below the section header, 12px padding inside the banner, compact 8px gap for the header reopen control
- **States**: visible, dismissed, hover, focus
- **Accessibility**: readable inline text, reachable dismiss/reopen buttons, persistence of dismissed state across visits
- **Motion**: no motion required beyond normal button feedback
- **Layout**: full-width inline callout with inline header action

### Output Queue Helper
- **Structure**: short dismissible note below the queue header with body copy and an X button, plus a compact reopen control beside the heading when collapsed
- **Variants**: expanded, dismissed
- **Spacing**: 12px gap below the section header, 12px padding inside the banner, compact 8px gap for the header reopen control
- **States**: visible, dismissed, hover, focus
- **Accessibility**: readable inline text, reachable dismiss/reopen buttons, persistence of dismissed state across visits
- **Motion**: no motion required beyond normal button feedback
- **Layout**: full-width inline callout with inline header action

## 6. Motion & Interaction

### Timing

| Type | Duration | Easing | Usage |
|------|----------|--------|-------|
| Micro | 100-150ms | ease-out | Button press, helper dismiss |
| Standard | 200-300ms | ease-in-out | Panel and overlay transitions |

### Rules

- Motion serves guidance and state, not decoration.
- Respect `prefers-reduced-motion` by keeping guidance visible without relying on animation.

## 7. Depth & Surface

### Strategy

Tonal shift with light borders.

### Rules

- App surfaces stay soft and low-contrast.
- Popovers and tooltips use the popover surface token and a border to separate them from the background.
- Avoid heavy shadows; the product reads more clearly as paper and panel than as chrome.

## 8. Accessibility Constraints & Accepted Debt

### Constraints

- WCAG 2.2 AA target.
- Visible focus on every interactive element.
- Helper guidance must remain readable, dismissible, reopenable from the header, and persist its closed state without blocking the comparison surface.
- Reduced motion must remain usable without hover-only dependence.

### Accepted Debt

| Item | Location | Why accepted | Owner / Exit |
|------|----------|--------------|--------------|
| Desktop keyboard-only slider nudge handlers removed | `src/lib/components/ComparisonSlider.svelte` | User asked for the touch-first fullscreen fix and then requested removal; keyboard nudge is intentionally absent for this surface | Revisit only if the comparison slider gets an explicit keyboard interaction requirement |
