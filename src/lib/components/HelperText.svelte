<script lang="ts">
	import { onMount } from 'svelte';
	import { X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button/index.js';

	let {
		collapsed = $bindable(false),
		ariaLabel,
		dismissLabel,
		storageKey,
		title,
		body
	} = $props<{
		collapsed?: boolean;
		ariaLabel: string;
		dismissLabel: string;
		storageKey: string;
		title: string;
		body: string;
	}>();

	function dismissHelper() {
		collapsed = true;
		localStorage.setItem(storageKey, '1');
	}

	onMount(() => {
		collapsed = localStorage.getItem(storageKey) === '1';
	});
</script>

{#if !collapsed}
	<div class="helper-banner" aria-label={ariaLabel}>
		<div class="helper-copy">
			<p class="helper-title">{title}</p>
			<p>{body}</p>
		</div>
		<Button
			type="button"
			size="icon-xs"
			variant="ghost"
			class="size-5 hover:cursor-pointer"
			aria-label={dismissLabel}
			onclick={dismissHelper}
		>
			<X size={14} />
		</Button>
	</div>
{/if}

<style>
	.helper-banner {
		display: grid;
		grid-template-columns: minmax(0, 1fr) auto;
		gap: 12px;
		align-items: start;
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
</style>
