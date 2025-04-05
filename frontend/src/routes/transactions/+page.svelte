<script lang="ts">
	import TransactionsTable from "$lib/components/table/TransactionsTable.svelte";
	import type { PageProps } from "./$types";

	const { data }: PageProps = $props();
</script>

<div class="flex items-center justify-center">
	<TransactionsTable transactions={data.transactions} />
</div>
