<script>
	import BalanceCard from "$lib/components/BalanceCard.svelte";
	import TransactionsTable from "$lib/components/table/data-table.svelte";
	import { CalendarDateTime } from "@internationalized/date";
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
			destinationAccount: "Restaurantes",
			postingDate: new CalendarDateTime(2025, 4, 4, 15, 30),
			sourceAccount: "Cartão de Crédito",
			title: "Jantar",
			categories: ["comida"],
		},
		{
			id: "58bb3988-9683-4cf3-86c8-d2a8c0e07ed9",
			amount: 21,
			destinationAccount: "Supermercado",
			postingDate: new CalendarDateTime(2025, 4, 4, 19, 20),
			sourceAccount: "Cartão de Crédito",
			title: "Compras do mês",
			categories: ["casa"],
		},
		{
			id: "59efcdd8-72e1-476d-872a-bda5c542818c",
			amount: 2034,
			destinationAccount: "Lazer",
			postingDate: new CalendarDateTime(2025, 4, 4, 21, 15),
			sourceAccount: "Conta Corrente",
			title: "Filme",
			categories: ["cinema", "entretenimento"],
		},
	]}
/>
