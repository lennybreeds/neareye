<script lang="ts">
	import { fade, slide } from 'svelte/transition';
	import check from '$lib/icons/check-circle.svg';
	import plus from '$lib/icons/plus.svg';
	import minus from '$lib/icons/minus.svg';
	import minusRed from '$lib/icons/minus-circle-red.svg';
	import { stringify } from 'postcss';

	export let open = false;
	const handleClick = () => (open = !open);

	type Reasons = {
		url: string;
		site_content: string;
	};
	export let title: string;
	export let reasons: string[];
	let reason_array = reasons.toString().split('|');
</script>

<div class="mb-2">
	<button on:click={handleClick} class="flex w-full flex-row items-center justify-between">
		<div class="flex flex-row items-center">
			<div class="w-fit rounded bg-gray-800 p-1">
				<!-- <h1>reasons: {reason_array[0].toString().toLowerCase()}</h1> -->
				{#if reason_array[0].toString().toLowerCase() == 'none'}
					<img src={check} alt="check" />
				{:else}
					<img src={minusRed} alt="negative-sign" />
				{/if}
			</div>
			<h3 class="ml-1 text-lg font-light text-white">{title}</h3>
		</div>
		<div>
			{#if open}
				<img src={minus} alt="minus" />
			{:else}
				<img src={plus} alt="plus" />
			{/if}
		</div>
	</button>
	{#if open}
		<div class="mb-2 ml-1" transition:slide>
			{#each reason_array as reason}
				<h3 class="text-sm font-light text-white">- {reason}</h3>
			{/each}
		</div>
	{/if}
</div>
