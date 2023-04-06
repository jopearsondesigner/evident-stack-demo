<script lang="ts">
	import { onDestroy } from 'svelte';
	import FlowMarker from './FlowMarker.svelte';
	import type { Id, PathProps } from './types';

	export let config: PathProps;

	const rectForId = (id: Id): DOMRect | undefined => {
		const element = document.getElementById(id);
		if (element) {
			return element.getBoundingClientRect();
		}
	};

	const rectsUpdated = (current: DOMRect, next: DOMRect): boolean => {
		return (
			current.x !== next.x ||
			current.y !== next.y ||
			current.width !== next.width ||
			current.height !== next.height
		);
	};

	let toRect: DOMRect | undefined = rectForId(config.to);
	let fromRect: DOMRect | undefined = rectForId(config.from);

	let updateLoop = setInterval(() => {
		const nextTo = rectForId(config.to);
		const nextFrom = rectForId(config.from);

        if (!nextTo || !nextFrom) {
            return;
        }

        if (!toRect || !fromRect) {
            toRect = nextTo;
            fromRect = nextFrom;
            update();
        } else if (rectsUpdated(toRect, nextTo) || rectsUpdated(fromRect, nextFrom)) {
            toRect = nextTo;
            fromRect = nextFrom;
            update();
        }
	}, config.refreshTime);

    let update = () => {
        console.info(`UPDATE: TO: ${config.to}, FROM: ${config.from}`)
        console.info("Bounding To: ", toRect);
        console.info("Bounding From: ", fromRect);
    }

	onDestroy(() => {
		clearInterval(updateLoop);
	});
</script>

<svg width="100%" height="100%" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg">
	<defs>
        <FlowMarker color={config.color} size={config.markerSize} />
    </defs>
</svg>
<!-- <svg width="100%" height="100%" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg">
    <defs>{ marker }</defs>
    { debugOverlay } { pathCoords }
</svg> -->
