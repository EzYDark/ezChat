<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="h-screen w-screen bg-[#1a1a1a] flex gap-2 p-2">
    <div class="flex-1 bg-[#9a9a9a]"></div>
    <div class="flex-3 bg-[#673636]"></div>
    <div class="flex-1 bg-[#366742]"></div>
</main>

<style>
</style>
