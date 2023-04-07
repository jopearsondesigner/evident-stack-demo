<script lang="ts">
    import type { PathCoordinates } from "./types";

	export let color: string;
	export let pathCoords: PathCoordinates;
	export let strokeWidth: number;
	export let dashness: number | undefined;	

	function buildPath({ to, from, fromBezPoint, toBezPoint }: PathCoordinates): string {
		return `M ${from.x} ${from.y}
        C ${fromBezPoint.x} ${fromBezPoint.y},
            ${toBezPoint.x} ${toBezPoint.y},
            ${to.x} ${to.y}`;
	}
	function dashArray(strokeWidth: number, dashness: number | undefined): number | null {
		return dashness ? 6 * strokeWidth : null;
	}
</script>

<path
	id="flow-path"
	d="${buildPath(pathCoords)}"
	stroke={color}
	stroke-width={strokeWidth}
	stroke-dasharray={dashArray(strokeWidth, dashness)}
	fill="none"
	marker-end="url(#arrow)"
/>
