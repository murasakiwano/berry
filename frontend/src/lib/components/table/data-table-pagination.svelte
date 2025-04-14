<script lang="ts" generics="TData">
	import type { Table } from "@tanstack/table-core";
	import { Button, Select } from "bits-ui";
	import {
		ChevronsLeft,
		ChevronLeft,
		ChevronRight,
		ChevronsRight,
		ChevronsDownUp,
		ChevronsUp,
		ChevronsDown,
	} from "@lucide/svelte";

	let { table }: { table: Table<TData> } = $props();

	const pageSizes: { value: string; label: string }[] = [10, 20, 30, 40, 50].map((n) => ({
		value: n.toString(),
		label: n.toString(),
	}));
	const selectedLabel = $derived(table.getState().pagination.pageSize);
</script>

<div class="flex items-center justify-between p-2">
	<div class="text-muted-foreground flex-1 text-sm">
		{table.getFilteredSelectedRowModel().rows.length} of
		{table.getFilteredRowModel().rows.length} row(s) selected.
	</div>
	<div class="flex items-center space-x-6 lg:space-x-8">
		<div class="flex items-center space-x-2">
			<p class="text-sm font-medium">Rows per page</p>
			<Select.Root
				type="single"
				onValueChange={(v) => table.setPageSize(Number(v))}
				items={pageSizes}
			>
				<Select.Trigger
					class="btn btn-outline hover:bg-base-100 data-placeholder:text-base-content-300/50 hover:border-base-content inline-flex h-8 w-[70px] items-center rounded-md px-[11px] font-mono text-sm transition-colors select-none"
					aria-label="Select a page size"
				>
					{selectedLabel}
					<ChevronsDownUp class="text-neutral-content ml-auto size-6" />
				</Select.Trigger>
				<Select.Portal>
					<Select.Content
						class="border-neutral bg-base-100 shadow-popover data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 max-h-[var(--bits-select-content-available-height)] w-[var(--bits-select-anchor-width)] min-w-[var(--bits-select-anchor-width)] rounded-xl border p-1 font-mono outline-hidden select-none data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1"
						sideOffset={10}
					>
						<Select.ScrollUpButton class="flex w-full items-center justify-center">
							<ChevronsUp class="size-3" />
						</Select.ScrollUpButton>
						<Select.Viewport class="p-1">
							{#each pageSizes as { value } (value)}
								<Select.Item
									class="data-highlighted:bg-neutral flex h-10 w-full items-center justify-center rounded-md px-1.5 py-3 text-sm outline-hidden select-none"
									{value}
									label={value.toString()}
								>
									{value}
								</Select.Item>
							{/each}
						</Select.Viewport>
						<Select.ScrollDownButton class="flex w-full items-center justify-center">
							<ChevronsDown class="size-3" />
						</Select.ScrollDownButton>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<div class="flex w-[100px] items-center justify-center text-sm font-medium">
			Page {table.getState().pagination.pageIndex + 1} of
			{table.getPageCount()}
		</div>
		<div class="flex items-center space-x-2">
			<Button.Root
				class="btn btn-outline hidden size-8 p-0 lg:flex"
				onclick={() => table.setPageIndex(0)}
				disabled={!table.getCanPreviousPage()}
			>
				<span class="sr-only">Go to first page</span>
				<ChevronsLeft />
			</Button.Root>
			<Button.Root
				class="btn btn-outline size-8 p-0"
				onclick={() => table.previousPage()}
				disabled={!table.getCanPreviousPage()}
			>
				<span class="sr-only">Go to previous page</span>
				<ChevronLeft />
			</Button.Root>
			<Button.Root
				class="btn btn-outline size-8 p-0"
				onclick={() => table.nextPage()}
				disabled={!table.getCanNextPage()}
			>
				<span class="sr-only">Go to next page</span>
				<ChevronRight />
			</Button.Root>
			<Button.Root
				class="btn btn-outline hidden size-8 p-0 lg:flex"
				onclick={() => table.setPageIndex(table.getPageCount() - 1)}
				disabled={!table.getCanNextPage()}
			>
				<span class="sr-only">Go to last page</span>
				<ChevronsRight />
			</Button.Root>
		</div>
	</div>
</div>
