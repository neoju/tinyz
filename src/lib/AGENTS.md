# LIBRARY KNOWLEDGE BASE

## OVERVIEW

`src/lib` holds browser-domain contracts, helpers, feature UI, the worker boundary, and
generated WASM consumed by the route.

## WHERE TO LOOK

| Task                   | Location                       | Notes                                                        |
| ---------------------- | ------------------------------ | ------------------------------------------------------------ |
| Image/result state     | `types.ts`                     | Shared discriminants and browser compatibility types         |
| File/format operations | `utils.ts`                     | Accepted MIME set, extensions, downloads, ZIP, class merging |
| Fullscreen behavior    | `hooks/useFullscreen.ts`       | Only consumer is `ComparisonSlider.svelte`                   |
| Feature components     | `components/`                  | Scoped child guidance applies                                |
| Worker protocol        | `workers/compressor.worker.ts` | Main-thread request to generated WASM API                    |
| Generated package      | `wasm/`                        | wasm-bindgen JS, declarations, binary, package metadata      |

## CONVENTIONS

- Keep `OutputFormat`, MIME/extension mappings, worker messages, and Rust-accepted format
  strings synchronized as one cross-language contract.
- Use explicit domain types from `types.ts`; narrow `ImageResult` to `ReadyImageResult`
  before download operations.
- Keep browser-only APIs in route events, `onMount`, workers, or hooks; this directory is
  compiled through SvelteKit and may otherwise be evaluated outside the browser.
- Preserve transferable `ArrayBuffer` use on both sides of the worker boundary.
- A worker initializes WASM once, emits lifecycle messages, and serializes its own jobs;
  page-level parallelism comes from multiple workers.
- Import concrete modules with `$lib/...`; do not add exports to `index.ts` merely to shorten
  an import.

## GENERATED WASM

- `wasm/wasm_compressor.js`, declarations, and `.wasm` are outputs of `wasm-pack`.
- Make source changes in `../../wasm-compressor/src/lib.rs`, then run `bun run wasm:build`
  from the repository root.
- Keep the generated JS and adjacent `wasm_compressor_bg.wasm` together; initialization
  resolves the binary relative to the module URL.
- Generated output is excluded from ESLint, so validate it through the Rust tests,
  regeneration, frontend build, and live compression flow.

## ANTI-PATTERNS

- Do not accept a new input/output format in only one mapping or one language layer.
- Do not reuse an input buffer after it has been transferred to a worker.
- Do not drop `id` or `format` from worker result/error messages; the page filters on both.
- Do not parallelize jobs inside one worker without reassessing WASM instance safety and
  message ordering.
- Do not hand-patch generated WASM glue or declarations.
