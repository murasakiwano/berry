import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { TransactionSchema, type Transaction } from "$lib";
import { error } from "@sveltejs/kit";
import { err, errAsync, ok, okAsync, ResultAsync } from "neverthrow";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }): Promise<{ transactions: Transaction[] }> => {
	const transactionsResult = await ResultAsync.fromPromise(
		fetch(`${PUBLIC_API_BASE_URL}/transactions`),
		(error) => {
			if (error instanceof Error) {
				console.error(error);
				return { status: 500, body: `failed to fetch transactions: ${error.message}` };
			}
			return { status: 500, body: `unknown error occurred: ${error}` };
		},
	)
		.andThen((res) => {
			if (!res.ok) {
				return errAsync({ status: res.status, body: res.statusText });
			}

			return okAsync(res);
		})
		.andThen((res) =>
			ResultAsync.fromPromise(res.json(), (err: unknown) => ({
				status: 500,
				body: `failed to parse json: ${err}`,
			})),
		)
		.andThen((body: unknown) => {
			const parsedBody = TransactionSchema.array().safeParse(body);
			if (parsedBody.error) {
				return err({ status: 500, body: parsedBody.error.toString() });
			}

			return ok(parsedBody.data);
		});

	if (transactionsResult.isErr()) {
		error(transactionsResult.error.status, transactionsResult.error.body);
	}

	return {
		transactions: transactionsResult.value,
	};
};
