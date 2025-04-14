<script lang="ts">
	import { ArrowDownUp } from "@lucide/svelte";
	import { Button } from "bits-ui";

	let props: Button.RootProps = $props();
</script>

<Button.Root class="btn btn-ghost" {...props}>
	Data
	<ArrowDownUp class="ml-2" />
</Button.Root>
