<script lang="ts">
	import { formatCurrency } from "$lib";

	type Props = { balance: number };

	let { balance }: Props = $props();

	const balanceColor = balance >= 0 ? "text-success" : "text-error";
</script>

<div class="card bg-base-100 mb-6 shadow-xl">
	<div class="card-body">
		<div class="flex items-center justify-between">
			<h3 class="card-title">Account Balance</h3>
			<div class="badge badge-primary">Checking</div>
		</div>
		<p class="font-mono text-3xl font-bold {balanceColor}">{formatCurrency(balance)}</p>
	</div>
</div>
