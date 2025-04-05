<script lang="ts">
	import { Checkbox, Label, useId, type WithoutChildrenOrChild } from "bits-ui";

	let {
		id = useId(),
		checked = $bindable(false),
		ref = $bindable(null),
		labelRef = $bindable(null),
		labelText,
		...restProps
	}: WithoutChildrenOrChild<Checkbox.RootProps> & {
		labelText?: string | null;
		labelRef?: HTMLLabelElement | null;
	} = $props();
</script>

<Checkbox.Root bind:checked bind:ref class="checkbox" {...restProps} />
{#if labelText}
	<Label.Root for={id} bind:ref={labelRef}>
		{labelText}
	</Label.Root>
{/if}
