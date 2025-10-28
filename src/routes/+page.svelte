<script lang="ts">
	import Seperator from "$lib/components/Seperator.svelte";
	import MessageBubble from "../lib/components/MessageBubble.svelte";

	let leftWidth = 280;
	let rightWidth = 260;
	const minWidth = 160;

	let activeHandle: "left" | "right" | null = null;
	let startX = 0;
	let startLeft = leftWidth;
	let startRight = rightWidth;

	function onPointerDown(handle: "left" | "right", event: PointerEvent) {
		activeHandle = handle;
		startX = event.clientX;
		startLeft = leftWidth;
		startRight = rightWidth;
		window.addEventListener("pointermove", onPointerMove);
		window.addEventListener("pointerup", onPointerUp, { once: true });
	}

	function onPointerMove(event: PointerEvent) {
		if (!activeHandle) return;
		const delta = event.clientX - startX;

		if (activeHandle === "left") {
			leftWidth = Math.max(minWidth, startLeft + delta);
		} else {
			rightWidth = Math.max(minWidth, startRight - delta);
		}
	}

	function onPointerUp() {
		activeHandle = null;
		window.removeEventListener("pointermove", onPointerMove);
	}
</script>

<main class="flex h-screen w-screen p-2">
	<div
		class="h-full shrink-0 bg-panel border border-border"
		style={`width:${leftWidth}px;`}
	></div>

	<div
		class="h-full w-2 cursor-col-resize"
		role="separator"
		on:pointerdown={(event) => onPointerDown("left", event)}
	></div>

	<div class="flex flex-col flex-1">
		<div
			class="h-full flex flex-col flex-1 gap-2 overflow-y-auto custom-scrollbar"
		>
			<MessageBubble
				sender="Kheper"
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
				attachments={[
					{ src: "/images/cat.png", alt: "Attachment 1" },
					{ src: "/images/screenshot_test.png", alt: "Attachment 2" },
					{
						src: "/images/screenshot_test2.png",
						alt: "Attachment 3",
					},
					{ src: "/images/cat.png", alt: "Attachment 4" },
				]}
			/>
			<Seperator timestamp="2025-01-01 12:00:00" />
			<MessageBubble
				sender="EzY"
				isUser={true}
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
			<MessageBubble
				sender="Kheper"
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
			<MessageBubble
				sender="EzY"
				isUser={true}
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
			<Seperator timestamp="2025-01-01 12:00:00" />
			<MessageBubble
				sender="Kheper"
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
			<MessageBubble
				sender="EzY"
				isUser={true}
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
			<MessageBubble
				sender="Kheper"
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
			<MessageBubble
				sender="EzY"
				isUser={true}
				text="Hello. I'm Kheper... How can I help you today? Is there anything specific you'd like to discuss? I'm here to assist you with a wide range of topics and questions. Feel free to ask me anything!"
			/>
		</div>
		<div class="h-10 bg-panel border border-border">
			<input
				type="text"
				class="w-full h-full bg-transparent border-none outline-none"
			/>
		</div>
	</div>

	<div
		class="h-full w-2 cursor-col-resize"
		role="separator"
		on:pointerdown={(event) => onPointerDown("right", event)}
	></div>

	<div
		class="h-full shrink-0 bg-panel border border-border"
		style={`width:${rightWidth}px;`}
	></div>
</main>
