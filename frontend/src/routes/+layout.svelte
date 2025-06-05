<script lang="ts">
	import "../app.css";
	import { Toaster } from "svelte-sonner";
	import ThemeController from "$lib/components/ThemeController.svelte";
	import type { LayoutData } from "./$types";
	import type { Snippet } from "svelte";

	let { children, data }: { children: Snippet; data: LayoutData } = $props();
</script>

<header class="navbar bg-base-300 mb-4 shadow-md">
	<div class="navbar-start">
		<div class="dropdown">
			<div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h8m-8 6h16"
					/>
				</svg>
			</div>

			<ul
				class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
			>
				<li><a href="/transactions">Transactions</a></li>
				<li><a href="/accounts">Accounts</a></li>
			</ul>
		</div>

		<a href="/" class="btn btn-ghost text-xl">berry</a>
	</div>

	<div class="navbar-center hidden lg:flex">
		<ul class="menu menu-horizontal px-1">
			<li><a href="/transactions">Transactions</a></li>
			<li><a href="/accounts">Accounts</a></li>
		</ul>
	</div>

	<div class="navbar-end">
		<ThemeController currentTheme={data.theme} />
	</div>
</header>

<main class="p-4">
	<Toaster />
	{@render children?.()}
</main>
