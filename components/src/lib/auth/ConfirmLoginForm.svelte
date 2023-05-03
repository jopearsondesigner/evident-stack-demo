<script context="module" lang="ts">
  export type FormState = 'validating' | 'idle' | 'submitting' | Error
</script>

<script lang="ts">
  import Button from "../Button.svelte";
  import Input from "../form/Input.svelte";

  export let state: FormState
  export let email: string | null = null

  /**
   * @throws {Error}
   */
  export let onSubmit = async (newEmail: string) => {
    email = newEmail

    console.log("ConfirmEmailForm email:", newEmail)
  }

  const handleSubmit: svelte.JSX.EventHandler<SubmitEvent, HTMLFormElement> = async ({currentTarget}) => {
    state = 'submitting'

    try {
      await onSubmit(new FormData(currentTarget).get('email') as string)
    } catch (error) {
      state = error as Error
    }
  }
</script>

<div>
  {#if state instanceof Error}
    <p>{state.message}</p>
  {:else if state === 'validating'}
    <p>Validating sign in link</p>
  {:else if state === 'submitting'}
    <p>We are signing you in as {email}</p>
  {:else}
    <p class="mb-2">
      Looks like you’re logging in from a different device from where
      you requested the sign in link.  Please confirm your email to
      complete your sign in.
    </p>
    <form on:submit|preventDefault={handleSubmit}>
      <Input name='email' type='email' placeholder='you@example.com' aria-label='email' required />
      <Button size='xl' color='primary' type='submit'><span slot='label'>finish sign in</span></Button>
    </form>
  {/if}
</div>
