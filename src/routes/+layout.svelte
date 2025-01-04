<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Alert from "$lib/components/Alert.svelte";
  import FileDrag from "$lib/components/FileDrag.svelte";
  let { children } = $props();
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import {
    appSettings,
    initAppSettings,
    setIsDragging,
  } from "$lib/store.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import db from "$lib/db";

  let unlisten: UnlistenFn;

  onMount(async () => {
    // initialize settings on load
    initAppSettings();

    unlisten = await listen("tauri://drag-enter", async (event) => {
      setIsDragging(true);
    });
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
  });

  $inspect(appSettings.locale)

  $effect(() => {
    db.appSettings.put({
      id: 1,
      ...$state.snapshot(appSettings),
    });
  });
</script>

<FileDrag />
<Alert />
<Modal />

<div class="flex h-screen bg-slate-50 dark:bg-slate-900">
  <Sidebar />
  <main class="flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>
