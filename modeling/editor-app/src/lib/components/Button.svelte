<script>
 import { createEventDispatcher } from 'svelte';
 import { buttonLabel } from '../stores';
 /**
  * Is this the principal call to action on the page?
  */
 export let primary = false;

 /**
  * What background color to use
  */
 export let backgroundColor;
 /**
  * How large should the button be?
  */
 export let size = 'medium';

 let mode = primary ? 'storybook-button--primary' : 'storybook-button--secondary';

 let style = backgroundColor ? `background-color: ${backgroundColor}` : '';

 const dispatch = createEventDispatcher();

 /**
  * Optional click handler
  */
 function onClick(event) {
     dispatch('click', event);
 }
</script>

<button
    type="button"
    class={['btn', `btn-${size}`, `btn-${style}`, mode].join(' ')}
          on:click={onClick}>
    { $buttonLabel || "Foo" }
</button>
