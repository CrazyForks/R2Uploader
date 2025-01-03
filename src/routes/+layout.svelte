<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Alert from "$lib/components/Alert.svelte";
  import FileDrag from "$lib/components/FileDrag.svelte";
  let { children } = $props();
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import { setIsDragging } from "$lib/store.svelte";

  let unlisten: UnlistenFn;

  onMount(async () => {
    unlisten = await listen("tauri://drag-enter", async (event) => {
      setIsDragging(true);
    });
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
  });
</script>

<FileDrag />
<Alert />

<div class="flex h-screen bg-slate-50 dark:bg-slate-900">
  <Sidebar />
  <main class="flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>
