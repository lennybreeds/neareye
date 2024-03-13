<script lang="ts">
	import VoteCard from '$lib/components/+voteCard.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Menu from '$lib/png/menu1.svg';

	import onMount from 'svelte'
	
	onMount(() => {
		import web3 from'@solana/web3.js';
		import anchor from"@coral-xyz/anchor";
		import fs from 'fs';
		import {AnchorProvider, Wallet} from '@coral-xyz/anchor';
		// Initialize connection to the Solana cluster
		const providerUrl = 'http://127.0.0.1:8899';
		const programId = new web3.PublicKey('EvWSp8yjbVF9vC7BrBWzejhS6GsqcyE3hELv54r614Yt'.trim());
		// Create a connection to the Solana cluster
		const connection = new web3.Connection(providerUrl, 'processed');

		// Load the wallet keypair
		const keypair = web3.Keypair.fromSecretKey(
		new Uint8Array(from_browser)
		);

		const wallet = new Wallet(keypair);

		const idl = JSON.parse(fs.readFileSync('..lib/idl.json', 'utf8'));
	})

</script>

<div class="items-centre flex w-full flex-row place-content-between border-b-2 border-white p-4">
	<a href="/">
		<h3 class="text-3xl text-white">SOLEYE</h3>
	</a>
	<Button href="/wallet" variant="secondary">Connect wallet</Button>
	<img src={Menu} />
</div>
<div class="flex w-full flex-col items-center justify-center">
	<VoteCard whitelist_votes="100" blacklist_votes="200" />
	<VoteCard />
	<VoteCard />
	<VoteCard />
	<VoteCard />
</div>

<style>
	h3 {
		font-family: 'aquire', sans-serif;
		text-shadow:
			0 0 10px #fff,
			0 0 20px #fff;
		/* 0 0 30px #fff; */
		/* 0 0 40px #fff; */
	}
	h3 {
		font-family: 'aquire', 'sans-serif';
	}
</style>
