<script>
	import BalanceCard from "$lib/components/BalanceCard.svelte";
	import TransactionsTable from "$lib/components/table/TransactionsTable.svelte";
</script>

<div class="mb-4 flex justify-center">
	<div class="stats shadow">
		<BalanceCard accountName="Checking" balance={-42} />
		<BalanceCard accountName="Checking" balance={0} />
		<BalanceCard accountName="Checking" balance={+42} />
	</div>
</div>

<TransactionsTable
	transactions={[
		{
			id: "1f651621-627d-4ac0-96c2-20a4837f128f",
			amount: 42,
			destinationAccount: "Destination",
			postingDate: new Date("2025-04-03"),
			sourceAccount: "Source",
			title: "Test transaction",
		},
	]}
/>
