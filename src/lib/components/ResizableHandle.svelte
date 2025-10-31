<script lang="ts">
	type Orientation = "vertical" | "horizontal";

	export let orientation: Orientation = "vertical";
	export let thickness = 2;
	export let onPointerDown: (event: PointerEvent) => void = () => {};

	$: sizeClasses =
		orientation === "vertical"
			? "h-full cursor-col-resize"
			: "w-full cursor-row-resize";
</script>

<div
		class={`bg-transparent ${sizeClasses}`}
	style={orientation === "vertical"
		? `width:${thickness}px;`
		: `height:${thickness}px;`}
	role="separator"
	on:pointerdown={onPointerDown}
>
</div>
