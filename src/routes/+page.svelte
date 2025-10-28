<script lang="ts">
	let leftWidth = 280;
	let rightWidth = 260;
	const minWidth = 160;

	let activeHandle: 'left' | 'right' | null = null;
	let startX = 0;
	let startLeft = leftWidth;
	let startRight = rightWidth;

	function onPointerDown(handle: 'left' | 'right', event: PointerEvent) {
		activeHandle = handle;
		startX = event.clientX;
		startLeft = leftWidth;
		startRight = rightWidth;
		window.addEventListener('pointermove', onPointerMove);
		window.addEventListener('pointerup', onPointerUp, { once: true });
	}

	function onPointerMove(event: PointerEvent) {
		if (!activeHandle) return;
		const delta = event.clientX - startX;

		if (activeHandle === 'left') {
			leftWidth = Math.max(minWidth, startLeft + delta);
		} else {
			rightWidth = Math.max(minWidth, startRight - delta);
		}
	}

	function onPointerUp() {
		activeHandle = null;
		window.removeEventListener('pointermove', onPointerMove);
	}
</script>

<main class="box-border flex h-screen w-screen bg-[#1a1a1a] p-2">
	<div
		class="h-full shrink-0 bg-[#9a9a9a]"
		style={`width:${leftWidth}px;`}
	></div>

	<div
		class="h-full w-2 cursor-col-resize bg-transparent"
		role="separator"
		on:pointerdown={(event) => onPointerDown('left', event)}
	></div>

	<div class="h-full min-w-0 flex-1 bg-[#673636]"></div>

	<div
		class="h-full w-2 cursor-col-resize bg-transparent"
		role="separator"
		on:pointerdown={(event) => onPointerDown('right', event)}
	></div>

	<div
		class="h-full shrink-0 bg-[#366742]"
		style={`width:${rightWidth}px;`}
	></div>
</main>
