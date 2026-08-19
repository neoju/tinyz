<script lang="ts">
	import type { OutputFormat } from '$lib/types';

	let {
		quality,
		format,
		busy,
		settingsDirty,
		hasItems,
		recompressRef,
		onqualityinput,
		onqualitychange,
		onformatchange,
		onrecompress
	} = $props<{
		quality: number;
		format: OutputFormat;
		busy: boolean;
		settingsDirty: boolean;
		hasItems: boolean;
		recompressRef: (el: HTMLButtonElement | null) => void;
		onqualityinput: (value: number) => void;
		onqualitychange: () => void;
		onformatchange: (value: OutputFormat) => void;
		onrecompress: () => void;
	}>();

	let recompressButton: HTMLButtonElement;

	$effect(() => {
		recompressRef(recompressButton);
		return () => recompressRef(null);
	});

	function handleQualityInput(event: Event) {
		onqualityinput(Number((event.currentTarget as HTMLInputElement).value));
	}

	function handleFormatChange(event: Event) {
		onformatchange((event.currentTarget as HTMLSelectElement).value as OutputFormat);
	}
</script>

<div class="controls">
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
	<select id="format" value={format} onchange={handleFormatChange}>
		<option value="png">PNG</option>
		<option value="jpeg">JPEG</option>
		<option value="webp">WebP</option>
	</select>
	<button
		bind:this={recompressButton}
		class="recompress"
		type="button"
		disabled={!settingsDirty || busy || !hasItems}
		onclick={onrecompress}>Re-compress <span>-></span></button
	>
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
		.controls select {
			padding: 6px 8px;
		}
		.recompress {
			padding: 7px 8px;
			font-size: 9px;
			white-space: nowrap;
		}
	}
</style>
