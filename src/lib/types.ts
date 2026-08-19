export type OutputFormat = 'png' | 'jpeg' | 'webp';

export type WorkerResponseMessage = {
	type: string;
	id?: number;
	bytes?: ArrayBuffer;
	compressionMs?: number;
	message?: string;
	format?: OutputFormat;
};

export type ImageResult = {
	id: number;
	name: string;
	originalBytes: number;
	originalUrl: string;
	compressedBytes?: number;
	compressionMs?: number;
	compressedUrl?: string;
	outputName?: string;
	status: 'queued' | 'compressing' | 'done' | 'error';
	error?: string;
};

export type ReadyImageResult = ImageResult & {
	compressedUrl: string;
	outputName: string;
};
