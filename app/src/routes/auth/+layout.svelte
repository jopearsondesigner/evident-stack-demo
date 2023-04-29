<script lang="ts">
  import { enhance } from "$app/forms";
  import "../../app.css";
  import type { LayoutData } from './$types'

  export let data: LayoutData

  let loading = false;

  function handleSubmit() {
		loading = true;
		return async () => {
			loading = false;
		};
	}
</script>

{#if data.session?.user}
  <p>
    You're already signed in. If you'd like to sign in with a
    different account, first
  </p>
  <form method="POST" action="/auth/sign-out" use:enhance={handleSubmit}>
    <button class="button inline" disabled={loading}>sign out</button>
  </form>.
{:else}
  <slot />
{/if}
