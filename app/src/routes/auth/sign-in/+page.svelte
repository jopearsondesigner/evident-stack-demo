<script lang="ts">
  import { Auth } from '@supabase/auth-ui-svelte';
  import { ThemeSupa } from '@supabase/auth-ui-shared';
  import type { PageData } from './$types';

  export let data: PageData;
</script>

<svelte:head>
  <title>Login | Evident Stack</title>
</svelte:head>

<h1>Login</h1>
<div class="mt-16">
  <Auth
    supabaseClient={data.supabase}
    view="magic_link"
    redirectTo={`${data.url}/auth/sign-in/loading?redirect=/`}
    showLinks={false}
    appearance={{ theme: ThemeSupa }} />
</div>
