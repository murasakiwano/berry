import { createAccountAction } from "$lib/services/accountService";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { z } from "zod";

const schema = z.object({
	name: z.string().min(1, "Account name is required"),
});

export const load = async () => {
	// Server API to load form
	const form = await superValidate(zod(schema));
	return { form };
};

export const actions = {
	default: createAccountAction,
};
