<script lang="ts">
  import { checkClipboardContent } from "$lib/tools";
  import type { File, UploadProgress } from "$lib/type";
  import { Eye, GripVertical, UploadCloud, X } from "lucide-svelte";
  import { onDestroy, onMount } from "svelte";
  import FileUploaderReady from "./FileUploaderReady.svelte";
  import FileUploaderPreview from "./FileUploaderPreview.svelte";
  import FileUploader from "./FileUploader.svelte";
  import FileUploaderProgress from "./FileUploaderProgress.svelte";
  import { globalState } from "$lib/store.svelte";

  onMount(async () => {
    window.addEventListener("keydown", handleKeyDown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeyDown);
  });

  // 监听 ctrl+v
  function handleKeyDown(e: KeyboardEvent) {
    // 如果当前焦点是输入框，则不处理
    if (
      e.target instanceof HTMLInputElement ||
      e.target instanceof HTMLTextAreaElement
    ) {
      return;
    }

    // 处理 ctrl+v
    if ((e.ctrlKey || e.metaKey) && e.key === "v") {
      console.log("ctrl+v");
      e.preventDefault();
      checkClipboardContent();
    }
  }
</script>

<div
  class="flex min-h-0 flex-1 flex-col items-center justify-center rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
>
  {#if !globalState.selectedBucket}
    <p class="dark:text-slate-300">您尚未设置存储桶，无法操作</p>
  {:else if !globalState.files.length}
    <FileUploaderReady />
  {:else if !globalState.isUploading}
    <FileUploader />
  {:else}
    <FileUploaderProgress />
  {/if}
</div>
