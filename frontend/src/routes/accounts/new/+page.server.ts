import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { error } from "@sveltejs/kit";
import { fail, message, superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { z } from "zod";
import type { PageServerLoad } from "./$types";
import { AccountSchema } from "$lib/models";

const schema = z.object({
	name: z.string().min(1, "Account name is required"),
});

export const load: PageServerLoad = async () => {
	// Server API to load form
	const form = await superValidate(zod(schema));
	return { form };
};

export const actions = {
	default: async ({ request }) => {
		const form = await superValidate(request, zod(schema));

		if (!form.valid) {
			return fail(400, { form });
		}

		try {
			// Server API to create account
			const response = await fetch(`${PUBLIC_API_BASE_URL}/accounts`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(form.data),
			});

			if (!response.ok) {
				throw error(response.status, "Failed to create account");
			}

			const data = await response.json();
			const account = AccountSchema.parse(data);

			return message(form, `Successfully created account: ${account.id}`);
		} catch (err) {
			console.error("Failed to create account:", err);
			return message(form, "Failed to create account", { status: 500 });
		}
	},
};
