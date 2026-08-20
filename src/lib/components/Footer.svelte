<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';
</script>

<footer class="flex justify-center border-t border-dashed">
	<div
		class="footer-content flex flex-col justify-center gap-1 border-dashed
    text-center md:flex-row md:justify-between md:border-x"
	>
		<span>{m.footer_year()}</span>
		<span>{m.footer_powered()}</span>
	</div>
</footer>

<style>
	footer {
		margin-top: auto;
		color: #9b9c94;
		font-size: 9px;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}
	.footer-content {
		padding: 20px 34px;
		max-width: 1100px;
		width: 100%;
	}
</style>
