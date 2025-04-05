<script lang="ts">
	import DotsThree from "phosphor-svelte/lib/DotsThree";
	import { Button, DropdownMenu } from "bits-ui";

	let { id }: { id: string } = $props();
</script>

<DropdownMenu.Root>
	<div class="dropdown-center">
		<DropdownMenu.Trigger>
			{#snippet child({ props })}
				<Button.Root
					{...props}
					class="btn btn-circle bg-base-100 hover:bg-accent hover:text-accent-content relative size-8 h-10 w-10 border-none p-0"
				>
					<span class="sr-only">Open menu</span>
					<DotsThree class="size-7" />
				</Button.Root>
			{/snippet}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content
			class="menu bg-base-200 rounded-box z-1 cursor-default p-2 shadow-sm focus-visible:outline-hidden"
		>
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading class="px-2 py-1.5 text-sm font-bold"
					>Ações</DropdownMenu.GroupHeading
				>
				<DropdownMenu.Item
					onclick={() => navigator.clipboard.writeText(id)}
					class="hover:bg-accent hover:text-accent-content my-1 rounded px-2 py-1 font-medium focus-visible:outline-none"
				>
					Copiar ID da transação
				</DropdownMenu.Item>
			</DropdownMenu.Group>
			<DropdownMenu.Separator class="bg-neutral -mx-1 my-1 h-px" />
			<DropdownMenu.Item
				class="hover:bg-accent hover:text-accent-content my-1 rounded px-2 py-1 font-medium focus-visible:outline-none"
				>Ver conta de origem</DropdownMenu.Item
			>
			<DropdownMenu.Item
				class="hover:bg-accent hover:text-accent-content my-1 rounded px-2 py-1 font-medium focus-visible:outline-none"
				>Ver conta de destino</DropdownMenu.Item
			>
			<DropdownMenu.Item
				class="hover:bg-error hover:text-error-content my-1 rounded px-2 py-1 font-medium focus-visible:outline-none"
				>Deletar transação</DropdownMenu.Item
			>
		</DropdownMenu.Content>
	</div>
</DropdownMenu.Root>
