<script lang="ts">
	import { Download, Trash2 } from 'lucide-svelte';
	import type { ImageResult, OutputFormat } from '$lib/types';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { formatKilobytes, formatMilliseconds } from '$lib/utils';

	let {
		items,
		format,
		selectedId,
		bulkTimeMs,
		busy,
		onSelect,
		onDownload,
		onDownloadAll,
		onClear
	} = $props<{
		items: ImageResult[];
		format: OutputFormat;
		selectedId: number;
		bulkTimeMs?: number;
		busy: boolean;
		onSelect: (id: number) => void;
		onDownload: (item: ImageResult) => void;
		onDownloadAll: () => void;
		onClear: () => void;
	}>();

	function getStatusText(item: ImageResult): string {
		if (item.status !== 'done' || item.compressedBytes === undefined) {
			return item.status;
		}

		const size = formatKilobytes(item.compressedBytes);
		const time = item.compressionMs ? formatMilliseconds(item.compressionMs) : '--';

		return `${size} / ${time}`;
	}

	function handleDownload(event: MouseEvent, item: ImageResult): void {
		event.stopPropagation();
		onDownload(item);
	}

	function handleRowKeydown(event: KeyboardEvent, id: number): void {
		if (event.key === 'Enter' || event.key === ' ') {
			onSelect(id);
		}
	}
</script>

<section class="results">
	<div class="section-heading">
		<span>
			02 / output queue
			{#if bulkTimeMs}
				<small>{formatMilliseconds(bulkTimeMs)} total</small>
			{/if}
		</span>

		<button
			class="clear"
			type="button"
			aria-label="Clear output queue"
			title="Clear output queue"
			disabled={busy}
			onclick={onClear}
		>
			<Trash2 size={14} strokeWidth={1.5} aria-hidden="true" />
			<span>clear</span>
		</button>

		<button
			class="download-all"
			onclick={onDownloadAll}
			aria-label="Download all images as ZIP"
			title="Download all as ZIP"
			disabled={busy || !items.some((item: ImageResult) => item.compressedUrl)}
		>
			<Download size={14} strokeWidth={1.5} aria-hidden="true" />
			<span>download all as zip</span>
		</button>
	</div>

	<div class="list">
		<ScrollArea class="list-scroll">
			{#each items as item (item.id)}
				<div
					class:selected={item.id === selectedId}
					class="row"
					role="button"
					tabindex="0"
					onclick={() => onSelect(item.id)}
					onkeydown={(event) => handleRowKeydown(event, item.id)}
				>
					<span class="row-number">{String(item.id + 1).padStart(2, '0')}</span>
					<span class="row-name">{item.name}</span>
					<span class="row-status">{getStatusText(item)}</span>
					{#if item.compressedUrl}
						<button
							class="row-download"
							type="button"
							aria-label={`Download ${item.outputName}`}
							title={`Download ${item.outputName}`}
							onclick={(event) => handleDownload(event, item)}
						>
							<Download size={14} strokeWidth={1.5} aria-hidden="true" />
						</button>
					{/if}
				</div>
			{/each}
		</ScrollArea>
	</div>
	<p class="format-note">Outputs are encoded as {format.toUpperCase()}.</p>
</section>

<style>
	.results {
		margin-top: 65px;
	}
	.section-heading {
		display: flex;
		align-items: center;
		justify-content: space-between;
		color: #7b7e76;
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}
	.section-heading button {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		border: 0;
		background: none;
		color: #596a1f;
		font: inherit;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		cursor: pointer;
	}
	:global(.section-heading svg),
	:global(.row-download svg) {
		width: 14px;
		height: 14px;
		fill: none;
		stroke: currentColor;
		stroke-linecap: round;
		stroke-linejoin: round;
		stroke-width: 1.5;
	}
	.section-heading .clear {
		margin-left: auto;
		color: #888a82;
	}
	.section-heading button:disabled {
		color: #aeb1a7;
		cursor: default;
	}
	.section-heading button:not(.clear) span {
		margin-left: 14px;
		color: #798d2e;
	}
	.section-heading small {
		margin-left: 10px;
		color: #798d2e;
		font-size: 9px;
		letter-spacing: 0.05em;
	}
	.list {
		margin-top: 15px;
		max-height: 520px;
		overflow-y: auto;
		overscroll-behavior: contain;
		scrollbar-color: #a8b77a transparent;
		border-top: 1px solid #d4d3ca;
	}
	:global(.list-scroll) {
		height: 100%;
	}
	.row {
		display: grid;
		width: 100%;
		grid-template-columns: 38px 1fr 100px 45px;
		gap: 10px;
		align-items: center;
		padding: 14px 8px;
		border: 0;
		border-bottom: 1px solid #d4d3ca;
		background: transparent;
		color: #373832;
		font: inherit;
		text-align: left;
		cursor: pointer;
	}
	.row.selected {
		background: rgba(198, 240, 74, 0.12);
	}
	.row-number,
	.row-status,
	.format-note {
		text-wrap: nowrap;
		color: #888a82;
		font-size: 10px;
	}
	.row-name {
		overflow: hidden;
		font-size: 12px;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	.row-download {
		display: inline-grid;
		width: 28px;
		height: 28px;
		place-items: center;
		padding: 0;
		border: 1px solid transparent;
		background: transparent;
		color: #798d2e;
		font-size: 10px;
		text-transform: uppercase;
	}
	.row-download:hover {
		border-color: #a8b77a;
	}
	.format-note {
		margin-top: 12px;
		font-size: 9px;
	}
	.section-heading button,
	.row-download {
		cursor: pointer;
	}
	@media (max-width: 650px) {
		.results {
			margin-top: 36px;
		}
		.row {
			grid-template-columns: 24px minmax(0, 1fr) 92px 28px;
			gap: 6px;
		}
		.row-status {
			font-size: 10px;
			white-space: nowrap;
		}
		.row-name {
			font-size: 13px;
		}
		.section-heading {
			font-size: 11px;
			gap: 8px;
			flex-wrap: wrap;
		}
		.section-heading > span:first-child {
			white-space: nowrap;
		}
		.section-heading small {
			margin-left: 5px;
			white-space: nowrap;
		}
		.section-heading button {
			min-height: 32px;
			padding: 6px 8px;
			border: 1px solid #c7c8c0;
			font-size: 9px;
			white-space: nowrap;
		}
		.section-heading .clear {
			min-width: 64px;
		}
		.section-heading .download-all {
			justify-content: center;
		}
		:global(.section-heading button svg) {
			width: 16px;
			height: 16px;
		}
		.download-all span {
			font-size: 0;
			margin-left: 0;
		}
		.download-all span::after {
			content: 'ZIP';
			font-size: 9px;
		}
	}
</style>
