<script lang="ts">
  import '../../app.css';
  import type { LayoutData } from './$types'

  export let data: LayoutData
</script>

{#if data.session?.user}
  <slot />
{:else}
  <p>Whoops, you're not signed in! Please <a class='text-primary' href='/auth/sign-in'>sign in</a> to continue.</p>
{/if}
