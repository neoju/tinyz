<script lang="ts">
	import { Maximize2, ZoomIn, ZoomOut } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button/index.js';

	let { originalUrl, compressedUrl } = $props<{ originalUrl: string; compressedUrl: string }>();
	let position = $state(50);
	let zoom = $state(1);
	let panX = $state(0);
	let panY = $state(0);
	let gesture = $state<'split' | 'pan' | null>(null);
	let lastPointerX = 0;
	let lastPointerY = 0;
	let pendingPointerX = 0;
	let pendingPointerY = 0;
	let animationFrame: number | undefined;
	let slider: HTMLDivElement;

	async function toggleFullscreen() {
		if (document.fullscreenElement) await document.exitFullscreen();
		else await slider.requestFullscreen();
	}

	function changeZoom(amount: number) {
		zoom = Math.min(10, Math.max(1, Number((zoom + amount).toFixed(2))));
		slider?.style.setProperty('--zoom', String(zoom));
		if (zoom === 1) {
			panX = 0;
			panY = 0;
			slider?.style.setProperty('--pan-x', '0px');
			slider?.style.setProperty('--pan-y', '0px');
		}
	}

	function updateSplit(clientX: number) {
		const bounds = slider.getBoundingClientRect();
		position = Math.min(100, Math.max(0, ((clientX - bounds.left) / bounds.width) * 100));
	}

	function startSplit(event: PointerEvent) {
		event.preventDefault();
		event.stopPropagation();
		gesture = 'split';
		slider.setPointerCapture(event.pointerId);
		updateSplit(event.clientX);
	}

	function startPan(event: PointerEvent) {
		if (zoom === 1 || event.button !== 0) return;
		event.preventDefault();
		gesture = 'pan';
		slider.setPointerCapture(event.pointerId);
		lastPointerX = event.clientX;
		lastPointerY = event.clientY;
	}

	function movePointer(event: PointerEvent) {
		if (!gesture) return;
		pendingPointerX = event.clientX;
		pendingPointerY = event.clientY;
		if (animationFrame === undefined) animationFrame = requestAnimationFrame(applyPointerMove);
	}

	function applyPointerMove() {
		animationFrame = undefined;
		if (!gesture) return;
		if (gesture === 'split') {
			updateSplit(pendingPointerX);
			return;
		}
		const maxX = (slider.clientWidth * (zoom - 1)) / 2;
		const maxY = (slider.clientHeight * (zoom - 1)) / 2;
		panX = Math.min(maxX, Math.max(-maxX, panX + pendingPointerX - lastPointerX));
		panY = Math.min(maxY, Math.max(-maxY, panY + pendingPointerY - lastPointerY));
		lastPointerX = pendingPointerX;
		lastPointerY = pendingPointerY;
	}

	function endPointer() {
		gesture = null;
		if (animationFrame !== undefined) {
			cancelAnimationFrame(animationFrame);
			animationFrame = undefined;
		}
	}
</script>

<svelte:window onpointermove={movePointer} onpointerup={endPointer} />

<div
	bind:this={slider}
	class:grabbing={gesture === 'pan'}
	class="slider"
	role="application"
	aria-label="Image comparison preview. Drag the image when zoomed to pan."
	style={`--split: ${position}%; --zoom: ${zoom}; --pan-x: ${panX}px; --pan-y: ${panY}px`}
	onpointerdown={startPan}
>
	<img class="base" src={compressedUrl} alt="Compressed preview" draggable="false" />
	<div class="clipped"><img src={originalUrl} alt="Original preview" draggable="false" /></div>
	<button
		class="handle"
		type="button"
		role="slider"
		aria-label="Comparison position"
		aria-valuemin="0"
		aria-valuemax="100"
		aria-valuenow={position}
		onpointerdown={startSplit}
		onkeydown={(event) => {
			if (event.key === 'ArrowLeft') position = Math.max(0, position - 1);
			if (event.key === 'ArrowRight') position = Math.min(100, position + 1);
		}}><span></span></button
	>
	<div class="tag original">original</div>
	<div class="tag compressed">compressed</div>
	<div
		class="view-controls"
		role="group"
		aria-label="Preview zoom controls"
		onpointerdown={(event) => event.stopPropagation()}
	>
		<button
			type="button"
			aria-label="Zoom out"
			title="Zoom out"
			disabled={zoom <= 1}
			onclick={() => changeZoom(-0.25)}><ZoomOut size={15} strokeWidth={1.5} /></button
		>
		<span>{Math.round(zoom * 100)}%</span>
		<button
			type="button"
			aria-label="Zoom in"
			title="Zoom in"
			disabled={zoom >= 10}
			onclick={() => changeZoom(0.25)}><ZoomIn size={15} strokeWidth={1.5} /></button
		>
	</div>
	<button
		class="fullscreen"
		type="button"
		aria-label="View preview fullscreen"
		title="View fullscreen"
		onpointerdown={(event) => event.stopPropagation()}
		onclick={toggleFullscreen}><Maximize2 size={16} strokeWidth={1.5} /></button
	>
</div>

<style>
	.slider {
		position: relative;
		container-type: inline-size;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		background: repeating-conic-gradient(#e2e0d8 0% 25%, #ebe9e2 0% 50%) 50% / 20px 20px;
		touch-action: none;
		user-select: none;
	}
	.slider img {
		width: 100%;
		height: 100%;
		object-fit: contain;
		pointer-events: none;
		will-change: transform;
		backface-visibility: hidden;
	}
	.base {
		display: block;
		transform: translate(var(--pan-x), var(--pan-y)) scale(var(--zoom));
	}
	.clipped {
		position: absolute;
		inset: 0 calc(100% - var(--split)) 0 0;
		overflow: hidden;
		border-right: 1px solid #c6f04a;
	}
	.clipped img {
		width: 100cqw;
		max-width: none;
		transform: translate(var(--pan-x), var(--pan-y)) scale(var(--zoom));
	}
	.handle {
		position: absolute;
		top: 0;
		bottom: 0;
		left: var(--split);
		width: 28px;
		margin-left: -14px;
		background: linear-gradient(
			to right,
			transparent 13px,
			#c6f04a 13px,
			#c6f04a 14px,
			transparent 14px
		);
		border: 0;
		padding: 0;
		cursor: ew-resize;
		pointer-events: none;
	}
	.handle {
		pointer-events: auto;
	}
	.handle span {
		position: absolute;
		top: 50%;
		left: 50%;
		width: 28px;
		height: 28px;
		border: 1px solid #1c1d1b;
		border-radius: 50%;
		background: #c6f04a;
		transform: translate(-50%, -50%);
	}
	.handle span::before,
	.handle span::after {
		position: absolute;
		top: 12px;
		width: 5px;
		height: 1px;
		background: #1c1d1b;
		content: '';
	}
	.handle span::before {
		left: 6px;
	}
	.handle span::after {
		right: 6px;
	}
	.slider.grabbing {
		cursor: grabbing;
	}
	.tag {
		position: absolute;
		top: 12px;
		padding: 5px 7px;
		background: rgba(28, 29, 27, 0.75);
		color: #f4f1eb;
		font-size: 9px;
		text-transform: uppercase;
	}
	.tag.original {
		left: 12px;
	}
	.tag.compressed {
		right: 12px;
	}
	.fullscreen {
		position: absolute;
		right: 12px;
		bottom: 12px;
		z-index: 2;
		display: grid;
		width: 30px;
		height: 30px;
		place-items: center;
		border: 1px solid rgba(244, 241, 235, 0.7);
		background: rgba(28, 29, 27, 0.75);
		color: #f4f1eb;
		cursor: pointer;
	}
	.fullscreen:hover {
		background: #1c1d1b;
	}
	.view-controls {
		position: absolute;
		right: 50px;
		bottom: 11px;
		z-index: 3;
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 4px;
		background: rgba(28, 29, 27, 0.75);
		color: #f4f1eb;
		font-size: 9px;
	}
	.view-controls button {
		display: grid;
		width: 24px;
		height: 24px;
		place-items: center;
		border: 0;
		background: transparent;
		color: inherit;
		cursor: pointer;
	}
	.view-controls button:hover:not(:disabled) {
		background: rgba(244, 241, 235, 0.15);
	}
	.view-controls button:disabled {
		color: #777a72;
		cursor: default;
	}
	.view-controls span {
		min-width: 34px;
		text-align: center;
	}
	.slider:fullscreen {
		width: 100vw;
		height: 100vh;
		max-width: none;
		aspect-ratio: auto;
		background: #1c1d1b;
	}
	@media (max-width: 650px) {
		.slider {
			aspect-ratio: 16 / 9;
		}
	}
</style>
