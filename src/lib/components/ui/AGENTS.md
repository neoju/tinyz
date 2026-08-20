# UI PRIMITIVE KNOWLEDGE BASE

## OVERVIEW

This is the shadcn-svelte/Bits UI adapter layer: 28 Svelte wrappers and five barrel indexes
across alert dialog, button, scroll area, select, and separator families.

## STRUCTURE

```text
ui/
├── alert-dialog/   # Compound AlertDialog wrappers and aliases
├── button/         # Button/anchor wrapper plus tailwind-variants API
├── scroll-area/    # Root and scrollbar wrappers
├── select/         # Compound Select wrappers and aliases
├── separator/      # Local Separator wrapper
└── slider/         # Empty placeholder; not an active primitive family
```

## WHERE TO LOOK

| Task                | Location               | Notes                                                  |
| ------------------- | ---------------------- | ------------------------------------------------------ |
| Public family APIs  | `*/index.ts`           | Canonical aliases, prop types, component exports       |
| Button variants     | `button/button.svelte` | Variant/size definitions and anchor support            |
| Dialog composition  | `alert-dialog/`        | Bits UI wrappers with local layout classes             |
| Select composition  | `select/`              | Root, portal, trigger, content, items, scroll controls |
| Shared prop helpers | `../../utils.ts`       | `cn`, element-ref and child prop utility types         |

## CONVENTIONS

- Preserve each compound family's barrel exports; feature code imports named aliases from
  the directory, not individual implementation files.
- Wrap the matching `bits-ui` primitive, forward rest props, expose `class` as `className`,
  and merge local classes through `cn`.
- Keep wrappers generic and compatible with the upstream primitive's props, bindings,
  snippets, and element refs.
- Tailwind class order is formatter-controlled. Run the root format command after edits.
- Add feature behavior one level up in `src/lib/components`, not in a reusable primitive.

## ANTI-PATTERNS

- Do not edit one member of a compound family without checking its barrel and sibling aliases.
- Do not replace Bits UI keyboard/focus/portal behavior with ad hoc DOM handling.
- Do not hard-code application copy, image formats, queue states, or compression behavior here.
- Do not introduce a new primitive family without populating its barrel and confirming the
  shadcn alias/configuration in `components.json`.
- Do not treat the empty `slider/` directory as an implemented component.
