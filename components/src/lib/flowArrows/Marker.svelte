<script lang="ts">
    export let uuid: string;
    export let color: string;
    export let size: number;

    interface MarkerGeometry {
        refX: number,
        refY: number,
        markerWidth: number,
        markerHeight: number,
        viewBox: string,
        path: string
    }

    const markerGeometry = (size: number): MarkerGeometry => {
        const halfSize = Math.ceil(size / 2);
        return {
            refX: halfSize,
            refY: 0,
            markerWidth: size,
            markerHeight: size,
            viewBox: `0 -${halfSize} ${size} ${size}`,
            path: `M0,-${halfSize} L${size},0L0,${halfSize}`,
        };
    };

    const { refX, refY, markerWidth, markerHeight, viewBox, path } = markerGeometry(size);
</script>

<marker id={"arrow" + uuid}>
    viewBox={viewBox}
    refX={refX}
    refY={refY}
    markerWidth={markerWidth}
    markerHeight={markerHeight}
    orient="auto"
    fill={color}>
          <path d={path}></path>
</marker>