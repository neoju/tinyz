<script lang="ts">
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';
	import { zipSync } from 'fflate';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	import ComparisonSlider from '$lib/components/ComparisonSlider.svelte';
	import DropZone from '$lib/components/DropZone.svelte';
	import ResultList from '$lib/components/ResultList.svelte';
	import type { ImageResult, OutputFormat } from '$lib/types';
	import { formatBytes, mimeType, reductionPercent } from '$lib/utils';
	import CompressorWorker from '$lib/workers/compressor.worker?worker';

	let workers: Worker[] = [];
	let nextWorker = 0;
	let recompressButton: HTMLButtonElement;
	let items = $state<ImageResult[]>([]);
	let selectedId = $state(0);
	let quality = $state(80);
	let format = $state<OutputFormat>('png');
	let settingsDirty = $state(false);
	let error = $state('');
	let files = new SvelteMap<number, File>();
	let activeBatchIds = new SvelteSet<number>();
	let batchStartedAt = 0;
	let bulkTimeMs = $state<number | undefined>();

	const selected = $derived(
		items.find((item) => item.id === selectedId) ?? items[0]
	);
	const busy = $derived(
		items.some(
			(item) => item.status === 'compressing' || item.status === 'queued'
		)
	);

	function handleWorkerMessage(
		event: MessageEvent<{
			type: string;
			id?: number;
			bytes?: ArrayBuffer;
			compressionMs?: number;
			message?: string;
			format?: OutputFormat;
		}>
	) {
		if (event.data.id === undefined) return;
		if (event.data.format && event.data.format !== format) return;
		const index = items.findIndex((item) => item.id === event.data.id);
		if (index < 0) return;
		if (event.data.type === 'started') {
			items[index] = { ...items[index], status: 'compressing' };
			items = items;
			return;
		}
		if (event.data.type === 'result' && event.data.bytes) {
			const item = items[index];
			if (item.compressedUrl) URL.revokeObjectURL(item.compressedUrl);
			const bytes = new Uint8Array(event.data.bytes);
			const outputName = `${item.name.replace(/\.[^.]+$/, '')}-tiny.${format === 'jpeg' ? 'jpg' : format}`;
			items[index] = {
				...item,
				compressedBytes: bytes.byteLength,
				compressionMs: event.data.compressionMs,
				compressedUrl: URL.createObjectURL(
					new Blob([bytes], { type: mimeType(format) })
				),
				outputName,
				status: 'done'
			};
		} else if (event.data.type === 'error') {
			items[index] = {
				...items[index],
				status: 'error',
				error: event.data.message
			};
		}
		activeBatchIds.delete(event.data.id);
		if (!activeBatchIds.size && batchStartedAt) {
			bulkTimeMs = Math.round(performance.now() - batchStartedAt);
			batchStartedAt = 0;
		}
		items = items;
	}

	onMount(() => {
		const workerCount = Math.min(
			4,
			Math.max(1, (navigator.hardwareConcurrency || 2) - 1)
		);
		workers = Array.from({ length: workerCount }, () => new CompressorWorker());
		for (const worker of workers) worker.onmessage = handleWorkerMessage;

		return () => {
			for (const worker of workers) worker.terminate();
			for (const item of items) {
				URL.revokeObjectURL(item.originalUrl);
				if (item.compressedUrl) URL.revokeObjectURL(item.compressedUrl);
			}
		};
	});

	function compress(id: number, file: File) {
		const queuedIndex = items.findIndex((item) => item.id === id);
		if (queuedIndex >= 0) {
			items[queuedIndex] = { ...items[queuedIndex], status: 'queued' };
			items = items;
		}
		file.arrayBuffer().then((input) => {
			const worker = workers[nextWorker % workers.length];
			nextWorker += 1;
			worker?.postMessage({ type: 'compress', id, input, quality, format }, [
				input
			]);
		});
	}

	function startBatch(batch: ImageResult[]) {
		activeBatchIds = new SvelteSet(batch.map((item) => item.id));
		batchStartedAt = performance.now();
		bulkTimeMs = undefined;
		for (const item of batch) {
			const file = files.get(item.id);
			if (file) compress(item.id, file);
		}
	}

	async function chooseFiles(input: FileList | File[]) {
		const accepted = Array.from(input).filter((file) =>
			['image/png', 'image/jpeg', 'image/webp'].includes(file.type)
		);
		if (!accepted.length) {
			error = 'Choose PNG, JPEG, or WebP images to begin.';
			return;
		}
		error = '';
		const wasEmpty = items.length === 0;
		const next = accepted.map((file, offset) => {
			const id = items.length + offset;
			files.set(id, file);
			return {
				id,
				name: file.name,
				originalBytes: file.size,
				originalUrl: URL.createObjectURL(file),
				status: 'queued' as const
			};
		});
		items = [...items, ...next];
		if (wasEmpty) selectedId = next[0].id;
		startBatch(next);
		requestAnimationFrame(() =>
			recompressButton?.scrollIntoView({ behavior: 'smooth', block: 'start' })
		);
	}

	function changeFormat(event: Event) {
		format = (event.currentTarget as HTMLSelectElement).value as OutputFormat;
		settingsDirty = true;
	}

	function recompressAll() {
		settingsDirty = false;
		startBatch(items);
	}

	function download(item: ImageResult) {
		if (!item.compressedUrl || !item.outputName) return;
		const link = document.createElement('a');
		link.href = item.compressedUrl;
		link.download = item.outputName;
		link.click();
	}

	function clearQueue() {
		for (const item of items) {
			URL.revokeObjectURL(item.originalUrl);
			if (item.compressedUrl) URL.revokeObjectURL(item.compressedUrl);
		}
		items = [];
		files.clear();
		activeBatchIds.clear();
		bulkTimeMs = undefined;
		selectedId = 0;
		error = '';
	}

	async function downloadAll() {
		const ready = items.filter(
			(
				item
			): item is ImageResult & { compressedUrl: string; outputName: string } =>
				Boolean(item.compressedUrl && item.outputName)
		);
		if (!ready.length) return;

		try {
			const entries: Record<string, Uint8Array> = {};
			await Promise.all(
				ready.map(async (item, index) => {
					const response = await fetch(item.compressedUrl);
					if (!response.ok)
						throw new Error(`Could not read ${item.outputName}`);
					let filename = item.outputName;
					if (entries[filename]) filename = `${index + 1}-${filename}`;
					entries[filename] = new Uint8Array(await response.arrayBuffer());
				})
			);

			const archive = zipSync(entries);
			const url = URL.createObjectURL(
				new Blob([archive], { type: 'application/zip' })
			);
			const link = document.createElement('a');
			link.href = url;
			link.download = `tinyz-${format}.zip`;
			document.body.append(link);
			link.click();
			link.remove();
			setTimeout(() => URL.revokeObjectURL(url), 0);
		} catch (cause) {
			error =
				cause instanceof Error
					? cause.message
					: 'Could not create ZIP archive.';
		}
	}
</script>

<svelte:head>
	<title>tinyz | Private image compression</title>
	<meta
		name="description"
		content="Compress multiple images locally in your browser."
	/>
</svelte:head>

<main class="shell">
	<nav class="nav">
		<a class="brand" href={resolve('/')} aria-label="tinyz home">
			<span class="brand-mark">tz</span> tinyz
		</a>
		<span class="privacy"
			><span class="status-dot"></span> local-first compression</span
		>
	</nav>
	<section class="hero">
		<p class="eyebrow">WASM image compressor</p>
		<h1>Make images lighter.<br /><em>Keep them yours.</em></h1>
		<p class="intro">
			Drop a batch of images. tinyz compresses them in a background thread, so
			your files never leave this device.
		</p>
	</section>
	<DropZone {busy} onFiles={chooseFiles} />

	<div class="controls">
		<label for="quality">Quality <strong>{quality}</strong></label>
		<input
			id="quality"
			type="range"
			min="10"
			max="100"
			bind:value={quality}
			disabled={busy}
			onchange={() => (settingsDirty = true)}
		/>
		<label for="format">Output</label>
		<select id="format" value={format} onchange={changeFormat}>
			<option value="png">PNG</option>
			<option value="jpeg">JPEG</option>
			<option value="webp">WebP</option>
		</select>
		<button
			variant="default"
			size="sm"
			bind:this={recompressButton}
			class="recompress"
			type="button"
			disabled={!settingsDirty || busy || !items.length}
			onclick={recompressAll}>Re-compress <span>-></span></button
		>
	</div>
	{#if error}<p class="error" role="alert">{error}</p>{/if}

	{#if selected}
		<section class="comparison">
			<div class="section-heading">
				<span>01 / Your images</span>
				{#if selected.compressedBytes}
					<strong>
						{reductionPercent(
							selected.originalBytes,
							selected.compressedBytes
						)}% smaller
					</strong>
				{/if}
			</div>

			{#if selected.compressedUrl}
				<ComparisonSlider
					originalUrl={selected.originalUrl}
					compressedUrl={selected.compressedUrl}
				/>
			{:else}
				<div class="waiting aspect-video w-full px-6">
					<p
						class="w-full overflow-hidden text-center text-nowrap text-ellipsis"
					>
						Compressing {selected.name}...
					</p>
				</div>
			{/if}

			<div class="preview-meta">
				<span>{selected.name}</span>
				<span>
					{formatBytes(selected.originalBytes)} -> {selected.compressedBytes
						? formatBytes(selected.compressedBytes)
						: '--'}
				</span>
			</div>
		</section>
	{/if}
	{#if items.length}
		<ResultList
			{items}
			{format}
			{selectedId}
			{bulkTimeMs}
			{busy}
			onSelect={(id) => (selectedId = id)}
			onDownload={download}
			onDownloadAll={downloadAll}
			onClear={clearQueue}
		/>
	{:else}
		<div class="tip text-center">
			<span>i</span> Your images are processed locally. Nothing is uploaded or stored.
		</div>
	{/if}

	<footer>
		<span>tinyz / 2026</span><span
			>Powered by Rust + imagequant + WebAssembly</span
		>
	</footer>
</main>

<style>
	:global(body) {
		margin: 0;
		background: #f4f1eb;
		color: #1c1d1b;
		font-family: 'IBM Plex Mono', 'SFMono-Regular', Consolas, monospace;
	}
	:global(*) {
		box-sizing: border-box;
	}
	.shell {
		max-width: 1100px;
		margin: 0 auto;
		padding: 28px 34px 24px;
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}
	.nav {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: 11px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}
	.brand {
		color: inherit;
		font-size: 17px;
		font-weight: 700;
		letter-spacing: -0.08em;
		text-decoration: none;
		text-transform: lowercase;
	}
	.brand-mark {
		display: inline-grid;
		width: 26px;
		height: 26px;
		margin-right: 7px;
		place-items: center;
		border-radius: 50%;
		background: #c6f04a;
		color: #242b19;
		font-size: 12px;
		letter-spacing: -0.15em;
	}
	.privacy {
		color: #70736c;
	}
	.status-dot {
		display: inline-block;
		width: 6px;
		height: 6px;
		margin-right: 7px;
		border-radius: 50%;
		background: #75ad50;
	}
	.hero {
		max-width: 700px;
		margin: 105px 0 48px;
	}
	.eyebrow,
	.section-heading {
		color: #7b7e76;
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}
	h1 {
		margin: 16px 0 20px;
		font:
			400 clamp(45px, 7vw, 78px)/0.92 Georgia,
			serif;
		letter-spacing: -0.07em;
	}
	.hero h1 em {
		color: #798d2e;
		font-style: normal;
	}
	.intro {
		max-width: 520px;
		color: #686b64;
		font-size: 13px;
		line-height: 1.7;
	}
	.controls {
		display: grid;
		grid-template-columns: auto 1fr auto auto auto;
		gap: 12px;
		align-items: center;
		max-width: 650px;
		margin: 23px 0 0 auto;
		color: #777a72;
		font-size: 10px;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}
	.controls strong {
		color: #252621;
		font-size: 12px;
	}
	.controls input {
		accent-color: #798d2e;
	}
	.controls select {
		padding: 6px 25px 6px 8px;
		border: 1px solid #b9bbb1;
		background: transparent;
		color: #373832;
		font: inherit;
	}
	.recompress {
		scroll-margin-top: 10px;
		padding: 7px 10px;
		border: 1px solid #798d2e;
		background: #798d2e;
		color: #f4f1eb;
		font: inherit;
		font-size: 10px;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		cursor: pointer;
	}
	.recompress span {
		margin-left: 8px;
		color: #c6f04a;
	}
	.recompress:disabled {
		border-color: #c7c8c0;
		background: transparent;
		color: #aeb1a7;
		cursor: default;
	}
	.error {
		color: #a34d3e;
		font-size: 12px;
	}
	.comparison {
		margin-top: 65px;
	}
	.section-heading {
		display: flex;
		justify-content: space-between;
		margin-bottom: 15px;
	}
	.section-heading strong {
		color: #798d2e;
		font-size: 11px;
	}
	.waiting {
		display: grid;
		place-items: center;
		border: 1px dashed #aeb1a7;
		color: #798d2e;
		font-size: 12px;
	}
	.preview-meta {
		display: flex;
		justify-content: space-between;
		padding: 12px 0;
		color: #777a72;
		font-size: 10px;
	}
	.tip {
		margin: 70px auto 0;
		color: #888a82;
		font-size: 11px;
	}
	.tip span {
		display: inline-grid;
		width: 16px;
		height: 16px;
		margin-right: 7px;
		place-items: center;
		border: 1px solid #aeb1a7;
		border-radius: 50%;
		color: #798d2e;
	}
	footer {
		display: flex;
		justify-content: space-between;
		margin-top: auto;
		padding-top: 100px;
		color: #9b9c94;
		font-size: 9px;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}
	@media (max-width: 650px) {
		.shell {
			padding: 22px 18px;
		}
		.privacy {
			display: none;
		}
		.hero {
			margin: 48px 0 24px;
		}
		.hero h1 {
			font-size: 54px;
		}
		.controls {
			grid-template-columns: auto 1fr auto auto auto;
			gap: 8px;
		}
		.controls label {
			white-space: nowrap;
		}
		.controls input {
			min-width: 70px;
		}
		.controls select {
			padding: 6px 8px;
		}
		.recompress {
			padding: 7px 8px;
			font-size: 9px;
			white-space: nowrap;
		}
		.comparison {
			margin-top: 32px;
		}
		.waiting {
			height: 220px;
		}
		.preview-meta {
			gap: 12px;
			flex-direction: column;
		}
		footer {
			padding-top: 42px;
			flex-direction: column;
			gap: 8px;
		}
	}
</style>
