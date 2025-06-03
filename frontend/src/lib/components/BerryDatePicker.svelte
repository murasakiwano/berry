<script lang="ts">
	import { DatePicker } from "bits-ui";
	import Calendar from "@lucide/svelte/icons/calendar";
	import ChevronLeft from "@lucide/svelte/icons/chevron-left";
	import ChevronRight from "@lucide/svelte/icons/chevron-right";
	import type { DateValue } from "@internationalized/date";
	import type { Snippet } from "svelte";

	let { date = $bindable(), extraInfo }: { date: DateValue; extraInfo?: Snippet } = $props();
</script>

<DatePicker.Root weekdayFormat="short" fixedWeeks={true} bind:value={date} locale="pt-BR">
	<DatePicker.Label class="fieldset-label">Data</DatePicker.Label>

	<DatePicker.Input class="input">
		{#snippet children({ segments })}
			{#each segments as { part, value }}
				<div class="inline-block select-none">
					{#if part === "literal"}
						<DatePicker.Segment {part} class="text-base-content/70 p-1">
							{value}
						</DatePicker.Segment>
					{:else}
						<DatePicker.Segment
							{part}
							class="hover:bg-base-300 focus:bg-base-300 aria-[valuetext=Empty]:text-base-content/70 rounded-sm px-1 py-1 focus-visible:ring-0! focus-visible:ring-offset-0!"
						>
							{value}
						</DatePicker.Segment>
					{/if}
				</div>
			{/each}
			<DatePicker.Trigger class="btn btn-circle btn-ghost ml-auto opacity-60 transition-all">
				<Calendar class="size-6" />
			</DatePicker.Trigger>
		{/snippet}
	</DatePicker.Input>

	<DatePicker.Content sideOffset={6} class="z-50">
		<DatePicker.Calendar
			class="border-base-300 bg-base-200 rounded-[15px] border p-[22px] shadow-lg"
		>
			{#snippet children({ months, weekdays })}
				<DatePicker.Header class="flex items-center justify-between">
					<DatePicker.PrevButton
						class="btn btn-circle btn-ghost transition-all active:scale-[0.98]"
					>
						<ChevronLeft class="size-6" />
					</DatePicker.PrevButton>
					<DatePicker.Heading class="text-[15px] font-medium" />
					<DatePicker.NextButton
						class="btn btn-circle btn-ghost transition-all active:scale-[0.98]"
					>
						<ChevronRight class="size-6" />
					</DatePicker.NextButton>
				</DatePicker.Header>
				<div class="flex flex-col space-y-4 pt-4 sm:flex-row sm:space-y-0 sm:space-x-4">
					{#each months as month}
						<DatePicker.Grid class="w-full border-collapse space-y-1 select-none">
							<DatePicker.GridHead>
								<DatePicker.GridRow class="mb-1 flex w-full justify-between">
									{#each weekdays as day}
										<DatePicker.HeadCell
											class="text-base-content/70 w-10 rounded-md text-xs font-normal!"
										>
											<div>{day.slice(0, 2)}</div>
										</DatePicker.HeadCell>
									{/each}
								</DatePicker.GridRow>
							</DatePicker.GridHead>
							<DatePicker.GridBody>
								{#each month.weeks as weekDates}
									<DatePicker.GridRow class="flex w-full">
										{#each weekDates as date}
											<DatePicker.Cell
												{date}
												month={month.value}
												class="relative size-10 p-0! text-center text-sm"
											>
												<DatePicker.Day
													class="rounded-9px text-base-content hover:border-base-content data-selected:bg-base-content data-disabled:text-base-content/30 data-selected:text-base-100 data-unavailable:text-base-content/70 group relative inline-flex size-10 items-center justify-center border border-transparent bg-transparent p-0 text-sm font-normal whitespace-nowrap transition-all data-disabled:pointer-events-none data-outside-month:pointer-events-none data-selected:font-medium data-unavailable:line-through"
												>
													<div
														class="bg-base-content group-data-selected:bg-base-100 absolute top-[5px] hidden size-1 rounded-full transition-all group-data-today:block"
													></div>
													{date.day}
												</DatePicker.Day>
											</DatePicker.Cell>
										{/each}
									</DatePicker.GridRow>
								{/each}
							</DatePicker.GridBody>
						</DatePicker.Grid>
					{/each}
				</div>
			{/snippet}
		</DatePicker.Calendar>
	</DatePicker.Content>

	{@render extraInfo?.()}
</DatePicker.Root>
