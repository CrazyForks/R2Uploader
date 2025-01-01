<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Upload, Folder, Check, FileText } from "lucide-svelte";
  import db from "$lib/db";
  import type { UploadTarget } from "$lib/db";
  import { onMount } from "svelte";
  import { setAlert } from "$lib/store.svelte";

  let filePath = $state("");
  let fileName = $state("");
  let remoteFileName = $state("");
  let textContent = $state("");
  let activeTab = $state<"file" | "folder" | "text">("file");
  let uploadStatus = $state<"idle" | "uploading" | "success" | "error">("idle");
  let showUploadButton = $derived(
    (activeTab === "file" && !!filePath) ||
      (activeTab === "text" && !!textContent)
  );
  let uploadTargets = $state<UploadTarget[]>([]);
  let selectedTarget = $state<UploadTarget | null>(null);

  // 获取上传目标
  async function getUploadTargets() {
    uploadTargets = await db.uploadTargets.toArray();
    if (uploadTargets.length > 0) {
      selectedTarget = uploadTargets[0];
    }
  }

  onMount(() => {
    getUploadTargets();
  });

  // 文件上传功能
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

      if (textContent) {
        remoteFileName = generateTimestampFileName();
        await invoke("r2_upload", {
          bucketName: selectedTarget.bucketName,
          accountId: selectedTarget.accountId,
          accessKey: selectedTarget.accessKey,
          secretKey: selectedTarget.secretKey,
          source: { fileContent: textContent },
          remoteFileName,
        });
      } else {
        await invoke("r2_upload", {
          bucketName: selectedTarget.bucketName,
          accountId: selectedTarget.accountId,
          accessKey: selectedTarget.accessKey,
          secretKey: selectedTarget.secretKey,
          source: { filePath },
          remoteFileName,
        });
      }

      // 保存上传记录
      await db.uploadHistory.add({
        fileName,
        remoteFileName,
        target: selectedTarget.bucketName,
        timestamp: new Date(),
      });

      uploadStatus = "success";
      setAlert("上传成功");
      filePath = "";
      fileName = "";
      remoteFileName = "";
    } catch (error) {
      console.error(error);
      uploadStatus = "error";
      setAlert("上传失败，请重试");
    }
  }
</script>

<div class="max-w-4xl mx-auto p-6">
  <h1 class="text-2xl font-bold mb-6 text-gray-800 dark:text-gray-200">
    文件上传
  </h1>

  <div
    class="bg-white/80 dark:bg-gray-800/80 backdrop-blur-lg rounded-2xl shadow-lg p-6 border border-white/20 dark:border-gray-700/20"
  >
    {#if uploadTargets.length === 0}
      <div
        class="mb-4 p-4 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg text-yellow-800 dark:text-yellow-200"
      >
        请先<a href="/setting" class="font-medium underline">设置上传目标</a
        >后再进行上传
      </div>
    {:else}
      <div class="mb-4">
        <p
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >
          上传目标
        </p>
        <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded-lg">
          {selectedTarget?.bucketName}
        </div>
      </div>
    {/if}
    <div class="space-y-4">
      <div class="flex gap-2 mb-4">
        <button
          class:btn-tab-active={activeTab === "file"}
          onclick={() => (activeTab = "file")}
          class="btn-tab"
        >
          <Upload class="w-5 h-5" /> 上传文件
        </button>
        <button
          class:btn-tab-active={activeTab === "folder"}
          onclick={() => (activeTab = "folder")}
          class="btn-tab"
        >
          <Folder class="w-5 h-5" /> 上传文件夹
        </button>
        <button
          class:btn-tab-active={activeTab === "text"}
          onclick={() => (activeTab = "text")}
          class="btn-tab"
        >
          <FileText class="w-5 h-5" /> 上传文本
        </button>
      </div>

      {#if activeTab === "file"}
        <div class="space-y-4">
          <button onclick={openFile} class="w-full btn btn-default">
            <Upload class="w-6 h-6" /> 选择文件
          </button>

          {#if fileName}
            <div class="space-y-2">
              <div>
                <p
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >
                  文件名
                </p>
                <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded-lg">
                  {fileName}
                </div>
              </div>
              <div>
                <p
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >
                  远程文件名
                </p>
                <input
                  bind:value={remoteFileName}
                  class="w-full bg-gray-50 dark:bg-gray-700 p-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="输入远程文件名"
                />
              </div>
            </div>
          {/if}
        </div>
      {:else if activeTab === "folder"}
        <div class="space-y-4">
          <button onclick={openDirectory} class="w-full btn btn-default">
            <Folder class="w-6 h-6" /> 选择文件夹
          </button>
        </div>
      {:else if activeTab === "text"}
        <div class="space-y-4">
          <div class="space-y-2">
            <div>
              <p
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >
                文本内容
              </p>
              <textarea
                bind:value={textContent}
                class="w-full bg-gray-50 dark:bg-gray-700 p-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="输入要上传的文本内容"
                rows="6"
              ></textarea>
            </div>
            <div>
              <p
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >
                远程文件名
              </p>
              <input
                bind:value={remoteFileName}
                class="w-full bg-gray-50 dark:bg-gray-700 p-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="输入远程文件名"
              />
            </div>
          </div>
        </div>
      {/if}

      {#if showUploadButton}
        <div class="space-y-2">
          <button
            onclick={uploadFile}
            class="w-full btn btn-primary"
            disabled={uploadStatus === "uploading"}
          >
            {#if uploadStatus === "uploading"}
              上传中...
            {:else}
              <Check class="w-6 h-6" />确认上传
            {/if}
          </button>

          {#if uploadStatus === "error"}
            <button onclick={uploadFile} class="w-full btn btn-default">
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
    @apply flex items-center justify-center gap-2 px-8 py-4 rounded-xl transition-all shadow-sm hover:shadow-md text-lg cursor-pointer;
  }

  .btn-default {
    @apply bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600 active:bg-gray-100 dark:active:bg-gray-500 border border-gray-200 dark:border-gray-600;
  }

  .btn-primary {
    @apply bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700;
  }

  .btn-tab {
    @apply flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors;
    @apply bg-transparent text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700;
  }

  .btn-tab-active {
    @apply bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400;
    @apply hover:bg-blue-100 dark:hover:bg-blue-900/30;
  }
</style>
