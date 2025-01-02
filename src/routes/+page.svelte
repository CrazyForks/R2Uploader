<script lang="ts">
  import type { UploadTarget } from "$lib/db";
  import db from "$lib/db";
  import { setAlert } from "$lib/store.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Select, type Selected } from "bits-ui";
  import {
    Check,
    ChevronsUpDown,
    Clipboard,
    FileText,
    Folder,
    Upload,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import clipboard from "tauri-plugin-clipboard-api";

  let filePath = $state("");
  let fileName = $state("");
  let remoteFileName = $state("");
  let textContent = $state("");
  let activeTab = $state<"file" | "folder" | "text" | "clipboard">("file");

  let uploadStatus = $state<"idle" | "uploading" | "success" | "error">("idle");
  let clipboardFiles = $state<string[]>([]);
  let clipboardText = $state("");
  let clipboardHtml = $state("");
  let clipboardImage = $state("");
  let clipboardRtf = $state("");
  let showUploadButton = $derived(
    (activeTab === "file" && !!filePath) ||
      (activeTab === "text" && !!textContent) ||
      (activeTab === "clipboard" &&
        (!!clipboardFiles.length ||
          !!clipboardText ||
          !!clipboardHtml ||
          !!clipboardImage ||
          !!clipboardRtf)),
  );
  let uploadTargets: Selected<UploadTarget>[] = $state([]);
  let selectedTarget: Selected<UploadTarget> | undefined = $state();

  async function getUploadTargets() {
    await db.uploadTargets.toArray().then((targets) => {
      uploadTargets = targets.map((target) => ({
        value: target,
        label: target.bucketName,
      }));
    });
    if (uploadTargets.length > 0) {
      selectedTarget = uploadTargets[0];
    }
  }

  async function checkClipboardContent() {
    try {
      if (await clipboard.hasText()) {
        clipboardText = await clipboard.readText();
      }
      if (await clipboard.hasHTML()) {
        clipboardHtml = await clipboard.readHtml();
      }
      if (await clipboard.hasImage()) {
        clipboardImage = await clipboard.readImageBase64();
      }
      if (await clipboard.hasRTF()) {
        clipboardRtf = await clipboard.readRtf();
      }
      if (await clipboard.hasFiles()) {
        clipboardFiles = await clipboard.readFiles();
      }
    } catch (error: unknown) {
      console.error(error);
      setAlert("读取剪贴板内容失败");
    }
  }

  onMount(async () => {
    getUploadTargets();
    await checkClipboardContent();
  });

  async function openFile() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    if (file) {
      filePath = file;
      fileName = file;
      remoteFileName = file;
    }
  }

  async function openDirectory() {
    await open({
      multiple: false,
      directory: true,
    });
  }

  function generateTimestampFileName() {
    const now = new Date();
    return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}_${String(now.getHours()).padStart(2, "0")}-${String(now.getMinutes()).padStart(2, "0")}-${String(now.getSeconds()).padStart(2, "0")}.txt`;
  }

  async function uploadFile() {
    if (!selectedTarget) return;
    try {
      uploadStatus = "uploading";

      let source: unknown;
      if (activeTab === "text") {
        source = { fileContent: textContent };
      } else if (activeTab === "clipboard") {
        if (clipboardText) {
          source = { fileContent: clipboardText };
        } else if (clipboardFiles.length > 0) {
          source = { filePath: clipboardFiles[0] };
        }
      } else {
        source = { filePath };
      }

      await invoke("r2_upload", {
        bucketName: selectedTarget.value.bucketName,
        accountId: selectedTarget.value.accountId,
        accessKey: selectedTarget.value.accessKey,
        secretKey: selectedTarget.value.secretKey,
        source,
        remoteFileName,
      });

      await db.uploadHistory.add({
        fileName,
        remoteFileName,
        target: selectedTarget.value.bucketName,
        timestamp: new Date(),
      });

      uploadStatus = "success";
      setAlert("上传成功");
      filePath = "";
      fileName = "";
      remoteFileName = "";
    } catch (error: unknown) {
      console.error(error);
      uploadStatus = "error";
      setAlert("上传失败，请重试");
    }
  }
</script>

<div class="mx-auto max-w-4xl p-6">
  <h1 class="mb-6 text-2xl font-bold text-slate-800 dark:text-slate-200">
    文件上传
  </h1>

  <div
    class="rounded-2xl border border-white/20 bg-white/80 p-6 shadow-lg backdrop-blur-lg dark:border-slate-700/20 dark:bg-slate-800/80"
  >
    {#if uploadTargets.length === 0}
      <div
        class="mb-4 rounded-lg bg-yellow-50 p-4 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-200"
      >
        请先<a href="/setting" class="font-medium underline">设置上传目标</a
        >后再进行上传
      </div>
    {:else}
      <div class="mb-4 flex items-center gap-4">
        <p class="text-sm font-medium text-slate-700 dark:text-slate-300">
          上传目标
        </p>
        <Select.Root
          items={uploadTargets}
          selected={selectedTarget}
          onSelectedChange={(e) => {
            selectedTarget = {
              value: e!.value,
              label: e?.value.bucketName || "unknown",
            };
          }}
        >
          <Select.Trigger class="select-trigger">
            <Select.Value placeholder="选择上传目标" />
            <ChevronsUpDown
              class="dark-text-slate-300 ml-auto size-4 text-slate-400"
            />
          </Select.Trigger>
          <Select.Content class="select-content">
            {#each uploadTargets as target}
              <Select.Item
                value={target.value}
                label={target.label}
                class="select-item"
              >
                {target.label}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    {/if}
    <div class="space-y-4">
      <div class="mb-4 flex gap-2">
        <button
          class:btn-tab-active={activeTab === "file"}
          onclick={() => (activeTab = "file")}
          class="btn-tab"
        >
          <Upload class="size-5" /> 上传文件
        </button>
        <button
          class:btn-tab-active={activeTab === "folder"}
          onclick={() => (activeTab = "folder")}
          class="btn-tab"
        >
          <Folder class="size-5" /> 上传文件夹
        </button>
        <button
          class:btn-tab-active={activeTab === "text"}
          onclick={() => {
            activeTab = "text";
            remoteFileName = generateTimestampFileName();
          }}
          class="btn-tab"
        >
          <FileText class="size-5" /> 上传文本
        </button>
        <button
          class:btn-tab-active={activeTab === "clipboard"}
          onclick={() => {
            activeTab = "clipboard";
            remoteFileName = generateTimestampFileName();
          }}
          class="btn-tab"
        >
          <Clipboard class="size-5" /> 剪贴板
        </button>
      </div>

      {#if activeTab === "file"}
        <div class="space-y-4">
          <button onclick={openFile} class="btn btn-default w-full">
            <Upload class="size-6" /> 选择文件
          </button>

          {#if fileName}
            <div class="space-y-2">
              <div>
                <p
                  class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
                >
                  文件名
                </p>
                <div class="rounded-lg bg-slate-50 p-3 dark:bg-slate-700">
                  {fileName}
                </div>
              </div>
              <div>
                <p
                  class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
                >
                  远程文件名
                </p>
                <input
                  bind:value={remoteFileName}
                  class="w-full rounded-lg bg-slate-50 p-3 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-700"
                  placeholder="输入远程文件名"
                />
              </div>
            </div>
          {/if}
        </div>
      {:else if activeTab === "folder"}
        <div class="space-y-4">
          <button onclick={openDirectory} class="btn btn-default w-full">
            <Folder class="size-6" /> 选择文件夹
          </button>
        </div>
      {:else if activeTab === "text"}
        <div class="space-y-4">
          <div class="space-y-2">
            <div>
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                文本内容
              </p>
              <textarea
                bind:value={textContent}
                class="w-full rounded-lg bg-slate-50 p-3 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-700"
                placeholder="输入要上传的文本内容"
                rows="6"
              ></textarea>
            </div>
            <div>
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                远程文件名
              </p>
              <input
                bind:value={remoteFileName}
                class="w-full rounded-lg bg-slate-50 p-3 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-700"
                placeholder="输入远程文件名"
              />
            </div>
          </div>
        </div>
      {:else if activeTab === "clipboard"}
        <div class="space-y-4">
          <button
            onclick={checkClipboardContent}
            class="btn btn-default w-full"
          >
            <Clipboard class="size-6" /> 刷新剪贴板内容
          </button>

          {#if clipboardText}
            <div class="space-y-2">
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                剪贴板文本
              </p>
              <div class="rounded-lg bg-slate-50 p-3 dark:bg-slate-700">
                {clipboardText}
              </div>
            </div>
          {/if}

          {#if clipboardHtml}
            <div class="space-y-2">
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                剪贴板 HTML
              </p>
              <div class="rounded-lg bg-slate-50 p-3 dark:bg-slate-700">
                {@html clipboardHtml}
              </div>
            </div>
          {/if}

          {#if clipboardImage}
            <div class="space-y-2">
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                剪贴板图片
              </p>
              <img
                src={`data:image/png;base64,${clipboardImage}`}
                alt="剪贴板图片"
                class="max-w-full"
              />
            </div>
          {/if}

          {#if clipboardRtf}
            <div class="space-y-2">
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                剪贴板 RTF
              </p>
              <div class="rounded-lg bg-slate-50 p-3 dark:bg-slate-700">
                {clipboardRtf}
              </div>
            </div>
          {/if}

          {#if clipboardFiles.length > 0}
            <div class="space-y-2">
              <p
                class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                剪贴板文件
              </p>
              <div class="rounded-lg bg-slate-50 p-3 dark:bg-slate-700">
                {#each clipboardFiles as file}
                  <div>{file}</div>
                {/each}
              </div>
            </div>
          {/if}

          <div class="space-y-2">
            <p
              class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
            >
              远程文件名
            </p>
            <input
              bind:value={remoteFileName}
              class="w-full rounded-lg bg-slate-50 p-3 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-700"
              placeholder="输入远程文件名"
            />
          </div>
        </div>
      {/if}

      {#if showUploadButton}
        <div class="space-y-2">
          <button
            onclick={uploadFile}
            class="btn btn-primary w-full"
            disabled={uploadStatus === "uploading"}
          >
            {#if uploadStatus === "uploading"}
              上传中...
            {:else}
              <Check class="size-6" />确认上传
            {/if}
          </button>

          {#if uploadStatus === "error"}
            <button onclick={uploadFile} class="btn btn-default w-full">
              重试上传
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="postcss">
  .btn {
    @apply flex cursor-pointer items-center justify-center gap-2 rounded-xl px-8 py-4 text-lg shadow-sm transition-all hover:shadow-md;
  }

  .btn-default {
    @apply border border-slate-200 bg-white text-slate-800 hover:bg-slate-50 active:bg-slate-100 dark:border-slate-600 dark:bg-slate-700 dark:text-slate-200 dark:hover:bg-slate-600 dark:active:bg-slate-500;
  }

  .btn-primary {
    @apply bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700;
  }

  .btn-tab {
    @apply flex cursor-pointer items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition-colors;
    @apply bg-transparent text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-700;
  }

  .btn-tab-active {
    @apply bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400;
    @apply hover:bg-blue-100 dark:hover:bg-blue-900/30;
  }

  /* Bits UI Select 样式 */
  :global(.select-trigger) {
    @apply flex min-w-48 cursor-pointer items-center justify-between rounded-xl border border-slate-200 bg-white px-4 py-2 text-gray-700 transition-colors hover:border-blue-500 dark:border-slate-600 dark:bg-slate-700 dark:text-gray-200;
  }

  :global(.select-trigger[disabled]) {
    @apply cursor-not-allowed bg-gray-100 text-gray-400 hover:border-slate-200 dark:bg-slate-800 dark:text-gray-500;
  }

  :global(.select-content) {
    @apply mt-1 overflow-hidden rounded-xl border border-slate-200 bg-white py-1 shadow-lg dark:border-slate-600 dark:bg-slate-700;
  }

  :global(.select-item) {
    @apply cursor-pointer px-4 py-1 text-gray-700 outline-none hover:bg-slate-100 dark:text-gray-200 dark:hover:bg-slate-600;
  }

  :global(.select-item[data-highlighted]) {
    @apply bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400;
  }
</style>
