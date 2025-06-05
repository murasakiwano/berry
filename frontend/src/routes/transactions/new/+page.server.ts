import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { TransactionSchema, TxSchemaOptionalId } from "$lib/models";
import type { DateValue } from "@internationalized/date";
import { fail, message, superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
	return { form: await superValidate(zod(TxSchemaOptionalId)) };
};

export const actions = {
	default: async ({ request }) => {
		const formData = await request.formData();
		const form = await superValidate(formData, zod(TxSchemaOptionalId));

		if (!form.valid) {
			console.error({ errors: form.errors });
			return fail(400, { form });
		}

		try {
			// Convert form data for API
			const apiFormData = new FormData();

			// Add all form fields from validated form data
			for (const [key, value] of Object.entries(form.data)) {
				if (key !== "postingDate") {
					apiFormData.append(key, String(value));
				}
			}

			// Format the date value - we need to handle DateValue object from the form
			const date = form.data.postingDate;
			if (date && typeof date === "object" && "calendar" in date) {
				const dateValue = date as DateValue;
				const formattedDate = `${dateValue.year}-${dateValue.month.toString().padStart(2, "0")}-${dateValue.day.toString().padStart(2, "0")}`;
				apiFormData.append("postingDate", formattedDate);
			}

			console.debug("Got form data to send", {
				formData: Object.fromEntries(apiFormData.entries()),
			});

			const response = await fetch(`${PUBLIC_API_BASE_URL}/transactions`, {
				method: "POST",
				body: apiFormData,
			});

			if (!response.ok) {
				const errorText = await response.text();
				return message(form, `Failed to create transaction: ${errorText}`, { status: 400 });
			}

			const data = await response.json();
			const tx = TransactionSchema.parse(data);

			return message(form, `Transaction created successfully: ${tx.id}`);
		} catch (err) {
			console.error("Failed to create transaction:", err);
			return message(form, "Failed to create transaction", { status: 500 });
		}
	},
} satisfies Actions;
