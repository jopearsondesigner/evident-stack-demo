<script lang="ts">
  // @ts-nocheck
  import ImageUploadIcon from './icons/ImageUpload.svelte';
  import ImagePlaceholder from './assets/images/product/design/ImagePlacholder.svg';
  import Icon from './Icon.svelte';
  import IconButton from './IconButton.svelte';

  let avatar: string | ArrayBuffer | null, fileinput: HTMLInputElement;

  const onFileSelected = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
    let image = e.target.files[0];
    let reader = new FileReader();
    reader.readAsDataURL(image);
    reader.onload = (e) => {
      avatar = e.target.result;
    };
  };
</script>

{#if avatar}
  <img class="avatar" src={avatar} alt="d" />
{:else}
  <img class="opacity-20 flex w-full h-full" src={ImagePlaceholder} alt="Placeholder" />
{/if}
<div class="h-auto">
  <IconButton
    size={50}
    margin="mt-3"
    on:click={() => {
      fileinput.click();
    }}
    on:keydown >
    <Icon
      name="image-upload-icon"
      size={40}
      iconColor="text-body-light dark:text-body-dark"
      pathName={ImageUploadIcon}
      viewBox="0 0 20 20" />
  </IconButton>
</div>
<div
  class="chan"
  on:click={() => {
    fileinput.click();
  }}
  on:keydown>
  Choose Image
</div>
<input
  style="display:none"
  type="file"
  accept=".jpg, .jpeg, .png"
  on:change={(e) => onFileSelected(e)}
  bind:this={fileinput} />
