<script lang="ts">
  import { bucketsState, filesState } from "$lib/files.svelte";
  import {
    dragState,
    setAlert,
    setDragPaths,
    showModal,
  } from "$lib/store.svelte";
  import { checkClipboardContent, parsePaths } from "$lib/tools";
  import type { File } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { LinkPreview } from "bits-ui";
  import { Eye, GripVertical, UploadCloud, X } from "lucide-svelte";
  import { onDestroy, onMount, tick } from "svelte";
  import { dragHandle, dragHandleZone } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import AddTextContent from "./AddTextContent.svelte";

  let {
    uploadStatus = $bindable("idle"),
    uploadStatusMap = $bindable({}),
    intervalId = $bindable<number | undefined>(),
  }: {
    uploadStatus?: "idle" | "uploading" | "success" | "error";
    uploadStatusMap?: Record<string, string>;
    intervalId?: number | undefined;
  } = $props();

  let oldPrefix = $state("");
  let prefix = $state("");
  const flipDurationMs = 200;

  // 预览相关状态
  let previewContent = $state<string | null>(null);
  let previewLoading = $state(false);
  let previewError = $state<string | null>(null);

  onMount(() => {
    window.addEventListener("keydown", handleKeyDown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeyDown);
  });

  $effect(() => {
    if (dragState.paths.length > 0) {
      parsePaths(dragState.paths);
      setDragPaths([]);
    }
  });

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
      e.preventDefault();
      checkClipboardContent();
    }
  }

  // 预览文件
  async function previewFile(file: File) {
    previewLoading = true;
    previewError = null;
    try {
      if ("filePath" in file.source) {
        const path = file.source.filePath;
        previewContent = await invoke<string>("preview_file", { path });
      } else {
        previewContent = file.source.fileContent || "";
      }
    } catch (error) {
      previewError = error instanceof Error ? error.message : "预览失败";
      setAlert(previewError);
    } finally {
      previewLoading = false;
    }
  }

  function handleSort(e: CustomEvent) {
    filesState.files = e.detail.items;
  }

  async function onChangePrefix() {
    await tick();
    filesState.files.forEach((file) => {
      if (file.remoteFilenamePrefix === oldPrefix) {
        file.remoteFilenamePrefix = prefix;
      }
    });
    oldPrefix = prefix;
  }

  async function checkUploadStatus() {
    try {
      const status = await invoke<Record<string, string>>("get_upload_status");
      uploadStatusMap = status;

      if (Object.keys(status).length === 0) {
        clearInterval(intervalId);
        intervalId = undefined;
        uploadStatus = "success";
        setAlert("上传成功");
        // files = [];

        //
        //
        //
      }
    } catch (error) {
      console.error("获取上传状态失败：", error);
    }
  }

  async function uploadFile() {
    if (!bucketsState.selected) return;
    try {
      uploadStatus = "uploading";

      const filesToUpload = filesState.files.map((file) => ({
        id: file.id,
        source: file.source,
        remoteFilename: `${file.remoteFilenamePrefix}/${file.remoteFilename}`,
      }));

      await invoke("r2_upload", {
        bucketName: bucketsState.selected.value.bucketName,
        accountId: bucketsState.selected.value.accountId,
        accessKey: bucketsState.selected.value.accessKey,
        secretKey: bucketsState.selected.value.secretKey,
        files: filesToUpload,
      });

      if (!intervalId) {
        intervalId = window.setInterval(checkUploadStatus, 500);
      }
    } catch (error: unknown) {
      console.error(error);
      uploadStatus = "error";
      setAlert("上传失败，请重试");
    }
  }

  async function openDialogForFiles() {
    const dialogFiles = await open({
      multiple: true,
      directory: false,
    });
    if (dialogFiles) {
      await parsePaths(dialogFiles);
    }
  }

  async function openDialogForDir() {
    const dialogFiles = await open({
      multiple: true,
      directory: true,
    });
    if (dialogFiles) {
      await parsePaths(dialogFiles);
    }
  }

  function removeFile(index: number) {
    filesState.files.splice(index, 1);
  }
</script>

<div
  class="flex min-h-0 flex-1 flex-col items-center justify-center rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
>
  {#if !bucketsState.selected}
    <p class="dark:text-slate-300">您尚未设置存储桶，无法操作</p>
  {:else if !filesState.files.length}
    {@render ready()}
  {:else}
    {@render uploader()}
  {/if}
</div>

{#snippet text()}
  <AddTextContent />
{/snippet}

{#snippet preview(file: File)}
  <LinkPreview.Root
    openDelay={100}
    closeDelay={100}
    onOpenChange={(isOpen) => {
      if (isOpen) {
        previewFile(file);
      } else {
        previewContent = null;
      }
    }}
  >
    <LinkPreview.Trigger>
      <button class="action-button">
        <Eye class="size-4" />
      </button>
    </LinkPreview.Trigger>
    <LinkPreview.Content side="top">
      <div
        class="w-80 space-y-2 rounded-lg bg-slate-50/90 p-2 shadow-md backdrop-blur-sm dark:bg-slate-900/80"
      >
        <div class="flex items-center justify-center">
          {#if previewLoading}
            <div class="flex items-center justify-center p-2">
              <div
                class="size-8 animate-spin rounded-full border-b-2 border-slate-900"
              ></div>
            </div>
          {:else if previewError}
            <div class="text-sm text-red-500">{previewError}</div>
          {:else if previewContent}
            {#if file.type === "image"}
              <img
                src={previewContent}
                alt="文件预览"
                class="max-h-48 max-w-48 rounded-md object-contain"
              />
            {:else if file.type === "text"}
              <div
                class="max-h-48 overflow-auto rounded-md bg-slate-100/50 p-2 text-sm dark:bg-slate-700/50"
              >
                <p>{previewContent}</p>
              </div>
            {/if}
          {/if}
        </div>

        <div class="space-y-2 text-xs">
          <div class="flex items-center justify-between">
            <span class="text-slate-500 dark:text-slate-400">文件名：</span>
            <span class="font-medium">{file.remoteFilename}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-slate-500 dark:text-slate-400">远程路径：</span>
            <span
              class="rounded bg-slate-100/50 px-2 py-1 font-mono text-slate-700 dark:bg-slate-700/50 dark:text-slate-300"
            >
              {file.remoteFilenamePrefix}/{file.remoteFilename}
            </span>
          </div>
        </div>
      </div>
    </LinkPreview.Content>
  </LinkPreview.Root>
{/snippet}

{#snippet ready()}
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
{/snippet}

{#snippet uploader()}
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
        onclick={() => (filesState.files = [])}
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
      use:dragHandleZone={{ items: filesState.files, flipDurationMs }}
      onconsider={handleSort}
      onfinalize={handleSort}
      class="flex-1 space-y-2 overflow-y-auto p-2 dark:text-slate-100"
    >
      {#each filesState.files as file, index (file.id)}
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
            {@render preview(file)}
            <button onclick={() => removeFile(index)} class="action-button">
              <X class="size-4" />
            </button>
          </div>
        </div>
      {/each}
    </section>
  </div>
{/snippet}

<style lang="postcss">
  .input {
    @apply rounded-md bg-white/80 px-2 py-1 text-sm backdrop-blur-sm transition-all placeholder:text-slate-400/60 focus:ring-2 focus:ring-cyan-500/50 focus:outline-none dark:bg-slate-700/80 dark:placeholder:text-slate-400/50 dark:focus:ring-cyan-500/30;
  }

  .action-button {
    @apply cursor-pointer rounded-md p-1 text-slate-500 backdrop-blur-sm transition-all hover:bg-slate-200/50 hover:shadow-sm dark:text-slate-400 dark:hover:bg-slate-600/50;
  }
</style>
