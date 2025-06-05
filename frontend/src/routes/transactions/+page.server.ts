import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { TransactionSchema } from "$lib/models";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_API_BASE_URL}/transactions`);

		if (!response.ok) {
			error(response.status, `Failed to load transactions: ${response.statusText}`);
		}

		const data = await response.json();
		const transactions = TransactionSchema.array().parse(data);

		return { transactions };
	} catch (err) {
		console.error("Failed to load accounts:", err);
		error(500, "Failed to load accounts");
	}
};
