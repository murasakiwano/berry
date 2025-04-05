<script lang="ts">
	import { formatCurrency } from "$lib";

	type Props = { balance: number; accountName: string };

	let { balance, accountName = "Checking" }: Props = $props();

	let balanceColor: "text-success" | "text-warning" | "text-error" = $state("text-warning");

	if (balance > 0) {
		balanceColor = "text-success";
	} else if (balance < 0) {
		balanceColor = "text-error";
	}
</script>

<div class="stat">
	<div class="stat-title">Account Balance</div>
	<p class="stat-value {balanceColor}">{formatCurrency(balance)}</p>
	<div class="stat-desc">{accountName}</div>
</div>
