import type { WebkitDocument, WebkitElement } from '$lib/types';

export function useFullscreen() {
	const doc = document as WebkitDocument;

	function isFullscreen(): boolean {
		return !!(document.fullscreenElement || doc.webkitFullscreenElement);
	}

	async function exit(): Promise<void> {
		if (document.exitFullscreen) {
			await document.exitFullscreen();
		} else if (doc.webkitExitFullscreen) {
			await doc.webkitExitFullscreen.call(document);
		}
	}

	async function request(element: HTMLElement): Promise<void> {
		const el = element as WebkitElement;
		if (element.requestFullscreen) {
			await element.requestFullscreen();
		} else if (el.webkitRequestFullscreen) {
			await el.webkitRequestFullscreen.call(element);
		}
	}

	async function toggle(element: HTMLElement): Promise<void> {
		if (isFullscreen()) {
			await exit();
		} else {
			await request(element);
		}
	}

	return { isFullscreen, exit, request, toggle };
}
