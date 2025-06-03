<script lang="ts">
	import { listAccounts } from "$lib/services/accountService";
	import type { Account } from "$lib/models";
	import { onMount } from "svelte";
	import { toast } from "svelte-sonner";
	import AccountForm from "$lib/components/AccountForm.svelte";

	export let data;

	let accounts: Account[] = [];
	let loading = true;
	let error = false;

	const fetchAccounts = async () => {
		loading = true;
		const result = await listAccounts();
		if (result.isErr()) {
			error = true;
			toast.error(result.error.message);
		} else {
			accounts = result.value;
		}
		loading = false;
	};

	onMount(fetchAccounts);
</script>

<div class="container mx-auto p-4">
	<h1 class="mb-6 text-3xl font-bold">Accounts</h1>

	<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<h2 class="card-title">Create New Account</h2>
				<p class="text-base-content/70 mb-4">
					Create a new account for your finances. An account can represent a bank account,
					credit card, cash, or any other financial category.
				</p>
				<AccountForm form={data.form} />
			</div>
		</div>

		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<div class="mb-4 flex items-center justify-between">
					<h2 class="card-title">Your Accounts</h2>
					<button class="btn btn-sm btn-outline" on:click={fetchAccounts}>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
							/>
						</svg>
						Refresh
					</button>
				</div>
				{#if loading}
					<div class="flex justify-center p-4">
						<span class="loading loading-spinner loading-lg"></span>
					</div>
				{:else if error}
					<div class="alert alert-error">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-6 w-6 shrink-0 stroke-current"
							fill="none"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
						<span>Failed to load accounts. Please try again.</span>
					</div>
				{:else if accounts.length === 0}
					<div class="alert">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							class="stroke-info h-6 w-6 shrink-0"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
						<span>No accounts found. Create your first account!</span>
					</div>
				{:else}
					<div class="overflow-x-auto">
						<table class="table-zebra table">
							<thead>
								<tr>
									<th>Name</th>
									<th>Balance</th>
								</tr>
							</thead>
							<tbody>
								{#each accounts as account}
									<tr>
										<td>{account.name}</td>
										<td>
											<span
												class:text-success={account.balance > 0}
												class:text-error={account.balance < 0}
											>
												${account.balance}
											</span>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
