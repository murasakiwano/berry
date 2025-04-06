<script lang="ts">
	import { Select, type WithoutChildren } from "bits-ui";

	type Props = WithoutChildren<Select.RootProps> & {
		triggerClass?: string;
		placeholder?: string;
		items: { value: string; label: string; disabled?: boolean }[];
		contentProps?: WithoutChildren<Select.ContentProps>;
	};

	let {
		value = $bindable(),
		items,
		contentProps,
		placeholder,
		triggerClass,
		...restProps
	}: Props = $props();

	const selectedLabel = $derived(items.find((item) => item.value === value)?.label);
</script>

<!--
TypeScript Discriminated Unions + destructing (required for "bindable") do not
get along, so we shut typescript up by casting `value` to `never`, however,
from the perspective of the consumer of this component, it will be typed appropriately.
-->
<Select.Root bind:value={value as never} {...restProps}>
	<Select.Trigger class={triggerClass}>
		{selectedLabel ? selectedLabel : placeholder}
	</Select.Trigger>
	<Select.Portal>
		<Select.Content {...contentProps}>
			<Select.ScrollUpButton>up</Select.ScrollUpButton>
			<Select.Viewport>
				{#each items as { value, label, disabled } (value)}
					<Select.Item {value} {label} {disabled}>
						{#snippet children()}
							{label}
						{/snippet}
					</Select.Item>
				{/each}
			</Select.Viewport>
			<Select.ScrollDownButton>down</Select.ScrollDownButton>
		</Select.Content>
	</Select.Portal>
</Select.Root>
