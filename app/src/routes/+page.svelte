<script lang="ts">
  import { handleSignOut } from '$lib/user';
  import type { PageData } from './$types';
  import Button from '$components/Button.svelte';

  export let data: PageData;
</script>

{#if data.session.user}
  <section class="mt-20">
    <h1>Welcome</h1>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

    <button on:click|preventDefault={handleSignOut}>Sign Out</button>

    <h4>session:</h4>
    <pre>
    {JSON.stringify(data.session, null, 2)}
  </pre>

    <Button gradient color="brandStackPrimary" size="sm" href="/new" label="Create New Model" />
  </section>
{:else}
  <p>
    Whoops, you're not signed in! Please <a class="text-primary" href="/auth/sign-in">sign in</a> to
    continue.
  </p>
{/if}
