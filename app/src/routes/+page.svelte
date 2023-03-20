<script>
  const handleClick = function(e) { console.log("clicked", e) };
</script>

<h1>Welcome</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={handleClick}>Greet</button>
