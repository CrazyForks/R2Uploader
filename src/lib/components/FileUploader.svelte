<script lang="ts">
  import { Upload, X, GripVertical, Eye, UploadCloud } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { tick } from "svelte";
  import { dragHandleZone, dragHandle } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import { invoke } from "@tauri-apps/api/core";
  import {
    dragState,
    modalState,
    setAlert,
    setDragPaths,
    showModal,
  } from "$lib/store.svelte";
  import { LinkPreview, type Selected } from "bits-ui";
  import { sep } from "@tauri-apps/api/path";
  import type { File } from "$lib/type";
  import { filesState } from "$lib/files.svelte";
  import TextUploader from "./TextUploader.svelte";

  let {
    uploadStatus = $bindable("idle"),
    uploadStatusMap = $bindable({}),
    intervalId = $bindable<number | undefined>(),
    selectedTarget,
  }: {
    uploadStatus?: "idle" | "uploading" | "success" | "error";
    uploadStatusMap?: Record<string, string>;
    intervalId?: number | undefined;
    selectedTarget?: Selected<{
      bucketName: string;
      accountId: string;
      accessKey: string;
      secretKey: string;
    }>;
  } = $props();

  let showUploadButton = $derived(filesState.files.length > 0);
  let showBigMenu = $derived(!filesState.files.length);
  let oldPrefix = $state("");
  let prefix = $state("");
  const flipDurationMs = 200;

  // 预览相关状态
  let previewContent = $state<string | null>(null);
  let previewLoading = $state(false);
  let previewError = $state<string | null>(null);

  $effect(() => {
    if (dragState.paths.length > 0) {
      parsePaths(dragState.paths);
      setDragPaths([]);
    }
  });

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
    if (!selectedTarget) return;
    try {
      uploadStatus = "uploading";

      const filesToUpload = filesState.files
        .filter((file) => file.selected)
        .map((file) => ({
          id: file.id,
          source: file.source,
          remoteFilename: `${file.remoteFilenamePrefix}/${file.remoteFilename}`,
        }));

      await invoke("r2_upload", {
        bucketName: selectedTarget.value.bucketName,
        accountId: selectedTarget.value.accountId,
        accessKey: selectedTarget.value.accessKey,
        secretKey: selectedTarget.value.secretKey,
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

  async function openFile() {
    const dialogFiles = await open({
      multiple: true,
      directory: false,
    });
    if (dialogFiles) {
      await parsePaths(dialogFiles);
    }
  }

  async function openDir() {
    const dialogFiles = await open({
      multiple: true,
      directory: true,
    });
    if (dialogFiles) {
      await parsePaths(dialogFiles);
    }
  }

  async function parsePaths(paths: string[]) {
    paths.forEach(async (file) => {
      const details = await getFileDetails(file);
      if (details && details.length > 0) {
        details.forEach((detail) => {
          filesState.files.push({
            type: "file",
            id: detail.id,
            source: {
              filePath: detail.path,
            },
            remoteFilename: handleRelativePath(detail.relativePath),
            remoteFilenamePrefix: "",
            selected: true,
          });
        });
      }
    });
  }

  // 如果以 sep 开头，去掉 sep，如果 sep() 不是 /，替换为 /
  function handleRelativePath(path: string) {
    const s = sep();
    return path.startsWith(s)
      ? path.slice(s.length).replaceAll(s, "/")
      : path.replaceAll(s, "/");
  }

  interface FileDetail {
    id: string;
    path: string;
    relativePath: string;
    isDir: boolean;
  }

  async function getFileDetails(path: string) {
    try {
      const details: Array<FileDetail> = await invoke("get_file_details", {
        path,
      });
      return details;
    } catch (e) {
      console.error(e);
      setAlert("获取文件详情失败");
    }
  }

  function removeFile(index: number) {
    filesState.files.splice(index, 1);
  }
</script>

{#snippet text()}
  <TextUploader />
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
        class="w-80 space-y-2 rounded-lg bg-slate-50/90 p-4 shadow-md backdrop-blur-sm dark:bg-slate-800/80"
      >
        <div class="flex items-center justify-center">
          {#if previewLoading}
            <div class="flex items-center justify-center p-4">
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
                class="max-h-48 max-w-48 rounded object-contain"
              />
            {:else if file.type === "text"}
              <div
                class="max-h-48 overflow-y-auto rounded bg-slate-100/50 p-2 text-sm dark:bg-slate-700/50"
              >
                {#each previewContent.split("\n") as line}
                  <div>{line}</div>
                {/each}
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

{#if showBigMenu}
  <div class="flex items-center justify-center gap-12">
    <UploadCloud class="hidden size-32 text-slate-400 sm:block" />
    <div class="flex flex-1 flex-col items-start gap-3">
      <p class="dark:text-slate-300">您的存储桶已就绪，拖放文件到此，或：</p>
      <div class="grid grid-cols-2 gap-2">
        <button onclick={openFile} class="button button-primary"
          >选择文件</button
        >
        <button onclick={openDir} class="button button-primary"
          >选择文件夹</button
        >
        <button onclick={openFile} class="button button-primary"
          >选择剪贴板</button
        >
        <button onclick={() => showModal(text)} class="button button-primary"
          >选择新建文本</button
        >
      </div>
    </div>
  </div>
{:else}
  <div class="flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-lg">
    <div
      class="flex h-12 items-center gap-4 rounded-t-lg bg-slate-50/80 backdrop-blur-sm dark:bg-slate-700/80"
    >
      <div class="px-2">
        <input
          bind:value={prefix}
          oninput={onChangePrefix}
          class="input w-36"
          placeholder="全局路径"
        />
      </div>
    </div>
    <section
      use:dragHandleZone={{ items: filesState.files, flipDurationMs }}
      onconsider={handleSort}
      onfinalize={handleSort}
      class="flex-1 space-y-2 overflow-y-auto p-2"
    >
      {#each filesState.files as file, index (file.id)}
        <div
          class="flex items-center gap-4 rounded-md bg-white/80 p-2 shadow-sm backdrop-blur-sm transition-all hover:shadow-md dark:bg-slate-700/80"
          animate:flip={{ duration: flipDurationMs }}
        >
          <div
            use:dragHandle
            class=" text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
          >
            <GripVertical class="size-4" />
          </div>
          <input
            type="checkbox"
            bind:checked={file.selected}
            class="size-4 rounded-md border-slate-300 text-cyan-600 focus:ring-cyan-500 dark:border-slate-600 dark:bg-slate-700"
          />
          <div class="flex-1">
            <div class="flex items-center gap-2">
              <input
                bind:value={file.remoteFilenamePrefix}
                class="input w-24"
                placeholder="远程路径"
              />
              <span>/</span>
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
{/if}

<style lang="postcss">
  .input {
    @apply rounded-md bg-white/80 px-2 py-1 text-sm backdrop-blur-sm transition-all placeholder:text-slate-400/60 focus:ring-2 focus:ring-cyan-500/50 focus:outline-none dark:bg-slate-700/80 dark:placeholder:text-slate-400/50 dark:focus:ring-cyan-500/30;
  }

  .action-button {
    @apply cursor-pointer rounded-md p-1 text-slate-500 backdrop-blur-sm transition-all hover:bg-slate-200/50 hover:shadow-sm dark:text-slate-400 dark:hover:bg-slate-600/50;
  }
</style>
