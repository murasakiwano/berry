<script lang="ts">
	import ArrowsDownUp from "phosphor-svelte/lib/ArrowsDownUp";
	import { Button } from "bits-ui";

	let props: Button.RootProps = $props();
</script>

<Button.Root class="btn btn-ghost" {...props}>
	Data
	<ArrowsDownUp class="ml-2" />
</Button.Root>
