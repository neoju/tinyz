<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';

	let { busy, onFiles } = $props<{
		busy: boolean;
		onFiles: (files: FileList | File[]) => void;
	}>();
	let dragging = $state(false);

	function handleDragOver(event: DragEvent): void {
		event.preventDefault();
		dragging = true;
	}

	function handleDragLeave(): void {
		dragging = false;
	}

	function handleDrop(event: DragEvent): void {
		event.preventDefault();
		dragging = false;
		onFiles(event.dataTransfer?.files ?? []);
	}

	function handleFileChange(event: Event): void {
		onFiles((event.currentTarget as HTMLInputElement).files ?? []);
	}
</script>

<label
	aria-label="Image upload"
	class:dragging
	class="drop-zone cursor-pointer"
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
>
	<input
		type="file"
		accept="image/png,image/jpeg,image/webp"
		multiple
		onchange={handleFileChange}
	/>
	<div class="drop-icon">+</div>
	<h2>{busy ? m.dropzone_busy() : m.dropzone_idle()}</h2>
	<p>or <span class="browse">{m.dropzone_browse()}</span></p>
	<span class="drop-note">{m.dropzone_note()}</span>
</label>

<style>
	.drop-zone {
		display: flex;
		min-height: 235px;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		border: 1px dashed #aeb1a7;
		background: rgba(255, 255, 255, 0.2);
		text-align: center;
		transition: 0.2s ease;
	}
	.drop-zone.dragging,
	.drop-zone:hover {
		border-color: #798d2e;
		background: rgba(198, 240, 74, 0.12);
	}
	.drop-zone input {
		display: none;
	}
	.drop-icon {
		width: 43px;
		height: 43px;
		margin-bottom: 18px;
		border: 1px solid #9ca095;
		border-radius: 50%;
		color: #798d2e;
		font:
			28px/39px Georgia,
			serif;
	}
	h2 {
		margin: 0 0 8px;
		font:
			400 22px Georgia,
			serif;
	}
	.drop-zone p {
		margin: 0 0 20px;
		color: #7b7e76;
		font-size: 12px;
	}
	.browse {
		border-bottom: 1px solid #798d2e;
		color: #596a1f;
	}
	.drop-note {
		color: #7b7e76;
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}
</style>
