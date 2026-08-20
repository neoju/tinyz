<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import ComparisonSlider from './ComparisonSlider.svelte';
	import type { ImageResult } from '$lib/types';
	import { formatBytes, reductionPercent } from '$lib/utils';
	import { CircleQuestionMark, X } from 'lucide-svelte';

	let { item } = $props<{ item: ImageResult }>();
	let helperCollapsed = $state(false);

	const helperKey = 'tinyz-preview-helper-dismissed';

	function getCompressedSize(item: ImageResult): string {
		if (item.compressedBytes === undefined) return '--';

		return formatBytes(item.compressedBytes);
	}

	function dismissHelper() {
		helperCollapsed = true;
		localStorage.setItem(helperKey, '1');
	}

	function openHelper() {
		helperCollapsed = false;
		localStorage.removeItem(helperKey);
	}

	onMount(() => {
		helperCollapsed = localStorage.getItem(helperKey) === '1';
	});
</script>

<section class="comparison">
	<div class="section-heading">
		<div class="flex items-center">
			<span>01 / Result preview</span>
			{#if helperCollapsed}
				<Button
					type="button"
					variant="ghost"
					class="border-0 hover:cursor-pointer"
					size="icon-xs"
					aria-label="Show comparison slider tips"
					onclick={openHelper}
				>
					<CircleQuestionMark size={14} />
				</Button>
			{/if}
		</div>

		<div class="section-heading-actions">
			{#if item.compressedBytes}
				<strong>
					{reductionPercent(item.originalBytes, item.compressedBytes)}% smaller
				</strong>
			{/if}
		</div>
	</div>

	{#if !helperCollapsed}
		<div class="helper-banner" aria-label="Comparison slider tips">
			<div class="helper-copy">
				<p class="helper-title">How to compare</p>
				<p>
					Drag the center handle to compare, use zoom to inspect details, and fullscreen for a
					larger view.
				</p>
			</div>
			<Button
				type="button"
				size="icon-xs"
				variant="ghost"
				class="size-5 hover:cursor-pointer"
				aria-label="Dismiss comparison slider tips"
				onclick={dismissHelper}
			>
				<X size={14} />
			</Button>
		</div>
	{/if}

	{#if item.compressedUrl}
		<ComparisonSlider originalUrl={item.originalUrl} compressedUrl={item.compressedUrl} />
	{:else}
		<div class="waiting aspect-square w-full px-6 md:aspect-video">
			<p class="w-full overflow-hidden text-center text-nowrap text-ellipsis">
				Compressing {item.name}...
			</p>
		</div>
	{/if}

	<div class="preview-meta">
		<span>{item.name}</span>
		<span>{formatBytes(item.originalBytes)} -> {getCompressedSize(item)}</span>
	</div>
</section>

<style>
	.comparison {
		margin-top: 35px;
	}
	.section-heading {
		display: flex;
		justify-content: space-between;
		margin-bottom: 15px;
		color: #7b7e76;
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}
	.section-heading-actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.section-heading strong {
		color: #798d2e;
		font-size: 11px;
	}
	.helper-banner {
		position: relative;
		display: grid;
		grid-template-columns: minmax(0, 1fr) auto;
		gap: 12px;
		align-items: start;
		margin-bottom: 12px;
		padding: 12px 12px 12px 14px;
		border: 1px solid #d4d3ca;
		background: rgba(251, 250, 247, 0.92);
		color: #1c1d1b;
	}
	.helper-copy {
		display: grid;
		gap: 4px;
	}
	.helper-title {
		margin: 0;
		color: #798d2e;
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}
	.helper-copy p:last-child {
		margin: 0;
		color: #7b7e76;
		font-size: 12px;
		line-height: 1.5;
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
	@media (max-width: 650px) {
		.waiting {
			height: 220px;
		}
		.preview-meta {
			gap: 12px;
			flex-direction: column;
		}
	}
</style>
