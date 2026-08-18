/* tslint:disable */
/* eslint-disable */

export class CompressionResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly bytes: Uint8Array;
    readonly compression_ms: number;
}

/**
 * Decode, quantize, and encode an image as png, jpeg, or webp.
 */
export function compress_image(input_bytes: Uint8Array, quality: number, format: string): Uint8Array;

/**
 * Compress an image and return the encoded bytes with Rust-side timing metadata.
 */
export function compress_image_with_metadata(input_bytes: Uint8Array, quality: number, format: string): CompressionResult;

/**
 * Keeps the original public API for PNG callers.
 */
export function compress_png(input_bytes: Uint8Array, quality: number): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_compressionresult_free: (a: number, b: number) => void;
    readonly compress_image: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly compress_image_with_metadata: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly compress_png: (a: number, b: number, c: number, d: number) => void;
    readonly compressionresult_bytes: (a: number, b: number) => void;
    readonly compressionresult_compression_ms: (a: number) => number;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
