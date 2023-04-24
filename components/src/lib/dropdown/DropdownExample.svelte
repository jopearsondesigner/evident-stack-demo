<script>
  import Icon from '$lib/Icon.svelte';
  import Profile from '$lib/icons/Profile.svelte';
  import DropdownMenu from '$lib/dropdown/DropdownMenu.svelte';
  import DropdownItem from '$lib/dropdown/DropdownItem.svelte';
  import DropdownDivider from '$lib/dropdown/DropdownDivider.svelte';

  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-4"><ThemeSwitch /></span>

<div class="w-full flex justify-center items-center my-32">
  <DropdownMenu product={true} name="profile" marginTop="mt-8" hidden>
    <Icon
      slot="button"
      name="profile"
      size={32}
      viewBox="0 0 32 32"
      class="vertical-middle"
      iconColor=""
      pathName={Profile}
    />
    <DropdownItem padding="pt-2 pb-4 px-4" textOnly={true}
      >lutobor.kostalova66@centrum.cz</DropdownItem
    >
    <DropdownItem href="#">Account</DropdownItem>
    <DropdownDivider />
    <DropdownItem>Sign Out</DropdownItem>
  </DropdownMenu>
</div>
