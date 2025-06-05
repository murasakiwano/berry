<script lang="ts">
	import { page } from "$app/state";
	import { goto } from "$app/navigation";
	import { AlertTriangle, Home, ArrowLeft, RefreshCw } from "@lucide/svelte";
	import { Button } from "bits-ui";

	const errorStatus = $derived(page.status);
	const errorMessage = $derived(page.error?.message ?? "An unexpected error occurred");

	const getErrorTitle = (status: number): string => {
		switch (status) {
			case 404:
				return "Page Not Found";
			case 403:
				return "Access Forbidden";
			case 500:
				return "Internal Server Error";
			case 503:
				return "Service Unavailable";
			default:
				return "Something went wrong";
		}
	};

	const getErrorDescription = (status: number): string => {
		switch (status) {
			case 404:
				return "The page you're looking for doesn't exist or has been moved.";
			case 403:
				return "You don't have permission to access this resource.";
			case 500:
				return "Our server encountered an unexpected error. Please try again later.";
			case 503:
				return "The service is temporarily unavailable. Please try again in a few moments.";
			default:
				return "We encountered an unexpected problem. Please try again or contact support if the problem persists.";
		}
	};

	const handleGoBack = () => {
		if (window.history.length > 1) {
			window.history.back();
		} else {
			goto("/");
		}
	};

	const handleRefresh = () => {
		window.location.reload();
	};
</script>

<svelte:head>
	<title>Error {errorStatus} - Berry</title>
</svelte:head>

<div class="flex min-h-[60vh] items-center justify-center px-4">
	<div class="w-full max-w-md">
		<!-- Error Card -->
		<div class="card bg-base-100 shadow-xl">
			<div class="card-body items-center text-center">
				<!-- Error Icon -->
				<div class="mb-4">
					<div
						class="bg-error/10 flex h-20 w-20 items-center justify-center rounded-full"
					>
						<AlertTriangle class="text-error h-10 w-10" />
					</div>
				</div>

				<!-- Error Status -->
				<div class="mb-2">
					<span class="text-error text-6xl font-bold">{errorStatus}</span>
				</div>

				<!-- Error Title -->
				<h1 class="text-base-content mb-2 text-2xl font-bold">
					{getErrorTitle(errorStatus)}
				</h1>

				<!-- Error Description -->
				<p class="text-base-content/70 mb-6 max-w-sm">
					{getErrorDescription(errorStatus)}
				</p>

				<!-- Technical Error Message (if in development or detailed error) -->
				{#if errorMessage && errorMessage !== getErrorDescription(errorStatus)}
					<div class="alert alert-error mb-6">
						<div class="text-left">
							<p class="text-sm font-medium">Technical Details:</p>
							<p class="font-mono text-sm">{errorMessage}</p>
						</div>
					</div>
				{/if}

				<!-- Action Buttons -->
				<div class="flex w-full flex-col gap-3 sm:flex-row">
					<Button.Root onclick={handleGoBack} class="btn btn-outline flex-1">
						<ArrowLeft class="h-4 w-4" />
						Go Back
					</Button.Root>

					<Button.Root onclick={handleRefresh} class="btn btn-outline flex-1">
						<RefreshCw class="h-4 w-4" />
						Refresh
					</Button.Root>

					<Button.Root href="/" class="btn btn-primary flex-1">
						<Home class="h-4 w-4" />
						Home
					</Button.Root>
				</div>

				<!-- Additional Help -->
				<div class="divider my-6"></div>

				<div class="text-center">
					<p class="text-base-content/60 mb-3 text-sm">Need help? Try these options:</p>
					<div class="flex flex-wrap justify-center gap-2">
						<a href="/accounts" class="btn btn-ghost btn-sm"> Accounts </a>
						<a href="/transactions" class="btn btn-ghost btn-sm"> Transactions </a>
						<a href="/accounts/new" class="btn btn-ghost btn-sm"> Create Account </a>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- Background decorative elements -->
<div class="pointer-events-none fixed inset-0 -z-10 overflow-hidden">
	<div class="bg-primary/5 absolute -top-40 -right-40 h-80 w-80 rounded-full"></div>
	<div class="bg-secondary/5 absolute -bottom-40 -left-40 h-80 w-80 rounded-full"></div>
</div>
