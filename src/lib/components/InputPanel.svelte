<script lang="ts">
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
		class="w-11 h-10 mx-5 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="w-9 h-9 text-white"
			viewBox="0 0 24 24"
			fill="none"
			stroke="#ffffff"
			stroke-width="1"
			stroke-linecap="round"
			stroke-linejoin="round"
			><path d="M5 12h14" /><path d="M12 5v14" /></svg
		>
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
		class="w-11 h-10 mx-5 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600"
		on:click={onSend}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="w-7 h-7 text-white"
			viewBox="0 0 24 24"
			fill="none"
			stroke="#ffffff"
			stroke-width="1"
			stroke-linecap="round"
			stroke-linejoin="round"
			><path
				d="M3.714 3.048a.498.498 0 0 0-.683.627l2.843 7.627a2 2 0 0 1 0 1.396l-2.842 7.627a.498.498 0 0 0 .682.627l18-8.5a.5.5 0 0 0 0-.904z"
			/><path d="M6 12h16" /></svg
		>
	</div>
</div>
