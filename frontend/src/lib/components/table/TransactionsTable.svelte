<script lang="ts">
	import { type Transaction } from "$lib";
	import { createSvelteTable, renderComponent, renderSnippet } from "$lib/table";
	import FlexRender from "$lib/table/flex-render.svelte";
	import { createColumnHelper, getCoreRowModel } from "@tanstack/table-core";
	import { format } from "date-fns";
	import { ptBR } from "date-fns/locale";
	import { createRawSnippet } from "svelte";
	import TableActions from "./TableActions.svelte";

	type Props = { transactions: Transaction[] };
	let { transactions }: Props = $props();

	const colHelp = createColumnHelper<Transaction>();

	const columnDefs = [
		colHelp.accessor("postingDate", {
			header: "Data",
			cell: (props) => format(props.getValue(), "PPP (EEEE)", { locale: ptBR }),
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
		colHelp.display({
			id: "actions",
			cell: ({ row }) => {
				return renderComponent(TableActions, { id: row.original.id });
			},
		}),
	];

	const table = createSvelteTable({
		data: transactions,
		columns: columnDefs,
		getCoreRowModel: getCoreRowModel(),
	});
</script>

<div class="rounded-box border-base-content/5 bg-base-100 overflow-x-auto border">
	<table class="table">
		<thead class="bg-neutral text-neutral-content">
			{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<tr>
					{#each headerGroup.headers as header (header.id)}
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
		<tbody>
			{#each table.getRowModel().rows as row (row.id)}
				<tr class="hover:bg-base-300 transition ease-in-out">
					{#each row.getAllCells() as cell (cell.id)}
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
