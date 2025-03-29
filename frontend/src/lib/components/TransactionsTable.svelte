<script lang="ts">
	import { type Transaction } from "$lib";
	import { createSvelteTable } from "$lib/table";
	import FlexRender from "$lib/table/flex-render.svelte";
	import { createColumnHelper, getCoreRowModel } from "@tanstack/table-core";
	import { format } from "date-fns";
	import { ptBR } from "date-fns/locale";

	type Props = { transactions: Transaction[] };
	let { transactions }: Props = $props();

	const colHelp = createColumnHelper<Transaction>();

	const columnDefs = [
		colHelp.accessor("postingDate", {
			header: "Data",
			cell: (props) => format(props.getValue(), "PPP (EEEE)", { locale: ptBR })
		}),
		colHelp.accessor("title", { header: "Descrição" }),
		colHelp.accessor("amount", { header: "Quantia" }),
		colHelp.accessor("sourceAccount", { header: "Conta de Origem" }),
		colHelp.accessor("destinationAccount", { header: "Conta de Destino" })
	];

	const table = createSvelteTable({
		data: transactions,
		columns: columnDefs,
		getCoreRowModel: getCoreRowModel()
	});
</script>

<div class="rounded-box border-base-content/5 bg-base-200 overflow-x-auto border">
	<table class="table">
		<thead>
			<tr>
				{#each table.getHeaderGroups() as headerGroup}
					{#each headerGroup.headers as header}
						<th>{header.column.columnDef.header}</th>
					{/each}
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each table.getRowModel().rows as row}
				<tr class="hover:bg-base-300">
					{#each row.getAllCells() as cell}
						<td><FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} /></td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>
