<script lang="ts">
	import { slide, fade } from 'svelte/transition';
	import BarIndicator from './BarIndicator.svelte';
	import Table from './Table.svelte';
	import navBack from '$lib/icons/nav-arrow-right.svg';
	import Button from './ui/button/button.svelte';

	let open: boolean = true;

	function toggle() {
		open = !open;
	}

	export let whitelist_votes;
	export let blacklist_votes;

	let reasons = ['Invalid1', 'invalid2'];
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="flex w-full flex-row place-items-center justify-center">
	<div class="my-5 ml-10 h-72 rounded-lg border-2 border-white" style="width:50%">
		<div class="mb-3 flex h-12 w-full max-w-3xl flex-row items-end justify-between px-8 text-white">
			<div class=" text-4xl">BINANCEE.com</div>
			<div class="flex flex-row items-end">
				<div class="aquire-font text-4xl">12:02:10</div>
				<div class="aquire-font text-xl" style="margin: 1.5px">:11</div>
			</div>
		</div>
		<div class="h-40 w-full">
			<div class="ml-8 bg-white" style="width: 250px; height: 0.5px;"></div>
			<Table title="URL" {reasons} />
			<Table title="Site content" {reasons} />
			<Table title="Domain age" {reasons} />
		</div>
		<div class="px-8">
			<div class="aquire-font flex flex-row place-content-between text-white">
				<h4 class="text-4xl">200</h4>
				<h4 class="text-4xl">100</h4>
			</div>
			<BarIndicator vote_left={whitelist_votes} vote_right={blacklist_votes} />
		</div>
	</div>
	{#if open}
		<div
			transition:slide={{ axis: 'x' }}
			class=" flex h-72 w-60 flex-col-reverse rounded border-2 border-white pb-3 text-white"
		>
			<Button variant="outline" style="color: black; width: 90%; margin: 0 auto;"
				><p class="aquire-font text-xl">Whitelist</p></Button
			>
			<div class="h-1 w-full"></div>
			<Button
				style="color: black; width: 90%; margin: 0 auto; border: white 2px solid; background-color: black;"
				><p class="aquire-font text-xl text-white">Blacklist</p></Button
			>
			<!-- <button>Whitelist</button> -->
			<!-- <button>Blacklist</button> -->
		</div>
	{/if}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		on:click={toggle}
		class="flex h-24 w-10 items-center justify-center rounded-e bg-white hover:cursor-pointer"
	>
		<h4 class="aquire-font rotate-90 transform">VOTE</h4>
		<!-- <img src={navBack} /> -->
	</div>
</div>

<style>
	.aquire-font {
		font-family: 'aquire', 'sans-serif';
	}
	h3 {
		font-family: 'aquire', sans-serif;
		text-shadow:
			0 0 10px #fff,
			0 0 20px #fff;
		/* 0 0 30px #fff; */
		/* 0 0 40px #fff; */
	}
</style>
