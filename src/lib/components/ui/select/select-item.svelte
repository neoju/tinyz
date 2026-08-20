<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import CheckIcon from 'lucide-svelte/icons/check';
	import { cn, type WithoutChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		value,
		label,
		children: childrenProp,
		...restProps
	}: WithoutChild<SelectPrimitive.ItemProps> = $props();
</script>

	<SelectPrimitive.Item
		bind:ref
		{value}
		data-slot="select-item"
		class={cn(
			"focus:bg-[rgba(198,240,74,0.12)] focus:text-[#1c1d1b] not-data-[variant=destructive]:focus:**:text-[#1c1d1b] data-highlighted:bg-[rgba(198,240,74,0.12)] data-highlighted:text-[#1c1d1b] relative flex w-full cursor-default items-center gap-1.5 rounded-none py-1 pr-8 pl-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2",
			className
		)}
	{...restProps}
>
	{#snippet children({ selected, highlighted })}
		<span class="absolute end-2 flex size-3.5 items-center justify-center">
			{#if selected}
				<CheckIcon class="cn-select-item-indicator-icon" />
			{/if}
		</span>
		<span class="flex flex-1 shrink-0 gap-2 whitespace-nowrap">
			{#if childrenProp}
				{@render childrenProp({ selected, highlighted })}
			{:else}
				{label || value}
			{/if}
		</span>
	{/snippet}
</SelectPrimitive.Item>
