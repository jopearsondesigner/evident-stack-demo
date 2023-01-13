<script lang="ts">
	export let href: string = '/';
	export let aClass: string =
		'text-sm text-body-light hover:text-body-dark dark:text-body-light dark:hover:text-body-dark transition duration-200 ease-in';
	export let spanClass: string = 'py-4 text-sm text-body-light dark:text-body-dark';
</script>

<!--MUST KEEP THESE LINKS UPDATED ALWAYS-->
<div class="text-center my-4">
	<a href="/privacy-policy" class={aClass}> Privacy </a>
	<span class={spanClass}>|</span>
	<a href="/terms-and-conditions" class={aClass}> Terms of Service </a>
</div>
