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
	let draftMessage = "";
	let composerTextarea: HTMLTextAreaElement | null = null;
	const composerMaxRows = 7;
	let composerMaxHeight = 0;

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

	function ensureComposerMetrics() {
		if (!composerTextarea || composerMaxHeight) return;

		const styles = getComputedStyle(composerTextarea);
		const fontSize = parseFloat(styles.fontSize) || 16;
		const rawLineHeight = parseFloat(styles.lineHeight);
		const lineHeight = Number.isFinite(rawLineHeight) && rawLineHeight > 0 ? rawLineHeight : fontSize * 1.3;
		const padding =
			parseFloat(styles.paddingTop) +
			parseFloat(styles.paddingBottom) +
			parseFloat(styles.borderTopWidth) +
			parseFloat(styles.borderBottomWidth);

		const verticalSpacing = Number.isFinite(padding) ? padding : 0;
		composerMaxHeight = lineHeight * composerMaxRows + verticalSpacing;
	}

	function resizeComposer() {
		if (!composerTextarea) return;
		if (!composerMaxHeight) ensureComposerMetrics();

		composerTextarea.style.height = "auto";
		const scrollHeight = composerTextarea.scrollHeight;
		const nextHeight =
			composerMaxHeight > 0 ? Math.min(scrollHeight, composerMaxHeight) : scrollHeight;
		composerTextarea.style.height = `${nextHeight}px`;
	}

	function sendMessage() {
		const trimmed = draftMessage.trim();
		if (!trimmed) return;

		console.info("sendMessage not wired to backend yet", trimmed);
		draftMessage = "";
		resizeComposer();
	}

	function onComposerKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			sendMessage();
		}
	}

	$: resizeComposer();
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
		<form
			class="mt-2 mb-2 flex shrink-0 items-end gap-2 rounded-lg border border-border bg-panel p-3 shadow-sm"
			on:submit|preventDefault={sendMessage}
		>
			<button
				type="button"
				class="h-10 w-10 shrink-0 rounded-full border border-border bg-message-background text-sm font-semibold"
				aria-label="Attach file"
			>
				+
			</button>
			<textarea
				bind:this={composerTextarea}
				class="flex-1 resize-none rounded-md border border-border bg-transparent px-3 py-2 text-sm text-foreground focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent/40"
				placeholder="Type a message…"
				bind:value={draftMessage}
				on:input={resizeComposer}
				on:keydown={onComposerKeydown}
				rows="1"
				style={`max-height:${composerMaxHeight ? `${composerMaxHeight}px` : "none"}; overflow-y:auto;`}
			></textarea>
			<button
				type="submit"
				class="h-10 shrink-0 rounded-md bg-accent px-4 text-sm font-semibold text-accent-foreground hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-accent"
			>
				Send
			</button>
		</form>
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
