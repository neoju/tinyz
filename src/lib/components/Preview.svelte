<script lang="ts">
	import ComparisonSlider from './ComparisonSlider.svelte';
	import type { ImageResult } from '$lib/types';
	import { formatBytes, reductionPercent } from '$lib/utils';

	let { item } = $props<{ item: ImageResult }>();

	function getCompressedSize(item: ImageResult): string {
		if (item.compressedBytes === undefined) return '--';

		return formatBytes(item.compressedBytes);
	}
</script>

<section class="comparison">
	<div class="section-heading">
		<span>01 / Your images</span>
		{#if item.compressedBytes}
			<strong>
				{reductionPercent(item.originalBytes, item.compressedBytes)}% smaller
			</strong>
		{/if}
	</div>

	{#if item.compressedUrl}
		<ComparisonSlider originalUrl={item.originalUrl} compressedUrl={item.compressedUrl} />
	{:else}
		<div class="waiting aspect-video w-full px-6">
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
