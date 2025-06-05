<script lang="ts">
	import { superForm, type SuperValidated } from "sveltekit-superforms";
	import { z } from "zod";
	import { zodClient } from "sveltekit-superforms/adapters";
	import { toast, Toaster } from "svelte-sonner";

	const accountSchema = z.object({ name: z.string().min(1, "Account name is required") });

	type Props = { form: SuperValidated<{ name: string }> };
	let { form: serverForm }: Props = $props();

	const { form, errors, constraints, enhance, reset } = superForm(serverForm.data, {
		validators: zodClient(accountSchema),
		onUpdated: ({ form }) => {
			if (form.valid) {
				toast.success(form.message, {
					description: "Account created successfully",
					duration: 4000,
				});
				reset();
			} else {
				toast.error(form.message);
			}
		},
	});
</script>

<form method="POST" use:enhance class="my-4">
	<fieldset
		class="fieldset bg-base-200 border-base-300 rounded-box mx-auto w-xs gap-y-4 px-8 py-4"
	>
		<div>
			<label for="name" class="fieldset-label">Nome da Conta</label>
			<input
				type="text"
				class="input"
				name="name"
				autocomplete="off"
				placeholder="Nome..."
				aria-invalid={$errors.name ? "true" : undefined}
				bind:value={$form.name}
				{...$constraints.name}
			/>
			{#if $errors.name}<span class="text-error">{$errors.name}</span>{/if}
		</div>

		<button type="submit" class="btn btn-primary mt-2" disabled={$form.name.length === 0}>
			Enviar
		</button>
	</fieldset>
</form>
