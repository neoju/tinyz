<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	import CompressorWorker from '$lib/workers/compressor.worker?worker';

	import Controls from '$lib/components/Controls.svelte';
	import DropZone from '$lib/components/DropZone.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Header from '$lib/components/Header.svelte';
	import Hero from '$lib/components/Hero.svelte';
	import Preview from '$lib/components/Preview.svelte';
	import ResultList from '$lib/components/ResultList.svelte';

	import type {
		ImageResult,
		OutputFormat,
		ReadyImageResult,
		WorkerResponseMessage
	} from '$lib/types';
	import {
		downloadZip,
		extensionToFormat,
		filterAcceptedImages,
		formatToExtension,
		mimeToFormat,
		mimeType,
		triggerDownload
	} from '$lib/utils';

	let workers: Worker[] = [];
	let nextWorker = 0;
	let controlsRef = $state<HTMLElement | null>(null);
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

	const selected = $derived(items.find((item) => item.id === selectedId) ?? items[0]);
	const busy = $derived(items.some((item) => ['compressing', 'queued'].includes(item.status)));

	function handleWorkerMessage(event: MessageEvent<WorkerResponseMessage>) {
		if (event.data.id === undefined || (event.data.format && event.data.format !== format)) return;

		const index = items.findIndex((item) => item.id === event.data.id);
		if (index < 0) return;

		if (event.data.type === 'started') {
			items[index] = { ...items[index], status: 'compressing' };
			return;
		}

		if (event.data.type === 'result' && event.data.bytes) {
			const item = items[index];
			if (item.compressedUrl) URL.revokeObjectURL(item.compressedUrl);

			const bytes = new Uint8Array(event.data.bytes);
			const extension = formatToExtension(format);

			items[index] = {
				...item,
				compressedBytes: bytes.byteLength,
				compressionMs: event.data.compressionMs,
				compressedUrl: URL.createObjectURL(new Blob([bytes], { type: mimeType(format) })),
				outputName: `${item.name.replace(/\.[^.]+$/, '')}-tiny.${extension}`,
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
	}

	function compress(id: number, file: File) {
		const queuedIndex = items.findIndex((item) => item.id === id);

		if (queuedIndex >= 0) {
			items[queuedIndex] = { ...items[queuedIndex], status: 'queued' };
		}

		file.arrayBuffer().then((input) => {
			const worker = workers[nextWorker % workers.length];
			nextWorker += 1;
			worker?.postMessage({ type: 'compress', id, input, quality, format }, [input]);
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

	function chooseFiles(input: FileList | File[]) {
		const accepted = filterAcceptedImages(input);

		if (!accepted.length) {
			error = 'Choose PNG, JPEG, or WebP images to begin.';
			return;
		}
		error = '';

		const wasEmpty = items.length === 0;
		if (wasEmpty) {
			const first = accepted[0];
			const ext = first.name.split('.').pop() ?? '';
			const detected = extensionToFormat(ext) ?? mimeToFormat(first.type);
			if (detected) format = detected;
		}

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
		scrollToRecompress();
	}

	function changeFormat(value: OutputFormat) {
		format = value;
		settingsDirty = true;
	}

	function markSettingsDirty() {
		settingsDirty = true;
	}

	function selectItem(id: number) {
		selectedId = id;
	}

	function scrollToRecompress() {
		requestAnimationFrame(() => {
			controlsRef?.scrollIntoView({ behavior: 'smooth', block: 'start' });
		});
	}

	function recompressAll() {
		settingsDirty = false;
		startBatch(items);
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

	function download(item: ImageResult) {
		if (!item.compressedUrl || !item.outputName) return;
		triggerDownload(item.compressedUrl, item.outputName);
	}

	function downloadAll() {
		const ready = items.filter((item) => Boolean(item.outputName && item.compressedUrl));

		if (!ready.length) return;

		downloadZip(ready as ReadyImageResult[]).catch((cause) => {
			error = cause instanceof Error ? cause.message : 'Could not create ZIP archive.';
		});
	}

	onMount(() => {
		const workerCount = Math.min(4, Math.max(1, (navigator.hardwareConcurrency || 2) - 1));
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
</script>

<svelte:head>
	<title>tinyz | Private image compression</title>
	<meta name="description" content="Compress multiple images locally in your browser." />
</svelte:head>

<Header />
<main class="shell border-dashed md:border-x">
	<Hero />
	<DropZone {busy} onFiles={chooseFiles} />

	<Controls
		bind:ref={controlsRef}
		{quality}
		{format}
		{busy}
		{settingsDirty}
		hasItems={items.length > 0}
		onqualityinput={(value) => (quality = value)}
		onqualitychange={markSettingsDirty}
		onformatchange={changeFormat}
		onrecompress={recompressAll}
	/>
	{#if error}<p class="error" role="alert">{error}</p>{/if}

	{#if selected}
		<Preview item={selected} />
	{/if}

	{#if items.length}
		<ResultList
			{items}
			{format}
			{selectedId}
			{bulkTimeMs}
			{busy}
			onSelect={selectItem}
			onDownload={download}
			onDownloadAll={downloadAll}
			onClear={clearQueue}
		/>
	{:else}
		<div class="tip text-center">
			<span>i</span> Your images are processed locally. Nothing is uploaded or stored.
		</div>
	{/if}
</main>
<Footer />

<style>
	:global(body) {
		margin: 0;
		background: #f4f1eb;
		color: #1c1d1b;
		font-family: 'IBM Plex Mono', 'SFMono-Regular', Consolas, monospace;
		min-height: 100dvh;
		display: flex;
		flex-direction: column;
	}
	:global(*) {
		box-sizing: border-box;
	}
	.shell {
		flex-grow: 1;
		max-width: 1100px;
		width: 100%;
		margin: 0 auto;
		padding: 28px 34px 24px;
		display: flex;
		flex-direction: column;
	}
	.error {
		color: #a34d3e;
		font-size: 12px;
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
	@media (max-width: 650px) {
		.shell {
			padding: 22px 18px;
		}
	}
</style>
