<script lang="ts">
    export let sender: string;
    export let text: string;
    export let isUser: boolean = false;
    export let timestamp: string = new Date().toDateString();
    export let attachments: { src: string; alt: string }[] = [];
</script>

<div class="max-w-120 w-fit min-w-50 {isUser ? 'self-end' : 'self-start'}">
    <div
        class="w-full border-t border-l border-r border-border p-1 text-[12px] bg-message-top text-message-top-text"
    >
        <p>{sender}</p>
    </div>
    <div class="w-full border border-border p-1 bg-message-background">
        {#if attachments.length > 0}
            <div class="grid grid-cols-2 gap-2 mb-2">
                {#each attachments as attachment}
                    <div
                        class="w-full h-full bg-message-background aspect-square"
                    >
                        <img
                            src={attachment.src}
                            alt={attachment.alt}
                            class="w-full h-full object-cover"
                        />
                    </div>
                {/each}
            </div>
        {/if}
        <p>{text}</p>
    </div>
    <div class="w-full text-[10px] mt-1 ml-1 text-message-top-text">
        <p>{timestamp}</p>
    </div>
</div>
