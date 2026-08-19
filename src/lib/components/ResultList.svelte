<script lang="ts">
	import { Download, Trash2 } from 'lucide-svelte';
	import type { ImageResult, OutputFormat } from '$lib/types';
	import {
		AlertDialog,
		AlertDialogAction,
		AlertDialogCancel,
		AlertDialogContent,
		AlertDialogDescription,
		AlertDialogFooter,
		AlertDialogHeader,
		AlertDialogTitle,
		AlertDialogTrigger
	} from '$lib/components/ui/alert-dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
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

		<div class="actions">
			<AlertDialog>
				<AlertDialogTrigger>
					{#snippet child({ props })}
						<Button
							{...props}
							class="hover:cursor-pointer"
							variant="destructive"
							size="icon-sm"
							aria-label="Clear output queue"
							title="Clear output queue"
							disabled={busy}
						>
							<Trash2 aria-hidden="true" />
						</Button>
					{/snippet}
				</AlertDialogTrigger>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Clear output queue?</AlertDialogTitle>
						<AlertDialogDescription>
							This removes all processed images and previews from this session. This cannot be
							undone.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel>Cancel</AlertDialogCancel>
						<AlertDialogAction onclick={onClear}>Clear</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>

			<Button
				variant="outline"
				size="sm"
				class="hover:cursor-pointer"
				aria-label="Download all images as ZIP"
				title="Download all as ZIP"
				disabled={busy || !items.some((item: ImageResult) => item.compressedUrl)}
				onclick={onDownloadAll}
			>
				ZIP <Download aria-hidden="true" />
			</Button>
		</div>
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
		margin-top: 35px;
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
	.actions {
		display: flex;
		gap: 8px;
	}
	:global(.row-download svg) {
		width: 14px;
		height: 14px;
		fill: none;
		stroke: currentColor;
		stroke-linecap: round;
		stroke-linejoin: round;
		stroke-width: 1.5;
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
	.row-download {
		cursor: pointer;
	}
	@media (max-width: 650px) {
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
	}
</style>
