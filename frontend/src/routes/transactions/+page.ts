import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { TransactionSchema, type Transaction } from "$lib/models";
import { error } from "@sveltejs/kit";
import { ResultAsync } from "neverthrow";
import type { PageLoad } from "./$types";
import { parseBody, readResponseAsJson, checkIfResponseIsOk } from "$lib";

export const load: PageLoad = async ({ fetch }): Promise<{ transactions: Transaction[] }> => {
	const transactionsResult = await ResultAsync.fromPromise(
		fetch(`${PUBLIC_API_BASE_URL}/transactions`),
		(error) => {
			if (error instanceof Error) {
				console.error(error);
				return { status: 500, message: `failed to fetch transactions: ${error.message}` };
			}
			return { status: 500, message: `unknown error occurred: ${error}` };
		},
	)
		.andThen(checkIfResponseIsOk)
		.andThen(readResponseAsJson)
		.andThen((body) => parseBody<Transaction[]>(body, TransactionSchema.array()));

	if (transactionsResult.isErr()) {
		error(transactionsResult.error.status, transactionsResult.error.message);
	}

	return {
		transactions: transactionsResult.value,
	};
};
