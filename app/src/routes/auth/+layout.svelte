<script lang="ts">
  import { handleSignOut } from "$lib/user";
  import "../../app.css";
  import type { LayoutData } from './$types'

  export let data: LayoutData
</script>

{#if data.session.user}
  <p>You're already signed in. If you'd like to sign in with a
    different account, first
    <a class=''
       href="#sign-out"
       on:click|preventDefault={handleSignOut}>
      sign out
    </a>.</p>
{:else}
  <slot />
{/if}
