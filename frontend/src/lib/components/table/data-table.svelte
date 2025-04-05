<script lang="ts">
	import { type Transaction } from "$lib";
	import { createSvelteTable, renderComponent, renderSnippet } from "$lib/table";
	import FlexRender from "$lib/table/flex-render.svelte";
	import { DateFormatter, getLocalTimeZone } from "@internationalized/date";
	import {
		createColumnHelper,
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
	import { Button, DropdownMenu } from "bits-ui";
	import Check from "phosphor-svelte/lib/Check";
	import { createRawSnippet } from "svelte";
	import Checkbox from "../Checkbox.svelte";
	import TableActions from "./data-table-actions.svelte";
	import DataTableDateButton from "./data-table-date-button.svelte";

	type Props = { transactions: Transaction[] };
	let { transactions }: Props = $props();

	const colHelp = createColumnHelper<Transaction>();

	const columnDefs = [
		colHelp.display({
			id: "select",
			header: ({ table }) =>
				renderComponent(Checkbox, {
					checked: table.getIsAllPageRowsSelected(),
					indeterminate:
						table.getIsSomePageRowsSelected() && !table.getIsAllPageRowsSelected(),
					onCheckedChange: (value) => table.toggleAllPageRowsSelected(value),
					"aria-label": "Select all",
				}),
			cell: ({ row }) =>
				renderComponent(Checkbox, {
					checked: row.getIsSelected(),
					indeterminate: row.getIsSomeSelected() && !row.getIsAllSubRowsSelected(),
					onCheckedChange: (value) => row.toggleExpanded(value),
					"aria-label": "Select row",
				}),
		}),
		colHelp.accessor("postingDate", {
			header: ({ column }) =>
				renderComponent(DataTableDateButton, {
					onclick: () => column.toggleSorting(column.getIsSorted() === "asc"),
				}),
			cell: ({ getValue }) => {
				const formatter = new DateFormatter("pt-BR", {
					dateStyle: "full",
					timeStyle: "short",
				});

				return formatter.format(getValue().toDate(getLocalTimeZone()));
			},
		}),
		colHelp.accessor("title", { header: "Descrição" }),
		colHelp.accessor("sourceAccount", { header: "Conta de Origem" }),
		colHelp.accessor("destinationAccount", { header: "Conta de Destino" }),
		colHelp.accessor("amount", {
			header: () => {
				const amountHeaderSnippet = createRawSnippet(() => ({
					render: () => `<div class="text-right">Quantia</div>`,
				}));
				return renderSnippet(amountHeaderSnippet, "");
			},
			cell: ({ row }) => {
				const formatter = new Intl.NumberFormat("pt-BF", {
					style: "currency",
					currency: "BRL",
				});

				const amountCellSnippet = createRawSnippet<[string]>((getAmount) => {
					const amount = getAmount();
					return {
						render: () => `<div class="text-right font-medium">${amount}</div>`,
					};
				});

				return renderSnippet(amountCellSnippet, formatter.format(row.getValue("amount")));
			},
		}),
		colHelp.accessor("categories", { header: "Categorias" }),
		colHelp.display({
			id: "actions",
			cell: ({ row }) => renderComponent(TableActions, { id: row.original.id }),
		}),
	];

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
	<div class="flex items-center py-4">
		<input
			type="text"
			placeholder="Filtrar conta de origem..."
			value={(table.getColumn("sourceAccount")?.getFilterValue() as string) ?? ""}
			onchange={(e) => {
				table.getColumn("sourceAccount")?.setFilterValue(e.currentTarget.value);
			}}
			oninput={(e) => {
				table.getColumn("sourceAccount")?.setFilterValue(e.currentTarget.value);
			}}
			class="input max-w-sm"
		/>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Button.Root {...props} class="btn btn-outline ml-auto">Columns</Button.Root>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Portal>
				<DropdownMenu.Content align="end" side="top" class="menu bg-base-200">
					{#each table
						.getAllColumns()
						.filter((col) => col.getCanHide()) as column (column)}
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
							{column.id}
						</DropdownMenu.CheckboxItem>
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	</div>
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
	<div class="flex items-center justify-end space-x-2 py-4">
		<Button.Root
			class="btn btn-outline btn-sm"
			onclick={() => table.previousPage()}
			disabled={!table.getCanPreviousPage()}
		>
			Previous
		</Button.Root>
		<Button.Root
			class="btn btn-outline btn-sm"
			onclick={() => table.nextPage()}
			disabled={!table.getCanNextPage()}
		>
			Next
		</Button.Root>
	</div>
</div>
