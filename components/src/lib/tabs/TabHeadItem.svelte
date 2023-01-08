<script lang='ts'>
    import classNames from 'classnames';
    export let id:number;
    export let activeTabValue:number;
    export let inactiveClass:string = '';
    export let activeClass:string = 'active';
    export let liClass:string = '';
    export let buttonClass:string = 'featureTabs';
    export let hClass:string = '';
    export let name:string = 'Tab name';
</script>

<li class={liClass} role="presentation">
    <button
        on:click
        class={classNames(activeTabValue === id ? activeClass : inactiveClass, buttonClass)}
        id="{id}-tabhead"
        type="button"
        role="tab">
        <h2 class={hClass}>{name}</h2>
        <slot/>
    </button>
</li>

<style>
    .featureButtons.active {
        border-color: #2FB6E1;
    }
    .featureButtons.active h2 {
        color: #2FB6E1;
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 150ms;
        transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
        transition-duration: 300ms;
    }
    .featureTabs {
        position: relative;
        display: inline-block;
        color: #586e75;
        text-decoration: none;
        padding: 0.3em 1em 0;
    }
    .featureTabs::before {
        content: "";
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: -1;
        border-bottom: none;
        border-radius: 0.5em 0.5em 0 0;
        background-color: #e6e6e6;
        box-shadow: 0 1px white inset;
        transform: scale(1.2, 1.3) perspective(0.5em) rotateX(5deg);
        transform-origin: bottom left;
        transition-property: color, background-color, border-color,
        text-decoration-color, fill, stroke, opacity, box-shadow, transform,
        filter, backdrop-filter;
        transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
        transition-duration: 300ms;
    }
    .featureTabs.active {
        color: white;
        z-index: 2;
    }
    .featureTabs.active::before {
        background-color: #2fb6e1;
        border-bottom-width: 1px;
        border-color: #2fb6e1;
    }
</style>


