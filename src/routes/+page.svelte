<script lang="ts">
	import LeftPanel from "$lib/components/LeftPanel.svelte";
	import MainPanel from "$lib/components/MainPanel.svelte";
	import RightPanel from "$lib/components/RightPanel.svelte";

	function handleSend() {
		/* TODO: wire up send action */
	}
</script>

<main class="flex h-screen w-screen p-2 gap-2">
	<LeftPanel />
	<MainPanel onSend={handleSend} />
	<RightPanel />
</main>
