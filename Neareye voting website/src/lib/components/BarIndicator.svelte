<script lang="ts">
	export let vote_left: number;
	export let vote_right: number;

	let total = parseFloat(vote_left) + parseFloat(vote_right);
	let left_percent = (vote_left * 100) / total;
	let right_percent = (vote_right * 100) / total;
</script>

<div class="flex w-full flex-row">
	<div class="h-3 rounded-s-full bg-white" style="width: {left_percent}%;"></div>
	<div class="h-3 rounded-e-full border-2 border-white" style="width: {right_percent}%;"></div>
</div>
