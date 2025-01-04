<script lang="ts">
  import { Upload, X, GripVertical, Eye, UploadCloud } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { tick } from "svelte";
  import { dragHandleZone, dragHandle } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import { invoke } from "@tauri-apps/api/core";
  import { setAlert } from "$lib/store.svelte";
  import type { Selected } from "bits-ui";
  import { sep } from "@tauri-apps/api/path";

  let {
    files = $bindable([]),
    uploadStatus = $bindable("idle"),
    uploadStatusMap = $bindable({}),
    intervalId = $bindable<number | undefined>(),
    selectedTarget,
  }: {
    files: {
      id: string;
      filename: string;
      remoteFilename: string;
      remoteFilenamePrefix: string;
      selected?: boolean;
    }[];
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

  let showUploadButton = $derived(files.length > 0);

  let showBigMenu = $derived(!files.length);
  let oldPrefix = $state("");
  let prefix = $state("");

  const flipDurationMs = 200;

  function handleSort(e: CustomEvent) {
    files = e.detail.items;
  }

  async function onChangePrefix() {
    await tick();
    files.forEach((file) => {
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

      // 如果所有文件都上传完成，停止轮询
      if (Object.keys(status).length === 0) {
        clearInterval(intervalId);
        intervalId = undefined;
        uploadStatus = "success";
        setAlert("上传成功");
        files = [];
      }
    } catch (error) {
      console.error("获取上传状态失败：", error);
    }
  }

  async function uploadFile() {
    if (!selectedTarget) return;
    try {
      uploadStatus = "uploading";

      const filesToUpload = files.map((file) => ({
        id: file.id,
        source: { filePath: file.filename },
        remoteFilename: `${file.remoteFilenamePrefix}/${file.remoteFilename}`,
      }));

      await invoke("r2_upload", {
        bucketName: selectedTarget.value.bucketName,
        accountId: selectedTarget.value.accountId,
        accessKey: selectedTarget.value.accessKey,
        secretKey: selectedTarget.value.secretKey,
        files: filesToUpload,
      });

      // 启动状态轮询
      if (!intervalId) {
        intervalId = setInterval(checkUploadStatus, 500);
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
      dialogFiles.forEach((file) => {
        files.push({
          id: file,
          filename: file,
          remoteFilename: file.split(sep()).pop() || "unknown",
          remoteFilenamePrefix: "",
        });
      });
    }
  }

  function removeFile(index: number) {
    files.splice(index, 1);
  }
</script>

<div
  class="max-h-96 space-y-2 overflow-y-auto rounded-2xl border border-white/20 bg-white/80 shadow-lg backdrop-blur-lg dark:border-slate-700/20 dark:bg-slate-800/80"
>
  {#if showBigMenu}
    <div class="flex h-64 items-center justify-center">
      <div class="flex items-center justify-center gap-12">
        <UploadCloud class="size-32" />
        <div class="flex flex-1 flex-col items-center gap-3">
          <p>您的存储桶已就绪</p>
          <div>
            拖放或
            <button onclick={openFile} class="cursor-pointer pl-2 text-blue-500"
              >点击选择文件</button
            >
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="">
      <div class="flex h-12 items-center gap-4 bg-slate-900">
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
        use:dragHandleZone={{ items: files, flipDurationMs }}
        onconsider={handleSort}
        onfinalize={handleSort}
        class="space-y-2 px-2 py-4"
      >
        {#each files as file, index (file.id)}
          <div
            class="flex items-center gap-4 rounded-xl bg-slate-50 p-2 dark:bg-slate-700"
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
              class="size-4 rounded-lg border-slate-300 text-blue-600 focus:ring-blue-500 dark:border-slate-600 dark:bg-slate-700"
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
              <button class="action-button">
                <Eye class="size-4" />
              </button>
              <button onclick={() => removeFile(index)} class="action-button">
                <X class="size-4" />
              </button>
            </div>
          </div>
        {/each}
      </section>
    </div>
  {/if}
</div>

<style lang="postcss">
  .input {
    @apply rounded-lg bg-slate-100 px-2 py-1 text-sm placeholder:text-slate-400 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600;
  }

  .action-button {
    @apply cursor-pointer rounded-lg p-1 text-slate-500 hover:bg-slate-200 dark:text-slate-400 dark:hover:bg-slate-600;
  }
</style>
