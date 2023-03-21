<script lang="ts">
  import { goto, invalidateAll } from '$app/navigation';
  import { page } from '$app/stores';
  import ConfirmLoginForm, { type FormState } from '$components/auth/ConfirmLoginForm.svelte'
  import { signInWithLink } from '$lib/firebase/client';
  import { clearSignInEmail } from '$lib/localStorage/signInEmail';
  // import { setUser } from '$lib/stores/user';
  import { onMount } from 'svelte';

  let email: string | null = $page.data.signInEmail
  let state: FormState = $page.data.isSignInLink ?
      'validating' :
      new Error("Invalid sign in link! Please close this page and try again.")

  const login = async (email: string) => {
    return signInWithLink(email, window.location.href)
        .then(credential => credential.user.getIdToken())
        .then(token => fetch('/auth/session', {
				  method: 'POST',
				  headers: {
					  authorization: `Bearer ${token}`,
				  }
        }))
      .then(() => {
        clearSignInEmail()
        invalidateAll()
      })
      .then(() => goto('/')) // TODO: store original intent URL someplace
  }

  onMount(async () => {
    if (email) {
      await login(email).catch(() => {
        state = new Error(
          'We had a problem signing you in. Please try again.'
        )
      })
    } else {
      state = 'idle'
    }
  })
</script>

<svelte:head>
  <title>Confirm Login | Evident Stack</title>
</svelte:head>

<h1>Confirm your email to login</h1>
<ConfirmLoginForm state={state} onSubmit={login}></ConfirmLoginForm>
