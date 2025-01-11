<script lang="ts">
  import { globalState, setAlert } from "$lib/store.svelte";
  import type { UploadProgress } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { LinkPreview } from "bits-ui";
  import { Eye, GripVertical, UploadCloud, X } from "lucide-svelte";
  import { onMount, tick } from "svelte";
  import { dragHandle, dragHandleZone } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import FileUploaderReady from "./FileUploaderReady.svelte";
  import FileUploaderPreview from "./FileUploaderPreview.svelte";

  let oldPrefix = $state("");
  let prefix = $state("");
  const flipDurationMs = 200;

  function handleSort(e: CustomEvent) {
    globalState.files = e.detail.items;
  }

  async function onChangePrefix() {
    await tick();
    globalState.files.forEach((file) => {
      if (file.remoteFilenamePrefix === oldPrefix) {
        file.remoteFilenamePrefix = prefix;
      }
    });
    oldPrefix = prefix;
  }

  async function uploadFile() {
    if (!globalState.selectedBucket) return;
    try {
      globalState.isUploading = true;

      const filesToUpload = globalState.files.map((file) => ({
        id: file.id,
        source: file.source,
        remoteFilename:
          file.remoteFilenamePrefix === ""
            ? file.remoteFilename
            : `${file.remoteFilenamePrefix}/${file.remoteFilename}`,
      }));

      await invoke("r2_upload", {
        bucketName: globalState.selectedBucket.value.bucketName,
        accountId: globalState.selectedBucket.value.accountId,
        accessKey: globalState.selectedBucket.value.accessKey,
        secretKey: globalState.selectedBucket.value.secretKey,
        files: filesToUpload,
      });
    } catch (error: unknown) {
      console.error(error);
      setAlert("上传失败，请重试");
      globalState.isUploading = false;
    }
  }

  function removeFile(index: number) {
    globalState.files.splice(index, 1);
  }
</script>

<div class="flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-lg">
  <!-- 功能条 -->
  <div
    class="flex items-center gap-2 rounded-t-lg bg-slate-200 p-1 shadow backdrop-blur-sm dark:bg-slate-700/80"
  >
    <input
      bind:value={prefix}
      oninput={onChangePrefix}
      class="input w-36"
      placeholder="全局路径"
    />
    <div class="flex-1"></div>
    <button
      onclick={() => (globalState.files = [])}
      class="cursor-pointer rounded-md border px-2 text-sm text-cyan-500"
      >清空</button
    >
    <button
      onclick={uploadFile}
      class="cursor-pointer rounded-md bg-cyan-500 px-6 text-white hover:bg-cyan-400"
      >上传</button
    >
  </div>
  <section
    use:dragHandleZone={{ items: globalState.files, flipDurationMs }}
    onconsider={handleSort}
    onfinalize={handleSort}
    class="flex-1 space-y-2 overflow-y-auto p-2 dark:text-slate-100"
  >
    {#each globalState.files as file, index (file.id)}
      <div
        class="flex items-center gap-4 rounded-md bg-slate-50/80 p-2 shadow-sm backdrop-blur-sm transition-all hover:shadow-md dark:bg-slate-700/80"
        animate:flip={{ duration: flipDurationMs }}
      >
        <div
          use:dragHandle
          class=" text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
        >
          <GripVertical class="size-4" />
        </div>
        <div class="flex-1">
          <div class="flex items-center gap-2">
            <input
              bind:value={file.remoteFilenamePrefix}
              class="input w-24"
              placeholder="远程路径"
            />
            <span class="text-slate-400">/</span>
            <input
              bind:value={file.remoteFilename}
              class="input flex-1"
              placeholder="远程文件名"
            />
          </div>
        </div>
        <div>
          <FileUploaderPreview {file} />
          <button onclick={() => removeFile(index)} class="action-button">
            <X class="size-4" />
          </button>
        </div>
      </div>
    {/each}
  </section>
</div>

<style lang="postcss">
  .input {
    @apply rounded-md bg-white/80 px-2 py-1 text-sm backdrop-blur-sm transition-all placeholder:text-slate-400/60 focus:ring-2 focus:ring-cyan-500/50 focus:outline-none dark:bg-slate-700/80 dark:placeholder:text-slate-400/50 dark:focus:ring-cyan-500/30;
  }

  .action-button {
    @apply cursor-pointer rounded-md p-1 text-slate-500 backdrop-blur-sm transition-all hover:bg-slate-200/50 hover:shadow-sm dark:text-slate-400 dark:hover:bg-slate-600/50;
  }
</style>
