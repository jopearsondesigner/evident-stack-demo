<script>
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	const progress = tweened(0, {
		duration: 400,
		easing: cubicOut
	});
</script>

<progress value={$progress} />

<style>
	progress {
		display: block;
		width: 100%;
		height: 4px;
		margin-right: 8px;
		margin-left: 8px;
	}
	progress::-webkit-progress-bar {
		accent-color: #00ff00;
		background: #e6e6e6;
	}
	progress::-moz-progress-bar {
		accent-color: #00ff00;
		background: #e6e6e6;
	}
</style>
