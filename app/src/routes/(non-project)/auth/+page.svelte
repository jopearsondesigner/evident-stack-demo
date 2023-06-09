<script lang="ts">
  import { Auth } from '@supabase/auth-ui-svelte';
  import { ThemeSupa } from '@supabase/auth-ui-shared';
  import type { PageData } from './$types';

  export let data: PageData;

  const { supabase, url } = data;
</script>

<svelte:head>
  <title>Login | Evident Stack</title>
</svelte:head>

<div class="container mx-auto p-10">
  <h1>Login</h1>
  <Auth
    supabaseClient={supabase}
    view="magic_link"
    redirectTo={`${url}/auth/callback`}
    showLinks={false}
    appearance={{ theme: ThemeSupa }}
    additionalData={undefined}
    />
</div>
