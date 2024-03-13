<script>
	import '../app.pcss';
	import '$lib/styles/default.css';
	import { fade, fly } from 'svelte/transition';

	export let data;
</script>

<!-- {#key data.url} -->
<!-- <div class="h-full w-full" transition:fade={{ delay: 300 }}> -->
<slot />
<!-- </div> -->
<!-- {/key} -->
