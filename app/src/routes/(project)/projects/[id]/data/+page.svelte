<script lang="ts">
  import Player from '$components/utils/Player.svelte';
  import Button from '$components/Button.svelte';

  import { page } from '$app/stores';
  import Icon from '$components/Icon.svelte';
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarWrapper from '$components/drawer/SidebarWrapper.svelte';
  import SidebarContainer from '$components/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$components/drawer/SidebarGroup.svelte';
  import SidebarDropdownWrapper from '$components/drawer/SidebarDropdownWrapper.svelte';
  import SidebarDropdownItem from '$components/drawer/SidebarDropdownItem.svelte';
  import SidebarItem from '$components/drawer/SidebarItem.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import Label from '$components/form/Label.svelte';
  import Schema from '$components/icons/Schema.svelte';
  import Download from '$components/icons/Download.svelte';
  import DataLogo from '$components/assets/images/product/data/evidentDataLogo.svg';

  let isVerticalOpen = false;
  let isClosed = true;
  let valueEventModel = '';
  let grid = true;
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

  let handleClick = () => {
    isClosed = !isClosed;
  };
</script>

<SidebarContainer
  src={DataLogo}
  id="data"
  title="Data"
  slot="data"
  on:click={() => (isClosed = true)}
  on:click={() => (isVerticalOpen = false)}
  on:click={() => (grid = false)}
  bind:expanded={dataExpanded}
>
  <SidebarGroup />
</SidebarContainer>

<div class="h-screen flex justify-center items-center">
  <div>
    <Player src="lottie/DataAnimation.json" width="500" />
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
