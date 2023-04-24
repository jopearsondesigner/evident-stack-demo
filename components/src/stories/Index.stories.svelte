<script>
  import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

  import Index from '$lib/home/IndexNav.svelte';
</script>

<Meta title="product/Index" component={Index} />

<Template let:args>
  <Index {...args} />
</Template>

<Story name="blank" />
