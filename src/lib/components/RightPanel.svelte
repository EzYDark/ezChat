<script lang="ts">
	import { onDestroy } from "svelte";
	import ResizableHandle from "$lib/components/ResizableHandle.svelte";
    import { ArrowDown, ArrowRightFromLine, ChevronDown, ChevronRight } from "@lucide/svelte";

	export let isRightPanelOpen: boolean = false;
	export let onToggleRightPanel: () => void = () => {};

	export let initialWidth = 260;
	export let minWidth = 160;
	export let handleThickness = 2;
	export let onResize: (width: number) => void = () => {};
	export let onResizeEnd: (width: number) => void = () => {};

	let sharedFilesOpen = true;

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

{#if isRightPanelOpen}
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
		<div class="mx-4 h-16 flex items-center jsutify-center">
			<div class="flex-1">
				<button class="w-10 h-10 flex-1 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer aspect-square" onclick={onToggleRightPanel}>
					<ArrowRightFromLine size="20" strokeWidth=1/>
				</button>
			</div>
			<h1 class="flex-1 text-[16px] font-bold text-center">Options</h1>
			<div class="flex-1"></div>
		</div>
		<div class="bg-message-top flex flex-col gap-2">
			<button class="flex w-full justify-between px-4 py-3 hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer" onclick={() => sharedFilesOpen = !sharedFilesOpen}>
				<p>Shared Files</p>
				<div class="w-5 h-5 flex justify-center items-center">
					{#if sharedFilesOpen}
						<ChevronDown size="20" strokeWidth=1 />
					{:else}
						<ChevronRight size="20" strokeWidth=1 />
					{/if}
				</div>
			</button>
			{#if sharedFilesOpen}
			<div class="flex-1 grid grid-cols-2 gap-2 overflow-y-auto p-2">
				<img src="/images/cat.png" alt="File 1" class="w-full h-full object-cover aspect-square border border-border2" />
				<img src="/images/screenshot_test.png" alt="File 2" class="w-full h-full object-cover aspect-square border border-border2" />
				<img src="/images/screenshot_test2.png" alt="File 3" class="w-full h-full object-cover aspect-square border border-border2 " />
			</div>
			{/if}
		</div>
	</div>
</div>
{/if}
