import { parseDateTime, type DateValue } from "@internationalized/date";
import { z } from "zod";

// Custom zod schema for DateValue objects
const dateValueSchema = z.custom<DateValue>(
	(val) => val && typeof val === "object" && "calendar" in val,
	{ message: "Expected a valid DateValue object" },
);

export const TransactionSchema = z.object({
	id: z.string().uuid(),
	title: z.string().nonempty(),
	amount: z.number(),
	/** Name of the source account */
	sourceAccount: z.string().nonempty(),
	/** Name of the destination account */
	destinationAccount: z.string().nonempty(),
	postingDate: z.string().transform(parseDateTime),
	categories: z
		.string()
		.transform((cats) => cats.split(",").map((c) => c.trim()))
		.optional(),
});

/**
 * Used for form validation and sending a POST request (id is not needed to create a transaction)
 */
export const TxFormSchema = TransactionSchema.extend({
	id: TransactionSchema.shape.id.optional(),
	// Convert DateValue to string format expected by the API
	postingDate: dateValueSchema.transform((dateValue) => {
		// Format date as ISO string: YYYY-MM-DD
		return `${dateValue.year}-${dateValue.month.toString().padStart(2, "0")}-${dateValue.day.toString().padStart(2, "0")}`;
	}),
});

/**
 * Schema used for client-side validation in superforms
 */
export const TxSchemaOptionalId = z.object({
	id: z.string().uuid().optional(),
	title: z.string().nonempty(),
	amount: z.number(),
	sourceAccount: z.string().nonempty(),
	destinationAccount: z.string().nonempty(),
	// Keep postingDate as DateValue for client-side form validation
	postingDate: dateValueSchema,
	categories: z.string().optional(),
});

export type Transaction = z.infer<typeof TransactionSchema>;

export const AccountSchema = z.object({
	id: z.string().uuid(),
	name: z.string().nonempty(),
	balance: z
		.string()
		.transform(parseFloat)
		.refine((balance) => !isNaN(balance), { message: "Balance must be a valid number" }),
});

export type Account = z.infer<typeof AccountSchema>;
