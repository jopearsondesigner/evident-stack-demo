<script>
  import Player from '$components/utils/Player.svelte';
  import Button from '$components/Button.svelte';
</script>

<div class="h-screen flex justify-center items-center">
  <div>
    <Player src="/lottie/DomainFunctionsAnimation.json" width="500" />
    <div class="flex justify-center">
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
