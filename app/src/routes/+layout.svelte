<script lang="ts">
  import '../app.css';
  import { invalidate } from "$app/navigation";
  import { onMount } from "svelte";
  import type { LayoutData } from "./$types";

  // === Authentication Handling

  export let data: LayoutData;

  $: ({ supabase, session } = data)

  onMount(() => {
    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange((_event, session_) => {
      if (session_?.expires_at !== session?.expires_at) {
        invalidate('supabase:auth');
      }
    });

    return () => subscription.unsubscribe();
  })

  // === End Authentication Handling
</script>

<svelte:head>
  <title>Evident Stack</title>
</svelte:head>

<slot />
