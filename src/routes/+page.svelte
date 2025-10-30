<script lang="ts">
	import Seperator from "$lib/components/Seperator.svelte";
	import { onMount } from "svelte";
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

	const MAX_HEIGHT = 160;
	let messageInput: HTMLTextAreaElement;

	function autoResize() {
		if (!messageInput) return;
		messageInput.style.height = "auto";
		const next = Math.min(messageInput.scrollHeight, MAX_HEIGHT);
		messageInput.style.height = `${next}px`;
	}

	onMount(autoResize);
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
		<div
			class="flex items-center bg-panel overflow-x-hidden border border-border"
		>
			<div
				class="w-11 h-10 mx-5 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600"
			>
				<svg
					class="w-8 h-8 text-white"
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<path
						stroke="currentColor"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1"
						d="M5 12h14m-7 7V5"
					/>
				</svg>
			</div>
			<textarea
				bind:this={messageInput}
				class="w-full my-2 resize-none border border-neutral-900 custom-scrollbar radius"
				style="height:1rem; min-height:1rem; max-height:16rem; padding:0.5rem; outline: none;"
				placeholder="Type your message..."
				on:input={autoResize}
			></textarea>
			<div
				class="w-11 h-10 mx-5 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600"
			>
				<svg
					class="w-7 h-7 text-white rotate-90"
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<path
						stroke="currentColor"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1"
						d="m12 18-7 3 7-18 7 18-7-3Zm0 0v-5"
					/>
				</svg>
			</div>
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
