<script lang="ts">
  import LoginForm from '$components/auth/LoginForm.svelte'
  import { sendSignInLink } from '$lib/firebase/client';

  const onSubmit = async (email: string) => {
    const redirectUrl = `${window.location.origin}/auth/session`
    return sendSignInLink(email, redirectUrl)
  }
</script>

<svelte:head>
  <title>Login | Evident Stack</title>
</svelte:head>

<h1>Login</h1>
<LoginForm onSubmit={onSubmit}></LoginForm>
