<script lang="ts">
    export let avatarSrc: string;
    export let title: string;
    export let preview: string;
    export let timestamp: string;
</script>

<div class="flex w-full h-16 items-center gap-3 px-2 hover:bg-neutral-700 active:bg-neutral-600">
    <div class="w-14 h-14 border border-border aspect-square">
        <img src={avatarSrc} alt="" class="w-full h-full object-cover" />
    </div>
    <div class="flex flex-col flex-1">
        <p>{title}</p>
        <p class="text-xs text-message-top-text truncate">{preview}</p>
    </div>
    <div class="text-right shrink-0 text-xs text-message-top-text mr-2">
        <p class="text-xs text-message-top-text">{timestamp}</p>
    </div>
</div>
