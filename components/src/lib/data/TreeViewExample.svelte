<script>
  // from https://svelte.dev/repl/82b00644720a4ca2bdb89c6a653ec987
  import TreeView from './TreeView.svelte';

  const tree = {
    label: 'USA',
    children: [
      {
        label: 'Florida',
        children: [
          { label: 'Jacksonville' },
          {
            label: 'Orlando',
            children: [
              { label: 'Disney World' },
              { label: 'Universal Studio' },
              { label: 'Sea World' }
            ]
          },
          { label: 'Miami' }
        ]
      },
      {
        label: 'California',
        children: [{ label: 'San Francisco' }, { label: 'Los Angeles' }, { label: 'Sacramento' }]
      }
    ]
  };

  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-4"><ThemeSwitch /></span>

<TreeView {tree} />
