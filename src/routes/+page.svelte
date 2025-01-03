<script lang="ts">
  import type { UploadTarget } from "$lib/db";
  import db from "$lib/db";
  import { setAlert } from "$lib/store.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import clipboard from "tauri-plugin-clipboard-api";
  import type { Selected } from "bits-ui";

  import UploadTargetSelector from "$lib/components/UploadTargetSelector.svelte";
  import TabSwitcher from "$lib/components/TabSwitcher.svelte";
  import FileUploader from "$lib/components/FileUploader.svelte";
  import TextUploader from "$lib/components/TextUploader.svelte";
  import ClipboardUploader from "$lib/components/ClipboardUploader.svelte";
  import { Check } from "lucide-svelte";
  import { page } from "$app/state";

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
        请先 <a href="/setting" class="font-medium underline">设置上传目标</a> 后再进行上传
      </div>
    {:else}
      <UploadTargetSelector
        {uploadTargets}
        {selectedTarget}
        onSelectedChange={(e) => {
          selectedTarget = {
            value: e!.value,
            label: e?.value.bucketName || "unknown",
          };
        }}
      />
    {/if}
    <div class="space-y-4">
      <TabSwitcher {activeTab} onTabChange={(tab) => (activeTab = tab)} />

      {#if activeTab === "file"}
        <FileUploader
          {filePath}
          {fileName}
          {remoteFileName}
          onFileSelect={openFile}
        />
      {:else if activeTab === "text"}
        <TextUploader {textContent} {remoteFileName} />
      {:else if activeTab === "clipboard"}
        <ClipboardUploader
          {clipboardText}
          {clipboardHtml}
          {clipboardImage}
          {clipboardRtf}
          {clipboardFiles}
          {remoteFileName}
          onRefreshClipboard={checkClipboardContent}
        />
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
