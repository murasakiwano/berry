<script lang="ts" generics="TData">
	import type { Table } from "@tanstack/table-core";
	import DataTableViewOptions from "./data-table-view-options.svelte";
	import { SOURCE_ACCOUNT_COL_ID } from "./columns";
	import { Button } from "bits-ui";
	import { X } from "@lucide/svelte";

	let { table }: { table: Table<TData> } = $props();

	const isFiltered = $derived(table.getState().columnFilters.length > 0);
</script>

<div class="flex items-center justify-between p-1">
	<div class="flex flex-1 items-center space-x-2">
		<input
			type="text"
			placeholder="Filtrar conta de origem..."
			value={(table.getColumn(SOURCE_ACCOUNT_COL_ID)?.getFilterValue() as string) ?? ""}
			onchange={(e) => {
				table.getColumn(SOURCE_ACCOUNT_COL_ID)?.setFilterValue(e.currentTarget.value);
			}}
			oninput={(e) => {
				table.getColumn(SOURCE_ACCOUNT_COL_ID)?.setFilterValue(e.currentTarget.value);
			}}
			class="input focus-visible:border-base-100 h-8 w-[150px] lg:w-[250px]"
		/>

		{#if isFiltered}
			<Button.Root
				class="btn btn-ghost h-8 px-2 lg:px-3"
				onclick={() => table.resetColumnFilters()}
			>
				Reset
				<X />
			</Button.Root>
		{/if}
	</div>
	<DataTableViewOptions {table} />
</div>
