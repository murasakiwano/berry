<script lang="ts" generics="TData">
	import type { Table } from "@tanstack/table-core";
	import { Button, DropdownMenu } from "bits-ui";
	import { Check } from "@lucide/svelte";

	let { table }: { table: Table<TData> } = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button.Root {...props} class="btn btn-outline ml-auto">View</Button.Root>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Portal>
		<DropdownMenu.Content align="end" side="top" class="menu bg-base-200">
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading class="px-2 py-1.5 text-sm font-bold"
					>Toggle columns</DropdownMenu.GroupHeading
				>
				<DropdownMenu.Separator class="bg-neutral -mx-1 my-1 h-px" />
				{#each table.getAllColumns().filter((col) => col.getCanHide()) as column (column)}
					<DropdownMenu.CheckboxItem
						class="data-[highlighted]:bg-accent data-[highlighted]:text-accent-content relative flex cursor-default items-center rounded-sm p-1.5 pr-2 pl-8 text-sm capitalize outline-none select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
						bind:checked={
							() => column.getIsVisible(), (v) => column.toggleVisibility(v)
						}
					>
						{#if column.getIsVisible()}
							<span
								class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center"
							>
								<Check class="h-4 w-4" />
							</span>
						{/if}
						{column.columnDef.id}
					</DropdownMenu.CheckboxItem>
				{/each}
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Portal>
</DropdownMenu.Root>
