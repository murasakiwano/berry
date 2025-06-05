<script lang="ts">
	import { browser } from "$app/environment";
	import { invalidateAll } from "$app/navigation";
	import { capitalize } from "$lib";
	import { DropdownMenu } from "bits-ui";
	import { Laptop, Moon, Paintbrush, Sun } from "@lucide/svelte";
	import { onMount } from "svelte";

	let { currentTheme = "light" }: { currentTheme?: string } = $props();

	let theme = $state(currentTheme);
	let systemTheme = $state("light");

	const themeCategories = {
		system: ["system"],
		light: ["light", "emerald", "wireframe", "tropical"],
		dark: ["dark", "dracula", "coffee", "forest", "sunset", "tropical-dark"],
	};

	function getSystemTheme(): string {
		if (!browser) return "light";
		return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
	}

	function applyTheme(selectedTheme: string): void {
		if (!browser) return;
		const themeToApply = selectedTheme === "system" ? systemTheme : selectedTheme;
		document.documentElement.setAttribute("data-theme", themeToApply);
		document.body.setAttribute("data-theme", themeToApply);
	}

	function setThemeCookie(themeName: string): void {
		if (!browser) return;
		
		const expires = new Date();
		expires.setFullYear(expires.getFullYear() + 1);
		
		document.cookie = `berry-theme=${themeName}; expires=${expires.toUTCString()}; path=/; SameSite=Lax`;
	}

	function updateTheme(newTheme: string) {
		theme = newTheme;
		applyTheme(newTheme);
		setThemeCookie(newTheme);
		
		// Save to localStorage as backup
		try {
			localStorage.setItem('theme', newTheme);
		} catch (e) {
			console.warn('Could not save theme to localStorage:', e);
		}

		// Invalidate to update server state
		invalidateAll();
	}

	if (browser) {
		onMount(() => {
			// Initialize with current theme
			theme = currentTheme;
			systemTheme = getSystemTheme();
			applyTheme(theme);

			// Set up system theme change listener
			const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
			const handleThemeChange = (e: MediaQueryListEvent) => {
				systemTheme = e.matches ? "dark" : "light";
				if (theme === "system") {
					applyTheme("system");
				}
			};

			mediaQuery.addEventListener("change", handleThemeChange);

			return () => {
				mediaQuery.removeEventListener("change", handleThemeChange);
			};
		});
	}
</script>

<DropdownMenu.Root>
	<div class="dropdown-center relative">
		<DropdownMenu.Trigger class="btn bg-base-300 hover:bg-base-200 border-base-300 mr-8">
			<Paintbrush class="h-5 w-5" />
			<span class="sr-only">Change theme</span>
		</DropdownMenu.Trigger>

		<DropdownMenu.Portal>
			<DropdownMenu.Content
				class="dropdown-content z-50 focus-visible:outline-hidden"
				sideOffset={5}
			>
				<div class="bg-base-200 rounded-box p-2 shadow-lg">
					<div class="text-base-content/70 px-2 py-1 text-sm font-medium">Themes</div>

					<DropdownMenu.RadioGroup bind:value={theme} class="w-full">
						<!-- System theme option -->
						<div class="text-base-content/50 px-2 pt-2 pb-1 text-xs font-medium uppercase">
							System
						</div>
						<DropdownMenu.RadioItem
							value="system"
							onclick={() => updateTheme("system")}
							class="theme-controller hover:bg-primary hover:text-primary-content focus-visible:bg-primary focus-visible:text-primary-content my-1 flex w-full cursor-default items-center gap-2 rounded-md px-2 py-1.5 text-sm font-medium focus-visible:outline-none"
						>
							<Laptop class="h-4 w-4" />
							<span>System Default</span>
						</DropdownMenu.RadioItem>

						<!-- Light themes -->
						<div class="text-base-content/50 px-2 pt-2 pb-1 text-xs font-medium uppercase">
							Light
						</div>
						{#each themeCategories.light as themeName}
							<DropdownMenu.RadioItem
								value={themeName}
								onclick={() => updateTheme(themeName)}
								class="theme-controller hover:bg-primary hover:text-primary-content focus-visible:bg-primary focus-visible:text-primary-content my-1 flex w-full cursor-default items-center gap-2 rounded-md px-2 py-1.5 text-sm font-medium focus-visible:outline-none"
								data-theme={themeName}
							>
								<Sun class="h-4 w-4" />
								<span>{capitalize(themeName)}</span>
							</DropdownMenu.RadioItem>
						{/each}

						<!-- Dark themes -->
						<div class="text-base-content/50 px-2 pt-2 pb-1 text-xs font-medium uppercase">
							Dark
						</div>
						{#each themeCategories.dark as themeName}
							<DropdownMenu.RadioItem
								value={themeName}
								onclick={() => updateTheme(themeName)}
								class="theme-controller hover:bg-primary hover:text-primary-content focus-visible:bg-primary focus-visible:text-primary-content my-1 flex w-full cursor-default items-center gap-2 rounded-md px-2 py-1.5 text-sm font-medium focus-visible:outline-none"
								data-theme={themeName}
							>
								<Moon class="h-4 w-4" />
								<span>{capitalize(themeName)}</span>
							</DropdownMenu.RadioItem>
						{/each}
					</DropdownMenu.RadioGroup>
				</div>
			</DropdownMenu.Content>
		</DropdownMenu.Portal>
	</div>
</DropdownMenu.Root>