<script lang="ts">
	import type { OutputFormat } from '$lib/types';
	import { formatLabel } from '$lib/utils';
	import { Button } from '$lib/components/ui/button/index.js';
	import {
		Select,
		SelectContent,
		SelectItem,
		SelectTrigger,
		SelectValue
	} from '$lib/components/ui/select/index.js';
	import { RefreshCcw } from 'lucide-svelte';

	let {
		ref = $bindable(null),
		quality,
		format,
		busy,
		settingsDirty,
		hasItems,
		onqualityinput,
		onqualitychange,
		onformatchange,
		onrecompress
	} = $props<{
		ref?: HTMLElement | null;
		quality: number;
		format: OutputFormat;
		busy: boolean;
		settingsDirty: boolean;
		hasItems: boolean;
		onqualityinput: (value: number) => void;
		onqualitychange: () => void;
		onformatchange: (value: OutputFormat) => void;
		onrecompress: () => void;
	}>();

	function handleQualityInput(event: Event) {
		onqualityinput(Number((event.currentTarget as HTMLInputElement).value));
	}

	function handleFormatChange(value: string) {
		onformatchange(value as OutputFormat);
	}
</script>

<div class="controls" bind:this={ref}>
	<label for="quality">Quality <strong>{quality}</strong></label>
	<input
		id="quality"
		type="range"
		min="10"
		max="100"
		value={quality}
		disabled={busy}
		oninput={handleQualityInput}
		onchange={onqualitychange}
	/>
	<label for="format">Output</label>
	<Select value={format} type="single" onValueChange={handleFormatChange}>
		<SelectTrigger
			id="format"
			class="hover:cursor-pointer"
			size="sm"
			aria-label="Output format"
			disabled={busy}
		>
			<SelectValue>{formatLabel(format)}</SelectValue>
		</SelectTrigger>
		<SelectContent class="hover:**:cursor-pointer">
			<SelectItem value="png">PNG</SelectItem>
			<SelectItem value="jpeg">JPEG</SelectItem>
			<SelectItem value="webp">WebP</SelectItem>
		</SelectContent>
	</Select>
	<Button
		size="sm"
		variant="outline"
		class="hover:cursor-pointer"
		disabled={!settingsDirty || busy || !hasItems}
		onclick={onrecompress}
	>
		<span class="hidden md:inline-block">Re-compress</span>
		<RefreshCcw />
	</Button>
</div>

<style>
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
		scroll-margin-top: 10px;
	}
	.controls strong {
		color: #252621;
		font-size: 12px;
	}
	.controls input {
		accent-color: #798d2e;
	}
	:global(.recompress-arrow) {
		margin-left: 8px;
		color: #c6f04a;
	}
	@media (max-width: 650px) {
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
	}
</style>
