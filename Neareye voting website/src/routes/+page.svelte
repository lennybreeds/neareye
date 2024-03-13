<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	let visible: Boolean = false;

	onMount(() => {
		visible = true;
	});
</script>

<!-- http://10.0.124.132:5173/ -->

{#if visible}
	<div transition:fade={{ delay: 1000, duration: 1500 }}>
		<div class="flex h-screen w-full items-center justify-center">
			<div class="flex flex-col items-center">
				<h1 class="text-white">SOLEYE</h1>
				<div class="space-x-2">
					<Button
						href="https://chrome.google.com/webstore/detail/your-extension-id"
						target="_blank"
						rel="noopener noreferrer">Get extension</Button
					>
					<Button href="/home" variant="secondary">Open DAPP</Button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	h1 {
		font-family: 'aquire', 'sans-serif';
		font-size: 200px;
	}
	h1,
	h3 {
		font-family: 'aquire', sans-serif;
		text-shadow:
			0 0 10px #fff,
			0 0 20px #fff;
		/* 0 0 30px #fff; */
		/* 0 0 40px #fff; */
	}
</style>
