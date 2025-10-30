<script lang="ts">
	import { onDestroy } from "svelte";
	import ResizableHandle from "$lib/components/ResizableHandle.svelte";

	export let initialWidth = 260;
	export let minWidth = 160;
	export let handleThickness = 2;
	export let onResize: (width: number) => void = () => {};
	export let onResizeEnd: (width: number) => void = () => {};

	let width = initialWidth;
	let startX = 0;
	let startWidth = width;
	let isDragging = false;

	function onPointerDown(event: PointerEvent) {
		event.preventDefault();
		isDragging = true;
		startX = event.clientX;
		startWidth = width;
		window.addEventListener("pointermove", onPointerMove);
		window.addEventListener("pointerup", onPointerUp, { once: true });
	}

	function onPointerMove(event: PointerEvent) {
		if (!isDragging) return;
		const delta = event.clientX - startX;
		width = Math.max(minWidth, startWidth - delta);
		onResize(width);
	}

	function onPointerUp() {
		if (!isDragging) return;
		isDragging = false;
		window.removeEventListener("pointermove", onPointerMove);
		onResizeEnd(width);
	}

	onDestroy(() => {
		window.removeEventListener("pointermove", onPointerMove);
		window.removeEventListener("pointerup", onPointerUp);
	});
</script>

<div class="flex h-full shrink-0">
	<ResizableHandle
		orientation="vertical"
		thickness={handleThickness}
		onPointerDown={onPointerDown}
	/>
	<div
		class="h-full bg-panel border border-border"
		style={`width:${width}px;`}
	>
		<slot />
	</div>
</div>
