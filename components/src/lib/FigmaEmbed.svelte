<script lang="ts">
  export let url: string | undefined;
  export let width: number | string = "100%";
  export let height: number | string = "100%";
</script>

{#if url}
<iframe
  title="Evident Stack Figma Embed"
  style="border: 1px solid rgba(0, 0, 0, 0.1);"
  {width}
  {height}
  src="https://www.figma.com/embed?embed_host=evidentstack.com&url={url}"
  allowfullscreen />
{/if}
