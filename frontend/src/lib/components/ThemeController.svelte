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
		"abyss",
		"business",
		"coffee",
		"dark",
		"darkberry",
		"forest",
		"light",
		"night",
		"sunset",
		"synthwave",
	];
</script>

<div class="dropdown dropdown-center">
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class="btn btn-ghost mr-8">
			<PaintBrushBroad class="text-primary h-5 w-5" />
		</DropdownMenu.Trigger>

		<DropdownMenu.Portal>
			<DropdownMenu.Content>
				<DropdownMenu.RadioGroup
					bind:value={theme}
					class="menu bg-base-200 dropdown-content w-auto rounded-sm p-0 shadow-lg"
				>
					{#each availableThemes as theme}
						<DropdownMenu.RadioItem
							value={theme}
							class="theme-controller hover:bg-primary hover:text-primary-content my-1 px-2 py-1 font-medium"
						>
							{#snippet children()}
								{capitalize(theme)}
							{/snippet}
						</DropdownMenu.RadioItem>
					{/each}
				</DropdownMenu.RadioGroup>
			</DropdownMenu.Content>
		</DropdownMenu.Portal>
	</DropdownMenu.Root>
</div>
