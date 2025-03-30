<script lang="ts">
	import { browser } from "$app/environment";

	import { capitalize } from "$lib";

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
		"default",
		"forest",
		"coffee",
		"night",
		"sunset",
		"abyss",
		"synthwave"
	];
</script>

<div class="dropdown dropdown-center">
	<div tabindex="0" role="button" class="btn m-1">
		Theme
		<svg
			width="12px"
			height="12px"
			class="inline-block h-2 w-2 fill-current opacity-60"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 2048 2048"
		>
			<path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
		</svg>
	</div>
	<ul
		tabindex="0"
		class="menu bg-base-100 dropdown-content z-1 w-auto rounded-sm p-2 shadow-2xl"
		role="radiogroup"
	>
		{#each availableThemes as t}
			<li>
				<input
					type="radio"
					name="theme-dropdown"
					class="theme-controller btn btn-sm btn-block btn-ghost my-1"
					aria-label={capitalize(t)}
					value={t}
					bind:group={theme}
				/>
			</li>
		{/each}
	</ul>
</div>
