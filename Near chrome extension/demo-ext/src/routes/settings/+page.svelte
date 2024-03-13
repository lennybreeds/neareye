<script lang="ts">
	import { goto } from '$app/navigation';
	import leftArrow from '$lib/icons/nav-arrow-left.svg';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	let visibility: Boolean = false;
	onMount(() => {
		visibility = true;
	});

	function goTo() {
		goto('/dashboard');
	}
</script>

{#if visibility}
	<div class="h-full w-full">
		<div class="flex h-1/8 w-full flex-row items-center justify-between text-white">
			<button on:click={goTo} transition:fade class=" flex w-1/3 justify-start">
				<img src={leftArrow} alt="back-button" />
			</button>
			<h1 class="font-aquire flex w-1/3 justify-center text-3xl text-white">Soleye</h1>
			<div class="w-1/3"></div>
		</div>
		<div class="custom-separator"></div>
		<div class="flex h-7/8 w-full flex-row items-center justify-center p-2" transition:fade>
			<div class="h-full w-3/5 rounded border border-white">
				<h3 class="ml-1 text-lg text-white">Settings</h3>
				<h4 class="ml-1 text-base text-white">Show banner:</h4>
				<div class="mb-1 ml-3 flex items-center">
					<input
						id="safe"
						type="checkbox"
						value=""
						class="h-3 w-3 rounded-2xl border-gray-300 bg-gray-100 text-gray-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600"
					/>
					<label for="safe" class="ms-2 text-sm text-white dark:text-gray-300"
						>Show safe banner</label
					>
				</div>
				<div class="mb-1 ml-3 flex items-center">
					<input
						id="loading"
						type="checkbox"
						value=""
						class="h-3 w-3 rounded-2xl border-gray-300 bg-gray-100 text-gray-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600"
					/>
					<label for="loading" class="ms-2 text-sm text-white dark:text-gray-300"
						>Show loading banner</label
					>
				</div>
				<div class="mb-1 ml-3 flex items-center">
					<input
						id="danger"
						type="checkbox"
						value=""
						class="h-3 w-3 rounded-2xl border-gray-300 bg-gray-100 text-gray-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600"
					/>
					<label for="danger" class="ms-2 text-sm text-white dark:text-gray-300"
						>Show danger banner</label
					>
				</div>
			</div>
			<div class="w-2"></div>
			<div class="h-full w-2/5 rounded border border-white">
				<h3 class="ml-1 text-base text-white">Whitelist</h3>
				<p class="ml-1 text-2xs text-gray-500">
					These websites won't be analysed and no warnings will appear
				</p>
				<textarea class="m-1 h-2/3 w-full bg-inherit text-white focus:border-none"
					>Kraken.com
				</textarea>
			</div>
		</div>
	</div>
{/if}

<style>
	.custom-separator {
		background: linear-gradient(
			to right,
			rgba(255, 255, 255, 0) 0%,
			rgba(255, 255, 255, 0) 20%,
			#ffffff 20%,
			#ffffff 80%,
			rgba(255, 255, 255, 0) 80%,
			rgba(255, 255, 255, 0) 100%
		);
		height: 1px; /* Adjust the height as needed */
	}
	@font-face {
		font-family: 'aquire';
		src: url('Aquire-BW0ox.otf') format('opentype');
		font-weight: normal;
		font-style: normal;
	}
	h1 {
		font-family: 'aquire', 'sans-serif';
	}
	textarea {
		border: none;
		overflow: auto;
		outline: none;

		-webkit-box-shadow: none;
		-moz-box-shadow: none;
		box-shadow: none;

		resize: none; /*remove the resize handle on the bottom right*/
	}
</style>
