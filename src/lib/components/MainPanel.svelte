<script lang="ts">
	import MessageBubble from "$lib/components/MessageBubble.svelte";
	import Seperator from "$lib/components/Seperator.svelte";
	import InputPanel from "$lib/components/InputPanel.svelte";
    import { ArrowLeftFromLine } from "@lucide/svelte";

	export let onSend: () => void = () => {};
</script>

<div class="flex flex-col flex-1">
	<div class="h-full flex flex-col flex-1 gap-2 overflow-y-auto custom-scrollbar relative">
		<div class="absolute right-2 top-1 flex">
			<div class="w-10 h-10 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer">
				<ArrowLeftFromLine size="20" strokeWidth="1"/>
			</div>
		</div>
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
	<InputPanel onSend={onSend} />
</div>
