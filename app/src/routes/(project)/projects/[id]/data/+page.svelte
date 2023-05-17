<script lang="ts">
  import Player from '$components/utils/Player.svelte';
  import Button from '$components/Button.svelte';

  import { page } from '$app/stores';

  let dataExpanded = false;
  let path: string;

  $: path = $page.url.pathname;
  console.log(path);

  $: if (path == '/design/models') {
    dataExpanded = true;
    console.log(path);
    console.log(dataExpanded);
  } else {
    dataExpanded = false;
  }
</script>

<div class="h-screen flex justify-center items-center">
  <div>
    <Player src="/lottie/DataAnimation.json" width="500" />
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
