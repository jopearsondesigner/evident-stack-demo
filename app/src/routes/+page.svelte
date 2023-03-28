<script lang="ts">
  import { goto, invalidateAll } from "$app/navigation"

  import type { PageData } from './$types'

  export let data: PageData

  const signOut = function(_e: any) {
    fetch('/auth/session', {method: 'DELETE'})
      .then(() => invalidateAll())
      .then(() => goto('/'))
  };
</script>

<h1>Welcome</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={signOut}>Sign Out</button>

<h4>session:</h4>
<pre>
{JSON.stringify(data.session, null, 2)}
</pre>

<a href="/design/models/new">Create new Model</a>
