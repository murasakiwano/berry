<script lang="ts">
	import AccountForm from "$lib/components/AccountForm.svelte";
	import { enhance } from "$app/forms";
	import { Trash2 } from "@lucide/svelte";
	import type { PageData } from "./$types";
	import { formatCurrency } from "$lib";

	let { data }: { data: PageData } = $props();

	let deletingAccounts = $state(new Set<string>());

	function handleDeleteSubmit(accountId: string) {
		return ({ cancel }: { cancel: () => void }) => {
			if (
				!confirm(
					"Are you sure you want to delete this account? This action cannot be undone.",
				)
			) {
				cancel();
				return;
			}

			deletingAccounts.add(accountId);

			return async ({ update }: { update: () => Promise<void> }) => {
				await update();
				deletingAccounts.delete(accountId);
			};
		};
	}
</script>

<div class="container mx-auto p-4">
	<h1 class="mb-6 text-3xl font-bold">Accounts</h1>

	<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
		<AccountForm form={data.form} />

		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<h2 class="card-title">Your Accounts ({data.accounts.length})</h2>

				{#if data.accounts.length === 0}
					<div class="py-8 text-center">
						<p class="text-base-content/70">
							No accounts yet. Create your first account!
						</p>
					</div>
				{:else}
					<div class="overflow-hidden">
						{#each data.accounts as account (account.id)}
							<div
								class="group border-primary hover:bg-base-200/30 flex items-center justify-between border-b p-3 transition-colors"
							>
								<div class="flex-1">
									<span class="font-medium">{account.name}</span>
								</div>
								<div class="flex items-center gap-4">
									<span
										class="min-w-[5rem] text-right font-mono font-semibold"
										class:text-success={account.balance > 0}
										class:text-error={account.balance < 0}
										class:text-base-content={account.balance === 0}
									>
										{formatCurrency(account.balance)}
									</span>

									<!-- Delete button - hidden by default, shown on group hover -->
									<form
										method="POST"
										action="?/delete"
										use:enhance={handleDeleteSubmit(account.id)}
										class="opacity-0 transition-opacity duration-200 group-hover:opacity-100"
									>
										<input type="hidden" name="accountId" value={account.id} />
										<button
											type="submit"
											class="btn btn-ghost btn-sm btn-circle text-error hover:bg-error hover:text-error-content"
											disabled={deletingAccounts.has(account.id)}
											title="Delete account"
										>
											{#if deletingAccounts.has(account.id)}
												<span class="loading loading-spinner loading-xs"
												></span>
											{:else}
												<Trash2 class="h-4 w-4" />
											{/if}
										</button>
									</form>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
