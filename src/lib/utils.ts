import { zipSync } from 'fflate';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { OutputFormat, ReadyImageResult } from '$lib/types';

export type WithElementRef<T> = T & { ref?: HTMLElement | null };
export type WithoutChild<T> = Omit<T, 'child'>;
export type WithoutChildrenOrChild<T> = Omit<T, 'children' | 'child'>;

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

const FORMATS = {
	png: { ext: 'png', label: 'PNG', mime: 'image/png' },
	jpeg: { ext: 'jpg', label: 'JPEG', mime: 'image/jpeg' },
	webp: { ext: 'webp', label: 'WebP', mime: 'image/webp' }
} as const;

const EXT_TO_FORMAT = Object.fromEntries(
	Object.entries(FORMATS).map(([fmt, { ext }]) => [ext, fmt as OutputFormat])
) as Record<string, OutputFormat>;

const MIME_TO_FORMAT = Object.fromEntries(
	Object.entries(FORMATS).map(([fmt, { mime }]) => [mime, fmt as OutputFormat])
) as Record<string, OutputFormat>;

export const ACCEPTED_IMAGE_TYPES: string[] = Object.values(FORMATS).map((f) => f.mime);

export function mimeType(format: OutputFormat) {
	return FORMATS[format].mime;
}

export function formatToExtension(format: OutputFormat) {
	return FORMATS[format].ext;
}

export function formatLabel(format: OutputFormat) {
	return FORMATS[format].label;
}

export function extensionToFormat(ext: string): OutputFormat | undefined {
	return EXT_TO_FORMAT[ext.toLowerCase()];
}

export function mimeToFormat(mime: string): OutputFormat | undefined {
	return MIME_TO_FORMAT[mime.toLowerCase()];
}

export function filterAcceptedImages(files: ArrayLike<File>) {
	return Array.from(files).filter((file) => ACCEPTED_IMAGE_TYPES.includes(file.type));
}

export function formatBytes(bytes: number) {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}

export function reductionPercent(originalBytes: number, compressedBytes: number) {
	return Math.max(0, Math.round((1 - compressedBytes / originalBytes) * 100));
}

export function formatKilobytes(bytes: number) {
	return `${(bytes / 1024).toFixed(1)} KB`;
}

export function formatMilliseconds(milliseconds: number) {
	return milliseconds < 1000 ? `${milliseconds} ms` : `${(milliseconds / 1000).toFixed(2)} s`;
}

export function triggerDownload(url: string, filename: string) {
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	link.click();
}

export async function downloadZip(items: ReadyImageResult[]) {
	if (!items.length) return;

	const extension = items[0].outputName.match(/\.([^.]+)$/)?.[1] ?? 'zip';
	const archiveName = `tinyz-${extension}.zip`;
	const entries: Record<string, Uint8Array> = {};

	await Promise.all(
		items.map(async (item, index) => {
			const response = await fetch(item.compressedUrl);
			if (!response.ok) throw new Error(`Could not read ${item.outputName}`);

			let filename = item.outputName;
			if (entries[filename]) filename = `${index + 1}-${filename}`;
			entries[filename] = new Uint8Array(await response.arrayBuffer());
		})
	);

	const url = URL.createObjectURL(new Blob([zipSync(entries)], { type: 'application/zip' }));
	triggerDownload(url, archiveName);
	setTimeout(() => URL.revokeObjectURL(url), 0);
}
