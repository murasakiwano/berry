import { parseDateTime } from "@internationalized/date";
import { z } from "zod";

export function formatCurrency(value: number): string {
	return Intl.NumberFormat("pt-BR", {
		currency: "BRL",
		style: "currency",
	}).format(value);
}

export function capitalize(s: string): string {
	if (s.length > 0) {
		const firstLetter = s.slice(0, 1);

		return firstLetter.toUpperCase() + s.substring(1);
	}

	return s;
}

const txSchemaSnakeCase = z.object({
	id: z.string().uuid(),
	title: z.string().nonempty(),
	amount: z.number(),
	/** Name of the source account */
	source_account: z.string().nonempty(),
	/** Name of the destination account */
	destination_account: z.string().nonempty(),
	postingDate: z
		.string()
		.datetime()
		.transform((value) => parseDateTime(value)),
	categories: z.optional(z.array(z.string())),
});

export const TransactionSchema = txSchemaSnakeCase.transform((arg) => ({
	id: arg.id,
	title: arg.title,
	amount: arg.amount,
	sourceAccount: arg.source_account,
	destinationAccount: arg.destination_account,
	postingDate: arg.postingDate,
	categories: arg.categories,
}));

export type Transaction = z.infer<typeof TransactionSchema>;

export const accountSchema = z.object({
	id: z.string().uuid(),
	name: z.string().nonempty(),
	balance: z.number(),
});

export type Account = z.infer<typeof accountSchema>;
