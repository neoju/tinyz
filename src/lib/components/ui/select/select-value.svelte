<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import type { WithoutChild } from '$lib/utils.js';

	let { ref = $bindable(null), ...restProps }: WithoutChild<SelectPrimitive.ValueProps> = $props();
</script>

<SelectPrimitive.Value bind:ref data-slot="select-value" {...restProps} />
