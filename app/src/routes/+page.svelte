<script>
  const signOut = function(e) {
    fetch('/auth/session', {method: 'DELETE'})
  };
</script>

<h1>Welcome</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={signOut}>Sign Out</button>
