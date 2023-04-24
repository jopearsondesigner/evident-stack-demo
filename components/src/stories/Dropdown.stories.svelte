<script>
  // @ts-nocheck

  import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

  import DropdownMenu from '$lib/dropdown/DropdownExample.svelte';
</script>

<Meta title="product/Dropdown Menu" component={DropdownMenu} />

<Template let:args>
  <DropdownMenu />
</Template>

<Story name="blank" args={{ type: 'blank' }} />
