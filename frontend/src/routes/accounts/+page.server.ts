import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { error, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import { AccountSchema } from "$lib/models";
import { fail, message, superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { z } from "zod";

const schema = z.object({
	name: z.string().min(1, "Account name is required"),
});

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_API_BASE_URL}/accounts`);

		if (!response.ok) {
			error(response.status, `Failed to load accounts: ${response.statusText}`);
		}

		const data = await response.json();
		const accounts = AccountSchema.array().parse(data);

		const form = await superValidate(zod(schema));

		return { accounts, form };
	} catch (err) {
		console.error("Failed to load accounts:", err);
		error(500, "Failed to load accounts");
	}
};

export const actions = {
	create: async ({ request, fetch }) => {
		const form = await superValidate(request, zod(schema));

		if (!form.valid) {
			return fail(400, { form });
		}

		try {
			const response = await fetch(`${PUBLIC_API_BASE_URL}/accounts`, {
				method: "POST",
				headers: { "Content-Type": "application/x-www-form-urlencoded" },
				body: `name=${encodeURIComponent(form.data.name)}`,
			});

			if (!response.ok) {
				const errorText = await response.text();
				return message(form, `Failed to create account: ${errorText}`, {
					status: 400,
				});
			}

			return message(form, "Account created successfully!");
		} catch (err) {
			console.error("Account creation failed:", err);
			return message(form, "Failed to create account", { status: 500 });
		}
	},

	delete: async ({ request, fetch }) => {
		const formData = await request.formData();
		const accountId = formData.get("accountId") as string;

		if (!accountId) {
			error(400, "Account ID is required");
		}

		const response = await fetch(`${PUBLIC_API_BASE_URL}/accounts/${accountId}`, {
			method: "delete",
		});

		if (!response.ok) {
			const errorText = await response.text();
			error(400, `Failed to delete account: ${errorText}`);
		}

		// Redirect to refresh the page and show updated data
		redirect(303, "/accounts");
	},
} satisfies Actions;
