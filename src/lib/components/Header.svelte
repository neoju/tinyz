<script lang="ts">
	import { resolve } from '$app/paths';
</script>

<nav class="nav">
	<a class="brand" href={resolve('/')} aria-label="tinyz home">
		<span class="brand-mark">tz</span> tinyz
	</a>
	<span class="privacy">
		<span class="status-dot"></span> local-first compression
	</span>
</nav>

<style>
	.nav {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: 11px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}
	.brand {
		color: inherit;
		font-size: 17px;
		font-weight: 700;
		letter-spacing: -0.08em;
		text-decoration: none;
		text-transform: lowercase;
	}
	.brand-mark {
		display: inline-grid;
		width: 26px;
		height: 26px;
		margin-right: 7px;
		place-items: center;
		border-radius: 50%;
		background: #c6f04a;
		color: #242b19;
		font-size: 12px;
		letter-spacing: -0.15em;
	}
	.privacy {
		color: #70736c;
	}
	.status-dot {
		display: inline-block;
		width: 6px;
		height: 6px;
		margin-right: 7px;
		border-radius: 50%;
		background: #75ad50;
	}
	@media (max-width: 650px) {
		.privacy {
			display: none;
		}
	}
</style>
