<script lang='ts'>
    import classNames from 'classnames';
	import Logo from '../assets/images/global/onoteLogo.svg';
    import Dropdown from '../dropdown/Dropdown.svelte';
    import Button from '../Button.svelte';
    import Icon from '../Icon.svelte';
    import ArrowRight from '../icons/ArrowRight.svelte';
    import Menu from '../icons/Menu.svelte';
	export let alt="Brand Logo";
    export let href: string;
    export let navbarClass='header shadow-md z-30 relative bg-white dark:bg-dark-2';
    export let navDivClass='xl:container mx-auto flex justify-between items-center lg:py-0 lg:justify-start lg:space-x-8';
    export let navBrandClass='flex min-w-[103.26px] no-underline';
    export let logoClass='lg:h-[28px] h-[25px] ml-4';
    export let product='oNote';
    export let navHamburgerClass='mr-4 lg:hidden bg-none rounded-md p-2 inline-flex items-center justify-center text-body-1 dark:text-body-4 no-underline';
    export let navClass='hidden lg:flex-1 lg:flex lg:items-center lg:justify-between';
    export let navUlClass='flex space-x-8 items-center';
    export let navLiClass='text-sm whitespace-nowrap text-gray-brand-1 hover:text-gray-brand-1 dark:text-gray-brand-4 dark:hover:text-gray-brand-1 transition duration-300 ease-in-out inline-flex no-underline';
    let website = false;
</script>
<!-- Navbar -->
<header class={navbarClass}>
    <!-- AppBar -->
    <div
        class={navDivClass}>
        <!-- Brand -->
        <a {href} class="">
            <span class="sr-only">{product}</span>
            <!-- Logo -->
            <img class={logoClass} src={Logo} {alt} height="32">
        </a>
        <!-- /Brand -->
        <!-- OpenMobileMenuButton -->
        <button type="button" class={navHamburgerClass}>
            <span class="sr-only">Open menu</span>
            <Icon name='menu' size={24} viewBox='0 0 24 24' iconColor='dark:text-gray-primary'><Menu/></Icon>
        </button>
        <!-- /OpenMobileMenuButton -->
        <!-- MainNav -->
        <nav
        class={navClass}>
            <ul class={navUlClass}>
                <Dropdown name='Why oNote?' {website}>
                    <ul>
                        <li {href} class={navLiClass}>Features</li>
                    </ul>
                </Dropdown>
                <li {href} class={navLiClass}>
                    Docs
                </li>
                <Dropdown name='Learn' {website}>
                    <ul class={navUlClass}>
                        <li {href} class={navLiClass}>Blog</li>
                        <li {href} class={navLiClass}>Webinars</li>
                        <li {href} class={navLiClass}>Conferences</li>
                    </ul>
                </Dropdown>
                <Dropdown name='Service & Support' {website}>
                    <ul class="flex space-x-4">
                        <li {href} class={navLiClass}>Support</li>
                    </ul>
                </Dropdown>
                <Dropdown name='Company' {website}>
                    <ul class="flex space-x-4">
                        <li {href} class="text-sm whitespace-nowrap text-gray-brand-1 hover:text-gray-brand-1 dark:text-gray-brand-4 dark:hover:text-gray-brand-1 transition duration-300 ease-in-out inline-flex no-underline">Team</li>
                        <li {href} class="text-sm whitespace-nowrap text-gray-brand-1 hover:text-gray-brand-1 dark:text-gray-brand-4 dark:hover:text-gray-brand-1 transition duration-300 ease-in-out inline-flex no-underline">Contact</li>
                    </ul>
                </Dropdown>
            </ul>
            <!-- AccountNav -->
            <ul class={classNames(navUlClass, 'lg:mr-12')}>
                <li {href} target="_blank" class={navLiClass}>Log In</li>
                <Button href='https://app.onote.com/sign-up' gradient color='brandSecondary' class='flex items-center' size='sm'>
                    Sign Up
                    <Icon name='arrow-right' slot=icon class='stroke-2 ml-1' iconColor='fill-current' size={12}><ArrowRight/></Icon>
                </Button>
            </ul>
            <!-- /AccountNav -->
        </nav>
        <!-- /MainNav -->
    </div>
    <!-- /AppBar -->
</header>