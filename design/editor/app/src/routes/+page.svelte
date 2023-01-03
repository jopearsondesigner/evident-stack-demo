<script>
 import state from "$lib/state";

 const handleClick = function(e) { state.greet() };
</script>

<h1>Welcome</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={handleClick}>Greet</button>
