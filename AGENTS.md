# PROJECT KNOWLEDGE BASE

**Generated:** 2026-08-20T09:52:09+07:00
**Commit:** 976255c
**Branch:** main

## OVERVIEW

`tinyz` is a private, local-first image compressor. A SvelteKit 2/Svelte 5 browser UI
dispatches PNG, JPEG, and WebP work to Web Workers backed by a Rust WebAssembly crate;
there is no upload or server/API path.

## STRUCTURE

```text
tinyz/
├── src/routes/                 # Single-page shell, global CSS, application orchestration
├── src/lib/components/         # Handwritten feature UI plus scoped primitive wrappers
├── src/lib/workers/            # Worker protocol and WASM invocation
├── src/lib/wasm/               # Checked-in wasm-pack output; generated
├── wasm-compressor/            # Rust source of truth and inline unit tests
├── static/                     # Favicon, robots file, cursor asset
└── package.json                # Bun command surface
```

## WHERE TO LOOK

| Task                          | Location                               | Notes                                                              |
| ----------------------------- | -------------------------------------- | ------------------------------------------------------------------ |
| Application state and batches | `src/routes/+page.svelte`              | Sole route; owns worker pool, queue, selection, downloads, cleanup |
| Shared image contracts        | `src/lib/types.ts`                     | Output formats, result states, worker response shape               |
| Format/file/download helpers  | `src/lib/utils.ts`                     | MIME mapping, filtering, ZIP assembly, `cn`                        |
| Feature UI                    | `src/lib/components/*.svelte`          | Handwritten components; see scoped guidance                        |
| Primitive UI                  | `src/lib/components/ui/`               | shadcn-svelte/Bits UI adapters; see scoped guidance                |
| Worker execution              | `src/lib/workers/compressor.worker.ts` | Per-worker serial queue, transferable buffers                      |
| Compression implementation    | `wasm-compressor/src/lib.rs`           | Decode, EXIF orientation, quantization, encoding, Rust tests       |
| Generated browser bindings    | `src/lib/wasm/`                        | Rebuilt from the Rust crate with `bun run wasm:build`              |
| Build integration             | `vite.config.ts`                       | WASM plugin is configured for main and worker builds               |

## CODE MAP

Codegraph found 63 symbols across the central route/worker/WASM flow. Rust LSP is
available; the configured Svelte/TypeScript LSP is not installed, so use
`bun run check` for those diagnostics.

| Symbol                         | Type          | Location                         | Refs                    | Role                                                      |
| ------------------------------ | ------------- | -------------------------------- | ----------------------- | --------------------------------------------------------- |
| `handleWorkerMessage`          | function      | `src/routes/+page.svelte`        | worker callback         | Reconciles started/result/error messages by ID and format |
| `compress`                     | function      | `src/routes/+page.svelte`        | `startBatch`            | Transfers a file buffer to the next worker                |
| `startBatch`                   | function      | `src/routes/+page.svelte`        | choose/recompress       | Tracks one batch and schedules every item                 |
| `downloadZip`                  | function      | `src/lib/utils.ts`               | route                   | Fetches compressed object URLs and assembles a ZIP        |
| `useFullscreen`                | function      | `src/lib/hooks/useFullscreen.ts` | `ComparisonSlider`      | Standard/WebKit fullscreen compatibility                  |
| `compress_image_with_metadata` | WASM export   | `wasm-compressor/src/lib.rs`     | generated bridge/worker | Returns encoded bytes and Rust-side timing                |
| `read_exif_orientation`        | Rust function | `wasm-compressor/src/lib.rs`     | compressor/tests        | Reads input orientation before pixel conversion           |
| `cn`                           | function      | `src/lib/utils.ts`               | UI wrappers             | Combines `clsx` and `tailwind-merge`                      |

## CONVENTIONS

- Use Bun and the checked-in `bun.lock`; scripts are the authoritative frontend workflow.
- Svelte files run in Svelte 5 runes mode. Use typed `$props`, `$state`, `$derived`,
  `SvelteMap`, and `SvelteSet` rather than legacy reactivity.
- Prettier uses tabs, single quotes, no trailing commas, 100 columns, Svelte parsing,
  and Tailwind class sorting against `src/routes/layout.css`.
- TypeScript is strict and checks JavaScript. ESLint intentionally disables `no-undef`
  for TypeScript and ignores generated WASM plus Rust target output.
- Rust uses edition 2024. Release WASM favors size: `opt-level = "z"`, LTO, stripping,
  one codegen unit, and no `wasm-opt` pass.

## ANTI-PATTERNS (THIS PROJECT)

- Do not add an upload/server dependency to compression; processing is intentionally local.
- Do not remove the WASM plugin from `vite.config.ts` worker configuration.
- Do not enable ESLint `no-undef` for TypeScript.
- Do not create object URLs without revoking them on replacement, queue clear, and teardown.
- Do not describe the UI's “50 MB” copy as enforcement: filtering currently checks exact MIME
  types only and has no file-size guard.

## UNIQUE STYLES

- `+page.svelte` deliberately owns orchestration rather than a store/service layer.
- The page dispatches round-robin across at most four workers; each worker serializes its jobs.
- JPEG output flattens alpha onto white. PNG and WebP use palette quantization; WebP is encoded
  losslessly after quality-dependent quantization. EXIF orientation is normalized first.
- All files in a batch share the current format and quality; controls remain locked while busy.

## COMMANDS

```bash
bun install
bun run dev
bun run check
bun run lint
bun run build
bun run preview
bun run wasm:build
cargo test --manifest-path wasm-compressor/Cargo.toml --all-targets
```

## NOTES

- Frontend tests and CI workflows do not exist. `bun run check`, lint, production build,
  Rust unit tests, and browser use are the available verification surfaces.
- `bun run lint` executes Prettier checking before ESLint; a formatting failure stops the chain.
- `wasm-compressor/target`, `wasm-compressor/pkg`, `.svelte-kit`, and provider output
  directories are build artifacts.
