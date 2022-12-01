<script>
 import Button from './Button.svelte';

 import { createEventDispatcher } from 'svelte';

 export let user = null;

 const dispatch = createEventDispatcher();

 function onLogin(event) {
     dispatch('login', event);
 }
 function onLogout(event) {
     dispatch('logout', event);
 }
 function onCreateAccount(event) {
     dispatch('createAccount', event);
 }
</script>

<div class="w-full navbar bg-neutral text-neutral-content">
    <div class="flex-none lg:hidden">
        <label for="my-drawer-3" class="btn btn-square btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
        </label>
    </div>
    <div class="flex-1 px-2 mx-2">oNote</div>
    <div class="flex-none hidden lg:block">
        <ul class="menu menu-horizontal">
            <!-- Navbar menu content here -->
            <li><a>Navbar Item 1</a></li>
            <li><a>Navbar Item 2</a></li>
        </ul>
    </div>
</div>
