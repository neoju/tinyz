<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import HelperText from './HelperText.svelte';
	import ComparisonSlider from './ComparisonSlider.svelte';
	import { m } from '$lib/paraglide/messages.js';
	import type { ImageResult } from '$lib/types';
	import { formatBytes, reductionPercent } from '$lib/utils';
	import { CircleQuestionMark } from 'lucide-svelte';

	let { item } = $props<{ item: ImageResult }>();
	let helperCollapsed = $state(false);

	const helperKey = 'tinyz-preview-helper-dismissed';

	function getCompressedSize(item: ImageResult): string {
		if (item.compressedBytes === undefined) return '--';

		return formatBytes(item.compressedBytes);
	}
</script>

<section class="comparison">
	<div class="section-heading">
		<div class="flex items-center">
			<span class="font-bold">01 / Result preview</span>
			{#if helperCollapsed}
				<Button
					type="button"
					variant="ghost"
					class="border-0 bg-transparent hover:cursor-pointer"
					size="icon-xs"
					aria-label={m.preview_tips_show()}
					onclick={() => (helperCollapsed = false)}
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

	<HelperText
		bind:collapsed={helperCollapsed}
		storageKey={helperKey}
		ariaLabel={m.preview_tips_title()}
		title={m.preview_tips_title()}
		body={m.preview_tips_body()}
		dismissLabel={m.preview_tips_hide()}
	/>

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
