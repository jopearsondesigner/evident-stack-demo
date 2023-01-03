<script lang='ts'>
    import classNames from 'classnames';
    import Navbar from './Navbar.svelte';
    import NavHamburger from './NavHamburger.svelte';
    import NavBrand from './NavBrand.svelte';
    import Nav from './Nav.svelte';
    import NavToolbar from './NavToolbar.svelte';
    import Icon from '../Icon.svelte';
    import Button from '../Button.svelte';
    import IconButton from '../IconButton.svelte';
    import Avatar from '../Avatar.svelte';
    import Home from '../icons/Home.svelte';
    import Docs from '../icons/Docs.svelte';
    import Support from '../icons/Support.svelte';
    import AdminPortal from '../icons/AdminPortal.svelte';
    let brandClass='flex no-underline mx-3';
    let website:boolean = false;
    export let user:boolean = true;
</script>

<Navbar website>
    <Nav {website}>
        <div class="inline-flex space-x-3 mx-3 items-center">
            <NavHamburger {website}/>
            <NavBrand height={32} class={classNames('mx-3',brandClass)}/>
            <NavToolbar>
                <IconButton iconBtnClass="hover:bg-gray-brand-1/0">
                    <Icon
                    name="home"
                    iconColor="text-body-light hover:text-brand-primary transition duration-200 ease-in">
                        <Home />
                    </Icon>
                </IconButton>
            </NavToolbar>
            {#if user}
            <div class="tooltip tooltip-bottom" data-tip="Docs">
                <IconButton><Icon name="docs"><Docs/></Icon></IconButton>
            </div>
            <div class="tooltip tooltip-bottom" data-tip="Support">
                <IconButton><Icon name="support"><Support/></Icon></IconButton>
            </div>
            <div class="tooltip tooltip-bottom" data-tip="Admin Portal">
                <IconButton><Icon name="admin-portal"><AdminPortal/></Icon></IconButton>
            </div>
            <span class="flex justify-center px-2">
                <Avatar/>
            </span>
            {:else}
            <Button color="secondary" size="sm" on:click label="Log in" />
            <Button gradient color="primary" size="sm" on:click label="Sign up" />
            {/if}
        </div>
    </Nav>
</Navbar>

