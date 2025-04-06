<script lang="ts">
	import { type Transaction } from "$lib";
	import { createSvelteTable } from "$lib/table";
	import FlexRender from "$lib/table/flex-render.svelte";
	import {
		getCoreRowModel,
		getFilteredRowModel,
		getPaginationRowModel,
		getSortedRowModel,
		type ColumnFiltersState,
		type PaginationState,
		type RowSelectionState,
		type SortingState,
		type VisibilityState,
	} from "@tanstack/table-core";
	import { Button } from "bits-ui";
	import { columnDefs } from "./columns";
	import DataTableViewOptions from "./data-table-view-options.svelte";
	import DataTableToolbar from "./data-table-toolbar.svelte";
	import DataTablePagination from "./data-table-pagination.svelte";

	type Props = { transactions: Transaction[] };
	let { transactions }: Props = $props();

	let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
	let sorting = $state<SortingState>([]);
	let columnFilters = $state<ColumnFiltersState>([]);
	let columnVisibility = $state<VisibilityState>({ categories: false });
	let rowSelection = $state<RowSelectionState>({});

	const table = createSvelteTable({
		get data() {
			return transactions;
		},
		columns: columnDefs,
		getCoreRowModel: getCoreRowModel(),
		getPaginationRowModel: getPaginationRowModel(),
		getSortedRowModel: getSortedRowModel(),
		getFilteredRowModel: getFilteredRowModel(),
		onSortingChange: (updater) => {
			if (typeof updater === "function") {
				sorting = updater(sorting);
			} else {
				sorting = updater;
			}
		},
		onPaginationChange: (updater) => {
			if (typeof updater === "function") {
				pagination = updater(pagination);
			} else {
				pagination = updater;
			}
		},
		onColumnFiltersChange: (updater) => {
			if (typeof updater === "function") {
				columnFilters = updater(columnFilters);
			} else {
				columnFilters = updater;
			}
		},
		onColumnVisibilityChange: (updater) => {
			if (typeof updater === "function") {
				columnVisibility = updater(columnVisibility);
			} else {
				columnVisibility = updater;
			}
		},
		onRowSelectionChange: (updater) => {
			if (typeof updater === "function") {
				rowSelection = updater(rowSelection);
			} else {
				rowSelection = updater;
			}
		},
		state: {
			get pagination() {
				return pagination;
			},
			get sorting() {
				return sorting;
			},
			get columnFilters() {
				return columnFilters;
			},
			get columnVisibility() {
				return columnVisibility;
			},
			get rowSelection() {
				return rowSelection;
			},
		},
	});
</script>

<div class="mx-auto max-w-[90%]">
	<DataTableToolbar {table} />
	<div class="border-base-content/5 bg-base-100 w-full overflow-x-auto rounded-md border">
		<table class="table border-collapse">
			<thead class="bg-neutral text-neutral-content">
				{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
					<tr>
						{#each headerGroup.headers.filter( (header) => header.column.getIsVisible(), ) as header (header.id)}
							<th>
								{#if !header.isPlaceholder}
									<FlexRender
										content={header.column.columnDef.header}
										context={header.getContext()}
									/>
								{/if}
							</th>
						{/each}
					</tr>
				{/each}
			</thead>
			<tbody class="border-base-content/5 border-t">
				{#each table.getRowModel().rows as row (row.id)}
					<tr class="not-last:border-base-content/5 not-last:border-b">
						{#each row
							.getAllCells()
							.filter((cell) => cell.column.getIsVisible()) as cell (cell.id)}
							<td>
								<FlexRender
									content={cell.column.columnDef.cell}
									context={cell.getContext()}
								/>
							</td>
						{/each}
					</tr>
				{:else}
					<tr>
						<td colspan={columnDefs.length} class="h-24 text-center">No results.</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
	<DataTablePagination {table} />
</div>
