<script lang="ts">
	import { Maximize2, Minimize2, RotateCcw, ZoomIn, ZoomOut } from 'lucide-svelte';
	import { useFullscreen } from '$lib/hooks/useFullscreen';

	let { originalUrl, compressedUrl } = $props<{
		originalUrl: string;
		compressedUrl: string;
	}>();

	const fullscreen = useFullscreen();

	let position = $state(50);
	let zoom = $state(1);
	let fallbackFullscreen = $state(false);
	let panX = $state(0);
	let panY = $state(0);
	let gesture = $state<'split' | 'pan' | null>(null);
	let lastPointerX = 0;
	let lastPointerY = 0;
	let pendingPointerX = 0;
	let pendingPointerY = 0;
	let animationFrame: number | undefined;
	let slider: HTMLDivElement;
	let fullscreenButton: HTMLButtonElement;

	function changeZoom(amount: number) {
		if (!slider) return;

		zoom = Math.min(10, Math.max(1, Number((zoom + amount).toFixed(2))));
		slider.style.setProperty('--zoom', String(zoom));

		if (zoom === 1) {
			panX = 0;
			panY = 0;
			slider.style.setProperty('--pan-x', '0px');
			slider.style.setProperty('--pan-y', '0px');
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

	async function toggleFullscreen() {
		if (fallbackFullscreen) {
			fallbackFullscreen = false;
			return;
		}

		if (fullscreen.isFullscreen()) {
			await fullscreen.exit();
			return;
		}

		if (!(await fullscreen.request(slider))) fallbackFullscreen = true;
	}

	$effect(() => {
		if (!fallbackFullscreen) return;

		const previousOverflow = document.body.style.overflow;
		document.body.style.overflow = 'hidden';
		document.body.classList.add('fullscreen-fallback-open');

		return () => {
			document.body.style.overflow = previousOverflow;
			document.body.classList.remove('fullscreen-fallback-open');
		};
	});
</script>

<svelte:window onpointermove={movePointer} onpointerup={endPointer} />

<div
	bind:this={slider}
	class:grabbing={gesture === 'pan'}
	class:split-grabbing={gesture === 'split'}
	class:fullscreen-fallback={fallbackFullscreen}
	class="slider aspect-square md:aspect-video"
	role="application"
	aria-label="Image comparison preview. Drag the image when zoomed to pan."
	style={`--split: ${position}%; --zoom: ${zoom}; --pan-x: ${panX}px; --pan-y: ${panY}px`}
	onpointerdown={startPan}
>
	<img class="base" src={compressedUrl} alt="Compressed preview" draggable="false" />

	<div class="clipped">
		<img src={originalUrl} alt="Original preview" draggable="false" />
	</div>

	<button
		class="handle"
		type="button"
		role="slider"
		aria-label="Comparison position"
		aria-valuemin="0"
		aria-valuemax="100"
		aria-valuenow={position}
		onpointerdown={startSplit}
	></button>

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
			onclick={() => changeZoom(-0.25)}
		>
			<ZoomOut size={15} strokeWidth={1.5} />
		</button>

		<button
			class="zoom-reset"
			type="button"
			aria-label="Reset zoom to 100%"
			title="Reset zoom to 100%"
			onclick={() => changeZoom(1 - zoom)}
		>
			<span class="zoom-value">{Math.round(zoom * 100)}%</span>
			<span class="zoom-reset-icon" aria-hidden="true">
				<RotateCcw size={14} strokeWidth={1.5} />
			</span>
		</button>

		<button
			type="button"
			aria-label="Zoom in"
			title="Zoom in"
			disabled={zoom >= 10}
			onclick={() => changeZoom(0.25)}
		>
			<ZoomIn size={15} strokeWidth={1.5} />
		</button>
	</div>

	<button
		class="fullscreen"
		bind:this={fullscreenButton}
		type="button"
		aria-label={fallbackFullscreen ? 'Exit preview fullscreen' : 'View preview fullscreen'}
		title={fallbackFullscreen ? 'Exit fullscreen' : 'View fullscreen'}
		onpointerdown={(event) => event.stopPropagation()}
		onclick={toggleFullscreen}
	>
		{#if fallbackFullscreen}
			<Minimize2 size={16} strokeWidth={1.5} />
		{:else}
			<Maximize2 size={16} strokeWidth={1.5} />
		{/if}
	</button>
</div>

<style>
	.slider {
		position: relative;
		container-type: inline-size;
		overflow: hidden;
		background: repeating-conic-gradient(#e2e0d8 0% 25%, #ebe9e2 0% 50%) 50% / 20px 20px;
		touch-action: none;
		user-select: none;
	}
	.slider.fullscreen-fallback {
		position: fixed;
		inset: 0;
		z-index: 100;
		width: 100vw;
		height: 100dvh;
		max-width: none;
		max-height: none;
		aspect-ratio: auto;
		overscroll-behavior: none;
	}
	@supports not (height: 100dvh) {
		.slider.fullscreen-fallback {
			height: 100vh;
		}
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
		cursor:
			url('/tz-cursor.svg') 12 12,
			ew-resize;
		pointer-events: none;
	}
	.handle {
		pointer-events: auto;
	}
	.slider.split-grabbing,
	.slider.split-grabbing * {
		cursor:
			url('/tz-cursor.svg') 12 12,
			ew-resize !important;
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
		bottom: 12px;
		z-index: 3;
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 2px;
		background: rgba(28, 29, 27, 0.75);
		border: 1px solid rgba(244, 241, 235, 0.7);
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
	.view-controls .zoom-reset {
		width: 34px;
		overflow: hidden;
	}
	.zoom-reset > span {
		grid-area: 1 / 1;
		transition:
			opacity 120ms ease,
			transform 120ms ease;
	}
	.zoom-reset-icon {
		display: grid;
		place-items: center;
		opacity: 0;
		transform: scale(0.75);
	}
	.zoom-reset:hover .zoom-value,
	.zoom-reset:focus-visible .zoom-value {
		opacity: 0;
		transform: scale(0.85);
	}
	.zoom-reset:hover .zoom-reset-icon,
	.zoom-reset:focus-visible .zoom-reset-icon {
		opacity: 1;
		transform: scale(1);
	}
	@media (prefers-reduced-motion: reduce) {
		.zoom-reset > span {
			transition: none;
		}
	}
</style>
