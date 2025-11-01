<script lang="ts">
	import LeftPanel from "$lib/components/LeftPanel.svelte";
	import MainPanel from "$lib/components/MainPanel.svelte";
	import RightPanel from "$lib/components/RightPanel.svelte";

	let isRightPanelOpen = true;
	function toggleRightPanel() {
		isRightPanelOpen = !isRightPanelOpen;
	}

	function handleSend() {
		/* TODO: wire up send action */
	}
</script>

<main class="flex h-screen w-screen p-2 gap-2">
	<LeftPanel />
	<MainPanel isRightPanelOpen={isRightPanelOpen} onToggleRightPanel={toggleRightPanel} onSend={handleSend} />
	<RightPanel isRightPanelOpen={isRightPanelOpen} onToggleRightPanel={toggleRightPanel} />
</main>
