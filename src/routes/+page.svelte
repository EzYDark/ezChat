<script lang="ts">
	import LeftPanel from "$lib/components/LeftPanel.svelte";
	import MainPanel from "$lib/components/MainPanel.svelte";
	import RightPanel from "$lib/components/RightPanel.svelte";

	let isRightPanelOpen = true;
	function toggleRightPanel() {
		isRightPanelOpen = !isRightPanelOpen;
	}

	let isLeftPanelOpen = true;
	function toggleLeftPanel() {
		isLeftPanelOpen = !isLeftPanelOpen;
	}

	function handleSend() {
		/* TODO: wire up send action */
	}
</script>

<main class="flex h-screen w-screen p-2 gap-2">
	<LeftPanel isLeftPanelOpen={isLeftPanelOpen} onToggleLeftPanel={toggleLeftPanel} />
	<MainPanel isRightPanelOpen={isRightPanelOpen} onToggleRightPanel={toggleRightPanel} isLeftPanelOpen={isLeftPanelOpen} onToggleLeftPanel={toggleLeftPanel} onSend={handleSend} />
	<RightPanel isRightPanelOpen={isRightPanelOpen} onToggleRightPanel={toggleRightPanel} />
</main>
