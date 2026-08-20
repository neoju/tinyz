import type { WebkitDocument, WebkitElement } from '$lib/types';

export function useFullscreen() {
	const doc = document as WebkitDocument;

	function isFullscreen(): boolean {
		return !!(document.fullscreenElement || doc.webkitFullscreenElement);
	}

	async function exit(): Promise<boolean> {
		try {
			if (document.exitFullscreen) {
				await document.exitFullscreen();
				return true;
			}
			if (doc.webkitExitFullscreen) {
				await doc.webkitExitFullscreen.call(document);
				return true;
			}
		} catch {
			return false;
		}

		return false;
	}

	async function request(element: HTMLElement): Promise<boolean> {
		const el = element as WebkitElement;
		try {
			if (element.requestFullscreen) {
				await element.requestFullscreen();
				return true;
			}
			if (el.webkitRequestFullscreen) {
				await el.webkitRequestFullscreen.call(element);
				return true;
			}
		} catch {
			return false;
		}

		return false;
	}

	async function toggle(element: HTMLElement): Promise<boolean> {
		if (isFullscreen()) {
			return exit();
		}

		return request(element);
	}

	return { isFullscreen, exit, request, toggle };
}
