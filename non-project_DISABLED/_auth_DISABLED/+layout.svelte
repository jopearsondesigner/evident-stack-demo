<script lang="ts">
  import { goto } from '$app/navigation';
  import type { LayoutData } from './$types'

  export let data: LayoutData;
  const { supabase } = data;

  const handleSubmit = async () => {
    await supabase.auth.signOut()
    await goto("/auth")
	};
</script>

{#if data.session?.user}
  <p>
    You're already signed in. If you'd like to sign in with a
    different account, first <button class="button inline" on:click|preventDefault={handleSubmit}>sign out</button>.
  </p>
{:else}
  <slot />
{/if}
