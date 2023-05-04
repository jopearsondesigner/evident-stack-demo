<script>
  import Player from '$components/utils/Player.svelte';
  import Form from '$components/form/Form.svelte';
  import Input from '$components/form/Input.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import Button from '$components/Button.svelte';
</script>

<div class="h-screen flex justify-center items-center">
  <div>
    <Player src="lottie/DataAnimation.json" width="500" />
    <Form formClass="p-3 w-full">
      <Input placeholder="Email" size="md" />
      <Textarea placeholder="Message" class="mt-3" size="md" />
      <input type="submit" id="layers-form" class="hidden" on:click />
    </Form>
    <div class="flex justify-start">
      <Button
        input
        boundTo="layers-form"
        tabindex={0}
        gradient
        color="brandStackPrimary"
        size="md"
        label="Keep me updated"
        class="m-3"
      />
    </div>
  </div>
</div>
