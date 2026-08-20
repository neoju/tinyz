<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import { getLocale, setLocale } from '$lib/paraglide/runtime.js';
	import {
		Select,
		SelectContent,
		SelectItem,
		SelectTrigger,
		SelectValue
	} from '$lib/components/ui/select/index.js';

	const locale = $derived(getLocale());

	function switchLocale(nextLocale: string) {
		if (nextLocale === 'en' || nextLocale === 'vi') {
			void setLocale(nextLocale);
		}
	}
</script>

<header class="flex w-full justify-center border-b border-dashed">
	<nav class="nav border-dashed md:border-x">
		<a class="brand" href={resolve('/')} aria-label={m.brand_home()}>
			<span class="brand-mark">tz</span> tinyz
		</a>
		<div class="nav-actions">
			<span class="privacy">
				<span class="status-dot"></span>
				{m.brand_tagline()}
			</span>
			<Select value={locale} type="single" onValueChange={switchLocale}>
				<SelectTrigger class="locale-switcher-shell" size="sm" aria-label={m.language_label()}>
					<SelectValue>
						<span class="text-xs">
							{locale === 'vi' ? m.language_vi() : m.language_en()}
						</span>
					</SelectValue>
				</SelectTrigger>
				<SelectContent class="locale-menu-shell">
					<SelectItem value="en">{m.language_en()}</SelectItem>
					<SelectItem value="vi">{m.language_vi()}</SelectItem>
				</SelectContent>
			</Select>
		</div>
	</nav>
</header>

<style>
	.nav {
		display: flex;
		max-width: 1100px;
		width: 100%;
		padding: 20px 34px;
		align-items: center;
		justify-content: space-between;
		font-size: 11px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}
	.nav-actions {
		display: flex;
		align-items: center;
		gap: 14px;
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
