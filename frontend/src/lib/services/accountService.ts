import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { checkIfResponseIsOk, parseBody, readResponseAsJson, type ErrResponse } from "$lib";
import { wrapApiError } from "$lib/errors";
import { accountSchema, type Account } from "$lib/models";
import { ResultAsync } from "neverthrow";
import { fail, message, superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { z } from "zod";

export const createAccount = (name: string): ResultAsync<Account, ErrResponse> => {
	return ResultAsync.fromPromise(
		fetch(`${PUBLIC_API_BASE_URL}/accounts`, {
			method: "POST",
			headers: {
				"Content-Type": "application/x-www-form-urlencoded",
			},
			body: `name=${encodeURIComponent(name)}`,
		}),
		wrapApiError,
	)
		.mapErr((error) => ({ status: error.status, message: error.body }))
		.andThen(checkIfResponseIsOk)
		.andThen(readResponseAsJson)
		.andThen((json) => parseBody<Account>(json, accountSchema));
};

export const listAccounts = (): ResultAsync<Account[], { status: number; message: string }> => {
	return ResultAsync.fromPromise(fetch(`${PUBLIC_API_BASE_URL}/accounts`), wrapApiError)
		.mapErr((error) => ({ status: error.status, message: error.body }))
		.andThen(checkIfResponseIsOk)
		.andThen(readResponseAsJson)
		.andThen((b) => parseBody<Account[]>(b, z.array(accountSchema)));
};

const schema = z.object({
	name: z.string().min(1, "Account name is required"),
});

export const createAccountAction = async ({ request }: { request: Request }) => {
	const form = await superValidate(request, zod(schema));

	if (!form.valid) {
		return fail(400, { form });
	}

	const res = await createAccount(form.data.name);
	if (res.isOk()) {
		return message(form, "Account created successfully!");
	}

	return message(form, res.error.message);
};
