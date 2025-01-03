<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { UploadCloud } from "lucide-svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { dragState, setIsDragging } from "$lib/store.svelte";

  let unlistenDrop: UnlistenFn;
  let unlistenLeave: UnlistenFn;

  onMount(async () => {
    unlistenDrop = await listen("tauri://drag-drop", async (event) => {
      console.log(event);
      setIsDragging(false);
    });

    unlistenLeave = await listen("tauri://drag-leave", async (e) => {
      setIsDragging(false);
    });
  });

  onDestroy(() => {
    unlistenDrop();
    unlistenLeave();
  });
</script>

<div class="overlay" class:active={dragState.isDragging}>
  <div class="content flex flex-col items-center gap-4">
    <UploadCloud class="size-16 text-gray-700" />
    <p class="text-2xl font-medium">拖动文件到此</p>
    <p class="text-gray-600">松手即可上传</p>
  </div>
</div>

<style lang="postcss">
  .overlay {
    @apply invisible fixed inset-0 z-[9999] flex h-screen w-screen items-center justify-center bg-black/50 opacity-0 backdrop-blur-sm transition-opacity duration-300;
  }

  .overlay.active {
    @apply visible opacity-100;
  }

  .content {
    @apply rounded-xl bg-white/90 p-8 text-center text-xl text-gray-800;
  }
</style>
