<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Upload, Folder, Check } from "lucide-svelte";

  let filePath = $state("");
  let showUploadButton = $derived(!!filePath);

  // 文件上传功能
  async function openFile() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    if (file) {
      filePath = file;
    }
  }

  async function openDirectory() {
    await open({
      multiple: false,
      directory: true,
    });
  }

  async function uploadFile() {
    // 	try {
    // 		await invoke("r2_upload", {
    // 			bucketName,
    // 			accountId,
    // 			accessKey,
    // 			secretKey,
    // 			filePath,
    // 		});
    // 		console.log("Upload success");
    // 	} catch (error) {
    // 		console.error(error);
    // 	}
  }
</script>

<div class="max-w-4xl mx-auto p-6">
  <h1 class="text-2xl font-bold mb-6 text-gray-800 dark:text-gray-200">
    文件上传
  </h1>

  <div
    class="bg-white/80 dark:bg-gray-800/80 backdrop-blur-lg rounded-2xl shadow-lg p-6 border border-white/20 dark:border-gray-700/20"
  >
    <div class="space-y-4">
      <div class="flex gap-4">
        <button onclick={openFile} class="flex-1 btn btn-default"
          ><Upload class="w-6 h-6" />选择文件</button
        >
        <button onclick={openDirectory} class="flex-1 btn btn-default"
          ><Folder class="w-6 h-6" />选择文件夹</button
        >
      </div>
      {#if showUploadButton}
        <button onclick={uploadFile} class="w-full btn btn-primary"
          ><Check class="w-6 h-6" />确认上传</button
        >
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
</style>
