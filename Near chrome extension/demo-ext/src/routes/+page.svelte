<script lang="ts">
	import { onNavigate } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { browser } from '$app/environment';
	import { json } from '@sveltejs/kit';

	let jsonData;

	onMount(() => {
		if (browser) {
			chrome.storage.sync.get(['current_url', 'reply'], function (items) {
				jsonData = items;
			});
		}
	});
	// if (browser) {
	// 	// console.log('browser');
	// 	chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
	// 		if (request.site === 'new') {
	// 			// console.log('received new site notification');

	// 			fetch('https://api.sampleapis.com/coffee/hot')
	// 				.then((response) => {
	// 					if (!response.ok) {
	// 						throw new Error('Network response was not ok');
	// 					}
	// 					return response.json();
	// 				})
	// 				.then((data) => {
	// 					// console.log(data); // Log or use the data here
	// 					sendResponse({ farewell: 'finished banner', data: data });
	// 				})
	// 				.catch((error) => {
	// 					console.error('There has been a problem with your fetch operation:', error);
	// 					sendResponse({ farewell: 'error fetching data', error: error.toString() });
	// 				});

	// 			return true; // Indicates that sendResponse will be sent asynchronously
	// 		}
	// 	});
	// }

	function modify() {
		// console.log('modify called');
		chrome.tabs.query({ currentWindow: true, active: true }, function (tabs) {
			// console.log('modify called 2');
			var activeTab = tabs[0];
			chrome.tabs.sendMessage(activeTab.id, { message: 'show-banner' }, function (response) {
				// console.log(response);
			});
		});
	}
	let visibility: Boolean = false;
	onMount(() => {
		visibility = true;
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="popup.css" />
</svelte:head>

{#if visibility}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 transition:fade={{ delay: 0 }} class=" text-7xl text-white">SolEye</h1>
		<p transition:fade={{ delay: 0 }} class="text-white">We watch your back</p>
		<div transition:fade={{ delay: 0 }}>
			<Button class="mt-3" variant="outline" href="/dashboard">Continue</Button>
			<!-- <button on:click={modify} class="text-white">Modify dom</button> -->
		</div>
		<!-- <h1 class="text-white">{(jsonData.current_url, jsonData.reply)}</h1> -->
	</div>
{/if}

<style>
	@font-face {
		font-family: 'aquire';
		src: url('/fonts/Aquire-BW0ox.otf') format('opentype');
		font-weight: normal;
		font-style: normal;
	}
	h1,
	p {
		font-family: 'aquire', sans-serif;
		text-shadow:
			0 0 10px #fff,
			0 0 20px #fff;
		/* 0 0 30px #fff; */
		/* 0 0 40px #fff; */
	}
</style>
