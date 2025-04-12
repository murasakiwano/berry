import type { Transaction } from "$lib/models";
import { renderComponent, renderSnippet } from "$lib/table";
import { DateFormatter, getLocalTimeZone } from "@internationalized/date";
import { createColumnHelper } from "@tanstack/table-core";
import { createRawSnippet } from "svelte";
import Checkbox from "../Checkbox.svelte";
import DataTableActions from "./data-table-actions.svelte";
import DataTableDateButton from "./data-table-date-button.svelte";

export const POSTING_DATE_COL_ID = "data";
export const TITLE_COL_ID = "descrição";
export const SOURCE_ACCOUNT_COL_ID = "conta de origem";
export const DESTINATION_ACCOUNT_COL_ID = "conta de destino";
export const AMOUNT_COL_ID = "quantia";
export const CATEGORIES_COL_ID = "categorias";
export const ACTIONS_COL_ID = "ações";

const colHelp = createColumnHelper<Transaction>();
export const columnDefs = [
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
				onCheckedChange: (value) => row.toggleSelected(value),
				"aria-label": "Select row",
			}),
	}),
	colHelp.accessor("postingDate", {
		id: POSTING_DATE_COL_ID,
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
	colHelp.accessor("title", { header: "Descrição", id: TITLE_COL_ID }),
	colHelp.accessor("sourceAccount", {
		header: "Conta de Origem",
		id: SOURCE_ACCOUNT_COL_ID,
	}),
	colHelp.accessor("destinationAccount", {
		header: "Conta de Destino",
		id: DESTINATION_ACCOUNT_COL_ID,
	}),
	colHelp.accessor("amount", {
		id: AMOUNT_COL_ID,
		header: () => {
			const amountHeaderSnippet = createRawSnippet(() => ({
				render: () => `<div class="text-right">Quantia</div>`,
			}));
			return renderSnippet(amountHeaderSnippet, "");
		},
		cell: ({ getValue }) => {
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

			return renderSnippet(amountCellSnippet, formatter.format(getValue()));
		},
	}),
	colHelp.accessor("categories", { header: "Categorias", id: CATEGORIES_COL_ID }),
	colHelp.display({
		id: ACTIONS_COL_ID,
		cell: ({ row }) => renderComponent(DataTableActions, { id: row.original.id }),
	}),
];
