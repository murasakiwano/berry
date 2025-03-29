<script lang="ts">
	import BalanceCard from "$lib/components/BalanceCard.svelte";
	import ThemeController from "$lib/components/ThemeController.svelte";
	import TransactionsTable from "$lib/components/TransactionsTable.svelte";
	import type { PageProps } from "./$types";

	const { data }: PageProps = $props();
</script>

<header class="mb-4 flex justify-between">
	<h2 class="text-primary mb-6 text-2xl font-bold">Berry</h2>
	<ThemeController />
</header>

<BalanceCard balance={42240.8} />

<TransactionsTable transactions={data.transactions} />
