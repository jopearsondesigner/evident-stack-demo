<script lang='ts'>
    import classNames from 'classnames';
    export let badgeClass: string = 'uppercase absolute pl-2 pr-2 px-2 mt-4 bg-black bg-opacity-90 text-gray-primary text-sm';
    export let borderClass: string = 'border-t border-r border-b border-gray-primary';
    export let divClass: string = '';
</script>

<div class={classNames(badgeClass, borderClass, divClass)}>Featured</div>