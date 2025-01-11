<script lang="ts">
  import { showModal } from "$lib/store.svelte";
  import {
    checkClipboardContent,
    openDialogForDir,
    openDialogForFiles,
  } from "$lib/tools";
  import { UploadCloud } from "lucide-svelte";
  import AddTextContent from "./AddTextContent.svelte";
</script>

<div class="flex items-center justify-center gap-12">
  <UploadCloud class="hidden size-32 text-slate-400 sm:block" />
  <div class="flex flex-1 flex-col items-start gap-3">
    <p class="text-slate-500 dark:text-slate-300">
      您的存储桶已就绪，拖放文件到此，或：
    </p>
    <div class="grid grid-cols-2 gap-2">
      <button onclick={openDialogForFiles} class="button button-primary"
        >选择文件</button
      >
      <button onclick={openDialogForDir} class="button button-primary"
        >选择文件夹</button
      >
      <button onclick={checkClipboardContent} class="button button-primary"
        >选择剪贴板</button
      >
      <button onclick={() => showModal(text)} class="button button-primary"
        >选择新建文本</button
      >
    </div>
  </div>
</div>

{#snippet text()}
  <AddTextContent />
{/snippet}
