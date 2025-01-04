<script lang="ts">
  import FileUploader from "$lib/components/FileUploader.svelte";
  import TabSwitcher from "$lib/components/TabSwitcher.svelte";
  import UploadTargetSelector from "$lib/components/UploadTargetSelector.svelte";
  import db from "$lib/db";
  import { setAlert } from "$lib/store.svelte";
  import { t } from "$lib/i18n/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { sep } from "@tauri-apps/api/path";
  import type { Selected } from "bits-ui";
  import { Check } from "lucide-svelte";
  import { onMount } from "svelte";
  import clipboard from "tauri-plugin-clipboard-api";
  import type { Bucket } from "$lib/type";

  let dialogFiles: string[] = [];
  let textContent = $state("");
  let activeTab = $state<"file" | "folder" | "text" | "clipboard">("file");

  let files = $state<
    {
      id: string;
      filename: string;
      remoteFilename: string;
      remoteFilenamePrefix: string;
    }[]
  >([]);
  let uploadStatus = $state<"idle" | "uploading" | "success" | "error">("idle");
  let uploadStatusMap = $state<Record<string, string>>({});
  let intervalId = $state<number | undefined>();

  let clipboardFiles = $state<string[]>([]);
  let clipboardText = $state("");
  let clipboardHtml = $state("");
  let clipboardImage = $state("");
  let clipboardRtf = $state("");
  let uploadTargets: Selected<Bucket>[] = $state([]);
  let selectedTarget: Selected<Bucket> | undefined = $state();

  async function getUploadTargets() {
    await db.buckets.toArray().then((targets) => {
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
        source:
          activeTab === "text"
            ? { fileContent: textContent }
            : { filePath: file.filename },
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
</script>

<div class="mx-auto max-w-4xl p-6">
  <h1 class="mb-6 text-2xl font-bold text-slate-800 dark:text-slate-200">
    {$t("common.file_upload")}
  </h1>

  <div>
    {#if uploadTargets.length === 0}
      <div
        class="mb-4 rounded-lg bg-yellow-50 p-4 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-200"
      >
        {$t("common.set_upload_target_first")}
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
          bind:files
          bind:uploadStatus
          bind:uploadStatusMap
          bind:intervalId
          {selectedTarget}
        />
      {:else if activeTab === "text"}
        <!-- <TextUploader
          {textContent}
          remoteFileName={remoteFileNames.join(",")}
        /> -->
      {:else if activeTab === "clipboard"}
        <!-- <ClipboardUploader
          {clipboardText}
          {clipboardHtml}
          {clipboardImage}
          {clipboardRtf}
          {clipboardFiles}
          remoteFileName={remoteFileNames.join(",")}
          onRefreshClipboard={checkClipboardContent}
        /> -->
      {/if}
    </div>
  </div>

  <div class="space-y-2 pt-4">
    <button
      onclick={uploadFile}
      class="btn btn-primary w-full"
      disabled={uploadStatus === "uploading"}
    >
      {#if uploadStatus === "uploading"}
        {$t("common.uploading")}
      {:else}
        <Check class="size-6" />{$t("common.confirm_upload")}
      {/if}
    </button>

    {#if uploadStatus === "error"}
      <button onclick={uploadFile} class="btn btn-default w-full">
        {$t("common.retry_upload")}
      </button>
    {/if}
  </div>
</div>

<style lang="postcss">
  /* Bits UI Select 样式 */
  :global(.select-trigger) {
    @apply flex min-w-48 cursor-pointer items-center justify-between rounded-xl border border-slate-200 bg-white px-4 py-2 text-gray-700 transition-colors hover:border-cyan-500 dark:border-slate-600 dark:bg-slate-700 dark:text-gray-200;
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
    @apply bg-cyan-50 text-cyan-600 dark:bg-cyan-900/30 dark:text-cyan-400;
  }
</style>
