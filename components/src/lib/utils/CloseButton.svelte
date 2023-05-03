<script lang="ts">
  import Icon from '../Icon.svelte';
  import Close from '../icons/Close.svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();
  function handleClick(event: any) {
    dispatch('click', event);
  }
</script>

<button type="button" on:click={handleClick}>
  <span class="sr-only">Close</span>
  <Icon
    name="close"
    size={16}
    viewBox="0 0 20 20"
    class="stroke-2"
    iconColor="text-body-light dark:text-gray-primary"
    pathName={Close}
  />
</button>
