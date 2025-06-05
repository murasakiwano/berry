<!-- routes/accounts/+page.svelte -->
<script lang="ts">
	import AccountForm from "$lib/components/AccountForm.svelte";
	import type { PageData } from "./$types";

	let { data }: { data: PageData } = $props();
</script>

<div class="container mx-auto p-4">
	<h1 class="mb-6 text-3xl font-bold">Accounts</h1>

	<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
		<AccountForm form={data.form} />

		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<h2 class="card-title">Your Accounts ({data.accounts.length})</h2>

				{#each data.accounts as account}
					<div class="flex justify-between border-b py-2">
						<span>{account.name}</span>
						<span
							class:text-success={account.balance > 0}
							class:text-error={account.balance < 0}
						>
							${account.balance}
						</span>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>
