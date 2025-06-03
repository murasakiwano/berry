<script lang="ts">
	import { formatCurrency, parseCurrency } from "$lib";
	import BerryDatePicker from "$lib/components/BerryDatePicker.svelte";
	import { TxSchemaOptionalId } from "$lib/models.js";
	import SuperDebug, { superForm } from "sveltekit-superforms";
	import { zodClient } from "sveltekit-superforms/adapters";

	let { data } = $props();

	const { form, errors, constraints, message, enhance } = superForm(data.form, {
		validators: zodClient(TxSchemaOptionalId),
	});

	let isFormComplete = $derived(
		$form.title.trim() !== "" &&
			$form.amount !== undefined &&
			$form.destinationAccount.trim() !== "" &&
			$form.sourceAccount.trim() !== "" &&
			$form.postingDate !== undefined,
	);

	function handleKeydownAmount(event: KeyboardEvent) {
		const input = event.target as HTMLInputElement | null;
		if (event.key === "," && input?.value.includes(",")) {
			event.preventDefault();
		}

		if (event.key === "-" && input?.value.startsWith("-")) {
			event.preventDefault();
		}
	}

	function handleInputAmount(event: Event) {
		const input = event.target as HTMLInputElement;
		const originalCursorPos = input.selectionStart;
		const originalLength = input.value.length;

		// Store only valid characters
		const value = input.value.replace(/[^\d-.,]/g, "");

		// Set the filtered value back to the input
		input.value = value;

		// Calculate the new cursor position
		if (originalCursorPos !== null) {
			const lengthDifference = input.value.length - originalLength;
			const newPosition = Math.max(0, originalCursorPos + lengthDifference);
			input.setSelectionRange(newPosition, newPosition);
		}
	}

	function handleBlurAmount(event: Event) {
		const input = event.target as HTMLInputElement;

		const parsed = parseCurrency(input.value);

		if (isNaN(parsed)) {
			input.value = "";
		} else {
			input.value = formatCurrency(parsed);
			$form.amount = parsed;
		}
	}

	let stringAmount = $state($form.amount ? formatCurrency($form.amount) : "");
</script>

<SuperDebug data={$form} />

{#if $message}<h3 class="text-error">{$message}</h3>{/if}

<!-- Transaction creation form -->
<form method="POST" use:enhance class="my-4">
	<fieldset
		class="fieldset bg-base-200 border-base-300 rounded-box mx-auto w-xs gap-y-4 px-8 py-4"
	>
		<legend class="fieldset-legend">Adicionar Transação</legend>

		<div>
			<label for="title" class="fieldset-label">Descrição</label>
			<input
				type="text"
				class="input"
				name="title"
				autocomplete="off"
				placeholder="Descrição..."
				aria-invalid={$errors.title ? "true" : undefined}
				bind:value={$form.title}
				{...$constraints.title}
			/>
			{#if $errors.title}<span class="text-error">{$errors.title}</span>{/if}
		</div>

		<div>
			<label for="amount" class="fieldset-label">Quantia</label>
			<div class="flex gap-2">
				<input
					type="text"
					class="input flex-grow font-mono"
					name="amount"
					placeholder="R$ 1.234,56 (use vírgula para decimais)"
					aria-invalid={$errors.amount ? "true" : undefined}
					bind:value={stringAmount}
					oninput={handleInputAmount}
					onblur={handleBlurAmount}
					onkeydown={handleKeydownAmount}
				/>
				<button
					type="button"
					class="btn btn-outline btn-sm"
					onclick={() => {
						if (!$form.amount) return;
						$form.amount = -$form.amount;
						stringAmount = formatCurrency($form.amount);
					}}
					disabled={!$form.amount}
					title="Alternar entre valor positivo e negativo"
				>
					+/-
				</button>
			</div>
			<input type="hidden" name="amount" value={$form.amount} {...$constraints.amount} />
			{#if $errors.amount}<span class="text-error">{$errors.amount}</span>{/if}
		</div>

		<div>
			<label for="sourceAccount" class="fieldset-label">Conta de Origem</label>
			<input
				type="text"
				class="input"
				name="sourceAccount"
				placeholder="Conta de Origem"
				aria-invalid={$errors.sourceAccount ? "true" : undefined}
				bind:value={$form.sourceAccount}
				{...$constraints.sourceAccount}
			/>
			{#if $errors.sourceAccount}
				<span class="text-error">
					{$errors.sourceAccount}
				</span>
			{/if}
		</div>

		<div>
			<label for="destinationAccount" class="fieldset-label">Conta de Destino</label>
			<input
				type="text"
				class="input"
				name="destinationAccount"
				placeholder="Conta de Destino"
				aria-invalid={$errors.destinationAccount ? "true" : undefined}
				bind:value={$form.destinationAccount}
				{...$constraints.destinationAccount}
			/>
			{#if $errors.destinationAccount}
				<span class="text-error">{$errors.destinationAccount}</span>
			{/if}
		</div>

		<div>
			<BerryDatePicker bind:date={$form.postingDate} />
			{#if $errors.postingDate}<span class="text-error">{$errors.postingDate}</span>{/if}
		</div>

		<div>
			<label for="categories" class="fieldset-label">Categorias</label>
			<input
				type="text"
				class="input"
				name="categories"
				placeholder="comida,lazer,..."
				bind:value={$form.categories}
				aria-invalid={$errors.categories ? "true" : undefined}
				{...$constraints.categories}
			/>
			{#if $errors.categories}<span class="text-error">{$errors.categories}</span>{/if}
		</div>

		<button type="submit" class="btn btn-primary mt-2" disabled={!isFormComplete}>
			Enviar
		</button>
	</fieldset>
</form>
