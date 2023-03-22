<script lang="ts">
  import LoginForm from '$components/auth/LoginForm.svelte'
  import { sendSignInLink } from '$lib/firebase/client';
  import { setSignInEmail } from '$lib/localStorage/signInEmail';

  const onSubmit = async (email: string) => {
    const redirectUrl = `${window.location.origin}/auth/session`
    await sendSignInLink(email, redirectUrl)
    setSignInEmail(email)
  }
</script>

<svelte:head>
  <title>Login | Evident Stack</title>
</svelte:head>

<h1>Login</h1>
<LoginForm onSubmit={onSubmit}></LoginForm>
