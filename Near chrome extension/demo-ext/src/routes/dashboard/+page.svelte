<script lang="ts">
	import { fade } from 'svelte/transition';
	import Label from '$lib/components/ui/label/label.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import settings_icon from '$lib/icons/settings.svg';
	import extractDomain from 'extract-domain';
	import Table from './Table.svelte';
	import { browser } from '$app/environment';
	export { load } from './+page.ts';

	let checked: boolean = true;
	import { onMount } from 'svelte';
	export let data;
	console.log('data on dashboard', data);

	// let found_reasons;

	let currentUrl: string | undefined = 'Loading...';
	let visibility: Boolean = false;
	onMount(() => {
		visibility = true;
		// if (browser) {
		// 	chrome.storage.sync.get(['current_url', 'reply'], function (items) {
		// 		// console.log('Settings retrieved', items);
		// 		found_reasons = items;
		// 		// console.log('found reasons', found_reasons);
		// 	});
		// }
	});

	let url;
	async function getUrl() {
		const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
		url = tab.url;
		// console.log(url);
	}

	import { goto } from '$app/navigation';

	function goTo() {
		goto('/settings');
	}

	onMount(async () => {
		if (chrome.tabs) {
			// Query the current active tab in the current window
			chrome.tabs.query({ active: true, currentWindow: true }, function (tabs) {
				const currentTab = tabs[0];
				extractDomain(currentTab.url, { tld: true })?.then((resolvedValue) => {
					currentUrl = resolvedValue;
					// console.log(currentUrl);
				});
				// console.log(currentUrl);
			});
		}
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="popup.css" />
</svelte:head>

{#if visibility}
	<div class="h-full w-full">
		<div class="flex h-1/8 w-full flex-row items-center justify-between text-white">
			<div class="flex w-1/3 justify-center"></div>
			<h1 class="font-aquire flex w-1/3 justify-center text-3xl text-white">Soleye</h1>
			<button on:click={goTo} transition:fade class="flex w-1/3 justify-end pr-2">
				<img src={settings_icon} alt="" />
			</button>
		</div>
		<div class="custom-separator"></div>
		<div class="h-1/8" transition:fade>
			<div class="flex items-center space-x-2 p-2">
				<Switch bind:checked id="enable-toggle" />
				<Label for="enable-toggle">Enable protection</Label>
			</div>
		</div>
		<div class="flex h-3/4 w-full flex-row items-center justify-center p-2" transition:fade>
			<div
				class="flex h-full w-1/2 flex-col items-center justify-between rounded border border-white"
			>
				{#if currentUrl}
					<h3 class=" text-white">
						{currentUrl.charAt(0).toUpperCase() + currentUrl.slice(1)}
					</h3>
				{:else}
					<h3 class="text-white">URL NOT FOUND</h3>
				{/if}
				<h1 class="text-7xl text-white">{data.props.data.reply.probability * 100}%</h1>
				{#if data.props.data.reply.probability <= 0.4}
					<h3 class="text-white">This site has a good rating</h3>
				{:else}
					<h3 class="text-white">This site has a poor rating</h3>
				{/if}
			</div>
			<div class="w-2"></div>
			<div class="h-full w-1/2 border-t border-white">
				<div class="mt-1">
					<Table title="URL" reasons={data.props.data.reply['URL reasons']} />
					<Table title="Site content" reasons={data.props.data.reply['site content reasons']} />
					<Table
						title="Javascript code"
						reasons={data.props.data.reply['javascript code reasons']}
					/>
					<Table title="Domain age" reasons={data.props.data.reply['domain age reasons']} />
				</div>
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
</style>
