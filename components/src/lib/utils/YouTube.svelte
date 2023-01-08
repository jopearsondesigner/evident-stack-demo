<script lang='ts'>
    export let containerClass: string = 'relative w-100% h-0 pb-[56.25%] my-8';
    export let videoClass: string = 'absolute t-0 l-0 w-full h-full';
    export let src: string;
</script>

<div class={containerClass}>
    <iframe width="100%" height="100%" {src} title="YouTube video player" frameborder="0" class={videoClass} allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>