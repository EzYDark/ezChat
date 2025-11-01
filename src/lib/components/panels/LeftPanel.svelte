<script lang="ts">
    import { onDestroy } from "svelte";
    import ResizableHandle from "$lib/components/ui/ResizableHandle.svelte";
    import ChatListItem from "$lib/components/chat/ChatListItem.svelte";
    import { ArrowLeftFromLine, SettingsIcon } from "@lucide/svelte";

    export let initialWidth = 280;
    export let minWidth = 220;
    export let handleThickness = 2;
    export let onResize: (width: number) => void = () => {};
    export let onResizeEnd: (width: number) => void = () => {};

    export let isLeftPanelOpen: boolean = false;
    export let onToggleLeftPanel: () => void = () => {};

    let width = initialWidth;
    let startX = 0;
    let startWidth = width;
    let isDragging = false;

    type ChatListEntry = {
        id: number;
        avatarSrc: string;
        title: string;
        preview: string;
        timestamp: string;
    };

    const chatItems: ChatListEntry[] = [
        {
            id: 1,
            avatarSrc: "/images/cat.png",
            title: "Chat",
            preview: "Text of the chat...",
            timestamp: "12m",
        },
        {
            id: 2,
            avatarSrc: "/images/cat.png",
            title: "Chat",
            preview: "Text of the chat...",
            timestamp: "12m",
        },
        {
            id: 3,
            avatarSrc: "/images/cat.png",
            title: "Chat",
            preview: "Text of the chat...",
            timestamp: "12m",
        },
        {
            id: 4,
            avatarSrc: "/images/cat.png",
            title: "Chat",
            preview: "Text of the chat...",
            timestamp: "12m",
        },
        {
            id: 5,
            avatarSrc: "/images/cat.png",
            title: "Chat",
            preview: "Text of the chat...",
            timestamp: "12m",
        },
        {
            id: 6,
            avatarSrc: "/images/cat.png",
            title: "Chat",
            preview: "Text of the chat...",
            timestamp: "12m",
        },
    ];

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
        width = Math.max(minWidth, startWidth + delta);
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
    {#if isLeftPanelOpen}
    <div
        class="h-full bg-panel border border-border flex flex-col gap-2"
        style={`width:${width}px;`}
    >
        <div class="mx-4 h-16 flex items-center justify-between">
            <h1 class="text-[20px] font-bold">ezChat</h1>
            <button class="w-10 h-10 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer aspect-square" onclick={onToggleLeftPanel}>
                <ArrowLeftFromLine size="20" strokeWidth=1 />
            </button>
        </div>
        <div
            class="bg-message-top w-[calc(100%-24px)] h-14 self-center border border-border2 mb-2"
        >
            <input
                class="w-full h-full outline-0 px-2"
                placeholder="Search user or chat..."
                type="text"
            />
        </div>
        <div class="flex-1 overflow-auto space-y-2">
            {#each chatItems as chat (chat.id)}
                <ChatListItem
                    avatarSrc={chat.avatarSrc}
                    title={chat.title}
                    preview={chat.preview}
                    timestamp={chat.timestamp}
                />
            {/each}
        </div>
        <div class="w-10 h-10 flex justify-center items-center rounded-md hover:bg-neutral-700 active:bg-neutral-600 cursor-pointer ml-4 mb-4 mt-auto">
            <SettingsIcon size="20" strokeWidth=1 />
        </div>
    </div>
    <ResizableHandle
        orientation="vertical"
        thickness={handleThickness}
        {onPointerDown}
    />
    {/if}
</div>
