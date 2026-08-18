export type OutputFormat = 'png' | 'jpeg' | 'webp';

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
