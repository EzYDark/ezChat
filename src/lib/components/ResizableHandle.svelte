<script lang="ts">
	export type Orientation = "vertical" | "horizontal";

	export let orientation: Orientation = "vertical";
	export let thickness = 2;
	export let onPointerDown: (event: PointerEvent) => void = () => {};

	const sizeClasses =
		orientation === "vertical"
			? `h-full cursor-col-resize`
			: `w-full cursor-row-resize`;
</script>

<div
	class={`bg-transparent ${sizeClasses}`}
	style={orientation === "vertical"
		? `width:${thickness}px;`
		: `height:${thickness}px;`}
	role="separator"
	on:pointerdown={onPointerDown}
/>
