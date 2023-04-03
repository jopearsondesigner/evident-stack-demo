<script lang="ts">
    export let marker: string;
    export let debugOverlay: string;
    export let pathCoords: string;
</script>
<svg width="100%" height="100%" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg">
    <defs>{ marker }</defs>
    { debugOverlay } { pathCoords }
</svg>