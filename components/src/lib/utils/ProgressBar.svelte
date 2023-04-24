<script>
  import { tweened } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';

  export let done = false;

  export let progressClass =
    '[&::-webkit-progress-bar]:rounded-lg [&::-webkit-progress-value]:rounded-l-lg [&::-webkit-progress-bar]:bg-[#e6e6e6] [&::-webkit-progress-bar]:dark:bg-gray-brand-1 [&::-webkit-progress-value]:bg-green [&::-moz-progress-bar]:bg-[#e6e6e6]/ [&::-moz-progress-bar]:dark:bg-gray-brand-1/ [&::-moz-progress-value]:bg-green/ block w-full h-1 mx-2';

  const progress = tweened(0, {
    duration: 400,
    easing: cubicOut
  });
</script>

{#if !done}
  <progress value={$progress} class={progressClass} />
{:else}
  <progress value="1" class={progressClass} />
{/if}
