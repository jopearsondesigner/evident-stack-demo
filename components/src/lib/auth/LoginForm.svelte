<script lang="ts">
  import Button from "../Button.svelte";
  import Input from "../form/Input.svelte";

  type FormState = 'idle' | 'submitting' | 'success' | Error

  let state: FormState = 'idle'

  let email: string | null = null

  /**
   * @throws {Error}
   */
  export let onSubmit = async (email: string) => {
    console.log("LoginForm email:", email)
  }

  const handleSubmit: svelte.JSX.EventHandler<SubmitEvent, HTMLFormElement> = async ({currentTarget}) => {
    email = new FormData(currentTarget).get('email') as string

    state = 'submitting'
    try {
      onSubmit(email)
      state = 'success'
    } catch (error) {
      state = error as Error
    }
  }
</script>

<div>
{#if state != 'success'}
  <form on:submit|preventDefault={handleSubmit}>
    <Input name='email' type='email' placeholder='you@example.com' aria-label='email' required />
    {#if state == 'submitting'}
      <p>emailing {email}...</p>
    {:else}
      <Button size='xl' color='primary' type='submit'><span slot='label'>send magic link</span></Button>
    {/if}
    {#if state instanceof Error}
      <p>Whoops, there was an error sending the email to {email}! Please try again.</p>
    {/if}
  </form>
{:else}
  <div>
    <p>We've sent your login email to {email}. Please find it in your inbox and click the link there to login.</p>
  </div>
{/if}
</div>
