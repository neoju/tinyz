<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';
</script>

<section class="hero">
	<p class="eyebrow">{m.hero_eyebrow()}</p>
	<h1>
		{m.hero_title().split('\n')[0]}<br /><em>{m.hero_title().split('\n')[1]}</em>
	</h1>
	<p class="intro">{m.hero_intro()}</p>
</section>

<style>
	.hero {
		max-width: 700px;
		margin: 85px 0 48px;
	}
	.eyebrow {
		color: #7b7e76;
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}
	h1 {
		margin: 16px 0 20px;
		font:
			400 clamp(45px, 7vw, 78px)/0.92 Georgia,
			serif;
		letter-spacing: -0.07em;
	}
	.hero h1 em {
		color: #798d2e;
		font-style: normal;
	}
	.intro {
		max-width: 520px;
		color: #686b64;
		font-size: 13px;
		line-height: 1.7;
	}
	@media (max-width: 650px) {
		.hero {
			margin: 48px 0 24px;
		}
		.hero h1 {
			font-size: 54px;
		}
	}
</style>
