import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { OutputFormat } from '$lib/types';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function mimeType(format: OutputFormat) {
	return format === 'jpeg' ? 'image/jpeg' : `image/${format}`;
}

export function formatBytes(bytes: number) {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}

export function reductionPercent(
	originalBytes: number,
	compressedBytes: number
) {
	return Math.max(0, Math.round((1 - compressedBytes / originalBytes) * 100));
}

export function formatKilobytes(bytes: number) {
	return `${(bytes / 1024).toFixed(1)} KB`;
}

export function formatMilliseconds(milliseconds: number) {
	return milliseconds < 1000
		? `${milliseconds} ms`
		: `${(milliseconds / 1000).toFixed(2)} s`;
}

export type WithElementRef<T> = T & { ref?: HTMLElement | null };
export type WithoutChild<T> = Omit<T, 'child'>;
export type WithoutChildrenOrChild<T> = Omit<T, 'children' | 'child'>;
