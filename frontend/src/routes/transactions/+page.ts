import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { error } from "@sveltejs/kit";
import { err, ok, ResultAsync } from "neverthrow";
import { TransactionSchema, type Transaction } from "$lib";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }): Promise<{ transactions: Transaction[] }> => {
	const response = await ResultAsync.fromPromise(
		fetch(`${PUBLIC_API_BASE_URL}/transactions`),
		(err) => {
			if (err instanceof Error) {
				console.error(err);
				return new Error(`failed to fetch transactions: ${err.message}`);
			}
			throw err;
		}
	);

	if (response.isErr()) {
		error(500, response.error.message);
	}

	const res = response.value;

	if (!res.ok) {
		error(res.status, res.statusText);
	}

	const body = (
		await ResultAsync.fromPromise(res.json(), (e) => new Error(`failed to parse json: ${e}`))
	).andThen((b: unknown) => {
		const parseResult = TransactionSchema.array().safeParse(b);
		if (parseResult.error) {
			return err(parseResult.error);
		}

		const txs = parseResult.data;

		return ok(txs);
	});

	if (body.isErr()) {
		console.error(body.error);
		error(500, body.error.message);
	}

	return {
		transactions: body.value
	};
};
