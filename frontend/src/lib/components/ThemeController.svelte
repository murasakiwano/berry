<script lang="ts">
	import { browser } from "$app/environment";

	import { capitalize } from "$lib";
	import { DropdownMenu } from "bits-ui";
	import PaintBrushBroad from "phosphor-svelte/lib/PaintBrushBroad";

	let theme = $state("default");

	if (browser) {
		$effect(() => {
			const saved = localStorage.getItem("theme");
			if (saved) {
				theme = saved;
			}
		});

		$effect(() => {
			localStorage.setItem("theme", theme);
			document.documentElement.setAttribute("data-theme", theme);
		});
	}

	const availableThemes = [
		"business",
		"coffee",
		"dark",
		"darkberry",
		"emerald",
		"light",
		"shadaisy",
		"sunset",
		"wireframe",
	];
</script>

<DropdownMenu.Root>
	<div class="dropdown-center">
		<DropdownMenu.Trigger class="btn bg-base-300 hover:bg-base-200 border-base-300 mr-8">
			<PaintBrushBroad weight="bold" class="h-5 w-5" />
		</DropdownMenu.Trigger>

		<DropdownMenu.Portal>
			<DropdownMenu.Content class="dropdown-content focus-visible:outline-hidden">
				<DropdownMenu.RadioGroup
					bind:value={theme}
					class="bg-base-200 w-auto rounded-sm p-0 shadow-lg"
				>
					{#each availableThemes as theme}
						<DropdownMenu.RadioItem
							value={theme}
							class="theme-controller hover:bg-primary hover:text-primary-content my-1 cursor-default rounded px-2 py-1 text-sm font-medium focus-visible:outline-none"
						>
							{#snippet children()}
								{capitalize(theme)}
							{/snippet}
						</DropdownMenu.RadioItem>
					{/each}
				</DropdownMenu.RadioGroup>
			</DropdownMenu.Content>
		</DropdownMenu.Portal>
	</div>
</DropdownMenu.Root>
