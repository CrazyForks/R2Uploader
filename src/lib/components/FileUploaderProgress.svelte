<script lang="ts">
  import db from "$lib/db";
  import { globalState, setAlert } from "$lib/store.svelte";
  import type { UploadHistory } from "$lib/type";
  import { Copy } from "lucide-svelte";

  let { activeTab }: { activeTab: "all" | "in-progress" | "completed" } =
    $props();
  let currentPage = $state(1);
  const pageSize = 10;
  let files = $state<UploadHistory[]>([]);

  $effect(() => {
    // 只要 globalState.statusChange 发生变化，就重新获取数据
    globalState.statusChange;
    switch (activeTab) {
      case "all":
        getAllFiles().then((f) => {
          files = f;
        });
      case "in-progress":
        getInProgressFiles().then((f) => {
          files = f;
        });
      case "completed":
        getCompletedFiles().then((f) => {
          files = f;
        });
    }
  });

  // 获取已完成文件
  async function getCompletedFiles() {
    const offset = (currentPage - 1) * pageSize;
    return await db.history
      .orderBy("timestamp")
      .reverse()
      .offset(offset)
      .limit(pageSize)
      .toArray();
  }

  // 获取所有文件
  async function getAllFiles() {
    const offset = (currentPage - 1) * pageSize;
    return await db.history
      .orderBy("timestamp")
      .reverse()
      .offset(offset)
      .limit(pageSize)
      .toArray();
  }

  // 获取正在上传文件
  async function getInProgressFiles() {
    const offset = (currentPage - 1) * pageSize;
    return await db.history
      .where("status")
      .equals("in-progress")
      .offset(offset)
      .limit(pageSize)
      .toArray();
  }

  async function copyLink(url: string) {
    try {
      await navigator.clipboard.writeText(url);
      setAlert("复制成功");
    } catch (e) {
      setAlert("复制失败");
    }
  }
</script>

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
            <div class="h-2 w-full overflow-hidden rounded-full bg-slate-200">
              <div
                class="h-full bg-blue-500 transition-all"
                style="width: {file.status.uploading.progress * 100}%"
              ></div>
            </div>
            <div class="mt-1 text-xs text-slate-500">
              {Math.floor(file.status.uploading.progress * 100)}% -
              {(file.status.uploading.bytesUploaded / 1024 / 1024).toFixed(2)}MB
              /
              {(file.status.uploading.totalBytes / 1024 / 1024).toFixed(2)}MB
              {#if file.status.uploading.speed > 0}
                - {(file.status.uploading.speed / 1024 / 1024).toFixed(2)}MB/s
              {/if}
            </div>
          {:else if file.status === "success"}
            <div class="text-sm text-green-500">上传完成</div>
          {:else if typeof file.status === "object" && "error" in file.status}
            <div class="text-sm text-red-500">
              上传失败：{file.status.error.message}
            </div>
          {:else if file.status === "cancelled"}
            <div class="text-sm text-yellow-500">已取消</div>
          {:else}
            <div class="text-sm text-slate-500">等待上传...</div>
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
</div>

<style lang="postcss">
  .action-button {
    @apply cursor-pointer rounded-md p-1 text-slate-500 backdrop-blur-sm transition-all hover:bg-slate-200/50 hover:shadow-sm dark:text-slate-400 dark:hover:bg-slate-600/50;
  }
</style>
