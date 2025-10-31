<script lang="ts">
    import { Plus, Send, SendHorizonal } from "@lucide/svelte";
	import { onMount } from "svelte";

	export let placeholder = "Type your message...";
	export let maxHeight = 160;
	export let onSend: () => void = () => {};

	let messageInput: HTMLTextAreaElement;

	function autoResize() {
		if (!messageInput) return;
		messageInput.style.height = "auto";
		const next = Math.min(messageInput.scrollHeight, maxHeight);
		messageInput.style.height = `${next}px`;
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			onSend();
		}
	}

	onMount(autoResize);
</script>

<div class="flex items-center bg-panel overflow-x-hidden border border-border">
	<div
		class="w-11 h-10 mx-5 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer"
	>
		<Plus
			size="28"
			strokeWidth=1
		/>
	</div>
	<textarea
		bind:this={messageInput}
		class="w-full my-2 resize-none border border-border2 custom-scrollbar radius"
		style="height:1rem; min-height:1rem; max-height:16rem; padding:0.5rem; outline: none;"
		{placeholder}
		on:input={autoResize}
		on:keydown={handleKeydown}
	></textarea>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="w-11 h-10 mx-5 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer"
		on:click={onSend}
	>
		<SendHorizonal size="22" strokeWidth=1 />
	</div>
</div>
