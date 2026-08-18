# tinyz

Private, local-first image compression in your browser. Drop in multiple PNG,
JPEG, or WebP images, choose the quality and output format, and download the
compressed files individually or as a ZIP archive.

Images are processed on your device in a Web Worker using Rust and WebAssembly;
they are not uploaded to a server.

## Requirements

- [Bun](https://bun.sh/)
- Rust and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) if you need to rebuild the compressor

## Development

Install dependencies and start the development server:

```sh
bun install
bun run dev
```

Open the local URL shown by Vite. To open it automatically, use:

```sh
bun run dev -- --open
```

## Build

Create and preview a production build:

```sh
bun run build
bun run preview
```

Run checks and linting with:

```sh
bun run check
bun run lint
```

## Rebuild WebAssembly

The generated WebAssembly files are stored in `src/lib/wasm`. Rebuild them
after changing the Rust compressor:

```sh
bun run wasm:build
```
