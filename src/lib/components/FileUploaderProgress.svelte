<script lang="ts">
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import { globalState, setAlert } from "$lib/store.svelte";
  import type { UploadHistory } from "$lib/type";
  import { Copy } from "lucide-svelte";
  import { onMount, tick } from "svelte";

  const pageSize = 20;
  const tabs: { id: "all" | "in-progress" | "completed"; label: string }[] = [
    { id: "all", label: t().transfer.tabs.all },
    { id: "in-progress", label: t().transfer.tabs.inProgress },
    { id: "completed", label: t().transfer.tabs.completed },
  ];

  let activeTab: "all" | "in-progress" | "completed" = $state("all");
  let files: UploadHistory[] = $state([]);
  let currentPage = $state(1);
  let totalItems = $state(0);
  let totalPages = $state(1);

  onMount(async () => {
    await getAllFiles();
  });

  async function changeTab() {
    await tick();
    switch (activeTab) {
      case "all":
        getAllFiles();
        break;
      case "in-progress":
        setInProgress();
        break;
      case "completed":
        completedFiles();
        break;
    }
  }

  function setInProgress() {
    files = Object.values(globalState.progress);
    totalItems = files.length;
    totalPages = 1;
  }

  // 获取已完成文件总数
  async function getCompletedFilesCount() {
    return await db.history.count();
  }

  async function completedFiles() {
    files = await getCompletedFiles();
  }

  // 获取已完成文件
  async function getCompletedFiles() {
    const offset = (currentPage - 1) * pageSize;
    const count = await getCompletedFilesCount();
    totalItems = count;
    totalPages = Math.ceil(count / pageSize);
    return await db.history
      .orderBy("timestamp")
      .reverse()
      .offset(offset)
      .limit(pageSize)
      .toArray();
  }

  // 获取所有文件
  async function getAllFiles() {
    const inProgressFiles = Object.values(globalState.progress);
    const completedFiles = await getCompletedFiles();
    files = [...inProgressFiles, ...completedFiles];
    totalItems = inProgressFiles.length + totalItems;
    totalPages = Math.ceil(totalItems / pageSize);
  }

  async function copyLink(url: string) {
    try {
      await navigator.clipboard.writeText(url);
      setAlert(t().fileUploader.uploadStatus.copySuccess);
    } catch (e) {
      setAlert(t().fileUploader.uploadStatus.copyFailed);
    }
  }
</script>

<div class="flex gap-2">
  {#each tabs as tab}
    <button
      class="nav-link gapped rounded-lg"
      class:bg={activeTab === tab.id}
      aria-current={activeTab === tab.id || undefined}
      onclick={() => {
        activeTab = tab.id;
        changeTab();
      }}
    >
      {tab.label}
    </button>
  {/each}
</div>

<div
  class="flex min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
>
  {#if !files.length}
    <div class="flex w-full items-center justify-center">
      {t().fileUploader.uploadStatus.nothing}
    </div>
  {:else}
    <div class="w-full space-y-2 overflow-y-auto p-2">
      {#each files as file (file.fileId)}
        <div
          class="flex w-full items-center gap-4 rounded-md bg-slate-50/80 p-2 shadow-sm backdrop-blur-sm transition-all hover:shadow-md dark:bg-slate-700/80"
        >
          <div class="flex flex-1 items-center justify-between">
            <div class="flex-1">
              <div class="text-sm text-slate-500 dark:text-slate-400">
                {file.filename}
              </div>
              {#if typeof file.status === "object" && "uploading" in file.status}
                <div
                  class="h-2 w-full overflow-hidden rounded-full bg-slate-200"
                >
                  <div
                    class="h-full bg-blue-500 transition-all"
                    style="width: {file.status.uploading.progress * 100}%"
                  ></div>
                </div>
                <div class="mt-1 text-xs text-slate-500">
                  {Math.floor(file.status.uploading.progress * 100)}% -
                  {(file.status.uploading.bytesUploaded / 1024 / 1024).toFixed(
                    2,
                  )}MB /
                  {(file.status.uploading.totalBytes / 1024 / 1024).toFixed(
                    2,
                  )}MB
                  {#if file.status.uploading.speed > 0}
                    - {(file.status.uploading.speed / 1024 / 1024).toFixed(
                      2,
                    )}{t().fileUploader.uploadStatus.speed}
                  {/if}
                </div>
              {:else if file.status === "success"}
                <div class="text-sm">
                  <span class="text-green-500"
                    >{t().fileUploader.uploadStatus.uploadComplete}</span
                  >
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {:else if typeof file.status === "object" && "error" in file.status}
                <div class="text-sm text-red-500">
                  {t().fileUploader.uploadStatus.uploadFailed}{file.status.error
                    .message} ·
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {:else if file.status === "cancelled"}
                <div class="text-sm text-yellow-500">
                  {t().fileUploader.uploadStatus.cancelled} ·
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {:else}
                <div class="text-sm text-slate-500">
                  {t().fileUploader.uploadStatus.waiting} ·
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {/if}
            </div>
            <div class="px-2">
              <button class="action-button" onclick={() => copyLink(file.url)}>
                <Copy class="size-4" />
              </button>
            </div>
          </div>
        </div>
      {/each}

      <!-- Pagination controls -->
      <div class="flex items-center justify-between">
        <div class="text-sm text-slate-600 dark:text-slate-400">
          {t().fileUploader.uploadStatus.total}: {totalItems} ·
          {t().fileUploader.uploadStatus.page}: {currentPage}/{totalPages}
        </div>
        <div class="flex gap-2">
          <button
            class="rounded px-3 py-1 text-sm {currentPage === 1
              ? 'cursor-not-allowed bg-slate-100 text-slate-400 dark:bg-slate-800'
              : 'cursor-pointer bg-blue-500 text-white hover:bg-blue-600'}"
            disabled={currentPage === 1}
            onclick={() => {
              currentPage--;
              changeTab();
            }}
          >
            {t().fileUploader.uploadStatus.previous}
          </button>
          <button
            class="rounded px-3 py-1 text-sm {currentPage === totalPages
              ? 'cursor-not-allowed bg-slate-100 text-slate-400 dark:bg-slate-800'
              : 'cursor-pointer bg-blue-500 text-white hover:bg-blue-600'}"
            disabled={currentPage === totalPages}
            onclick={() => {
              currentPage++;
              changeTab();
            }}
          >
            {t().fileUploader.uploadStatus.next}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style lang="postcss">
  .action-button {
    @apply cursor-pointer rounded-md p-1 text-slate-500 backdrop-blur-sm transition-all hover:bg-slate-200/50 hover:shadow-sm dark:text-slate-400 dark:hover:bg-slate-600/50;
  }
  .nav-link {
    @apply flex cursor-pointer items-center text-slate-700 transition-colors dark:text-slate-200;
  }

  .gapped {
    @apply px-4 py-2;
  }

  .nav-link[aria-current] {
    @apply text-cyan-600 dark:text-cyan-400;
  }

  .nav-link[aria-current].bg {
    @apply bg-cyan-50 dark:bg-cyan-900/30;
  }
</style>
