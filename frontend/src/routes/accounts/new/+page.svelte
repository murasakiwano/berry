<script lang="ts">
	import { goto } from "$app/navigation";
	import AccountForm from "$lib/components/AccountForm.svelte";
	import { Button } from "bits-ui";

	export let data;
</script>

<div class="container mx-auto p-4">
	<div class="mb-6 flex items-center">
		<Button.Root class="btn btn-ghost mr-4" onclick={() => goto("/accounts")}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 w-6"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M15 19l-7-7 7-7"
				/>
			</svg>
		</Button.Root>
		<h1 class="text-3xl font-bold">Create New Account</h1>
	</div>

	<div class="card bg-base-100 mx-auto max-w-md shadow-xl">
		<div class="card-body">
			<h2 class="card-title">New Account</h2>
			<p class="text-base-content/70 mb-4">
				Create a new account for your finances. An account can represent a bank account,
				credit card, cash, or any other financial category.
			</p>
			<AccountForm form={data.form} />
		</div>
	</div>
</div>
