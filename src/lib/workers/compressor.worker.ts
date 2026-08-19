import init, { compress_image_with_metadata } from '$lib/wasm/wasm_compressor.js';
import type { OutputFormat } from '$lib/types';

type CompressMessage = {
	type: 'compress';
	id: number;
	input: ArrayBuffer | File;
	quality: number;
	format: OutputFormat;
};

type WorkerMessage =
	| { type: 'ready' }
	| { type: 'started'; format: OutputFormat }
	| {
			type: 'result';
			bytes: ArrayBuffer;
			compressionMs: number;
			format: OutputFormat;
	  }
	| { type: 'error'; message: string; format: OutputFormat };

type WorkerScope = {
	postMessage: (message: WorkerMessage & { id?: number }, transfer?: Transferable[]) => void;
	onmessage: ((event: MessageEvent<CompressMessage>) => void) | null;
};

const wasmReady = init();
const workerScope = self as unknown as WorkerScope;
workerScope.postMessage({ type: 'ready' } satisfies WorkerMessage);

let queue = Promise.resolve();
workerScope.onmessage = (event: MessageEvent<CompressMessage>) => {
	if (event.data.type !== 'compress') return;

	queue = queue.then(async () => {
		try {
			workerScope.postMessage({
				type: 'started',
				id: event.data.id,
				format: event.data.format
			});

			await wasmReady;
			const input =
				event.data.input instanceof ArrayBuffer
					? event.data.input
					: await event.data.input.arrayBuffer();

			const result = compress_image_with_metadata(
				new Uint8Array(input),
				event.data.quality,
				event.data.format
			);

			const bytes = result.bytes.slice().buffer;

			workerScope.postMessage(
				{
					type: 'result',
					id: event.data.id,
					bytes,
					compressionMs: result.compression_ms,
					format: event.data.format
				},
				[bytes]
			);
		} catch (error) {
			workerScope.postMessage({
				type: 'error',
				id: event.data.id,
				format: event.data.format,
				message: error instanceof Error ? error.message : 'Compression failed.'
			});
		}
	});
};
