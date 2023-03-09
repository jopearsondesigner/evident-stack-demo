<script>
	import classNames from 'classnames';
	export let asideClass =
		'h-screen w-60 bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-gray-brand-1';
	// export let menuClass = 'h-10 flex items-center px-4 border-b border-gray-primary';
	// export let spanClass = 'flex space-x-3 table my-0 mx-auto';
	// export let menuBtnClass = 'font-extrabold text-default transition duration-200 ease-in';
	export let isEventModelOpen = true;
	const handleMenu = () => {
		isEventModelOpen = !isEventModelOpen;
	};
</script>

<aside class={classNames(asideClass)}>
	<!-- <nav id="sidebarMenu" class={classNames(menuClass)}>
		<span class={classNames(spanClass)}>
			{#if isEventModelOpen}
				<button class={classNames('active', menuBtnClass)} on:click={() => handleMenu()}
					>Event Model</button
				>
				<button class={classNames(menuBtnClass)} on:click={() => handleMenu()}>Domain Models</button
				>
			{:else}
				<button class={classNames(menuBtnClass)} on:click={() => handleMenu()}>Event Model</button>
				<button class={classNames('active', menuBtnClass)} on:click={() => handleMenu()}
					>Domain Models</button
				>
			{/if}
		</span>
	</nav> -->
	{#if isEventModelOpen}
		<slot name="eventModel" />
	{:else}
		<slot name="domainModels" />
	{/if}
</aside>
