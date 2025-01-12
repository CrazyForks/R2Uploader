<script lang="ts">
  import { globalState } from "$lib/store.svelte";
</script>

{#each globalState.files as file, index (file.id)}
  {@const progress = globalState.progress[file.id]}
  <div
    class="flex w-full items-center gap-4 rounded-md bg-slate-50/80 p-2 shadow-sm backdrop-blur-sm transition-all hover:shadow-md dark:bg-slate-700/80"
  >
    <div class="flex-1">
      <div class="mb-1 text-sm text-slate-500 dark:text-slate-400">
        {file.remoteFilename}
      </div>
      {#if typeof progress?.status === "object" && "uploading" in progress.status}
        <div class="h-2 w-full overflow-hidden rounded-full bg-slate-200">
          <div
            class="h-full bg-blue-500 transition-all"
            style="width: {progress.status.uploading.progress * 100}%"
          ></div>
        </div>
        <div class="mt-1 text-xs text-slate-500">
          {Math.floor(progress.status.uploading.progress * 100)}% -
          {(progress.status.uploading.bytes_uploaded / 1024 / 1024).toFixed(
            2,
          )}MB /
          {(progress.status.uploading.total_bytes / 1024 / 1024).toFixed(2)}MB
          {#if progress.status.uploading.speed > 0}
            - {(progress.status.uploading.speed / 1024 / 1024).toFixed(2)}MB/s
          {/if}
        </div>
      {:else if progress?.status === "success"}
        <div class="text-sm text-green-500">上传完成</div>
      {:else if typeof progress?.status === "object" && "error" in progress.status}
        <div class="text-sm text-red-500">
          上传失败：{progress.status.error.message}
        </div>
      {:else if progress?.status === "cancelled"}
        <div class="text-sm text-yellow-500">已取消</div>
      {:else}
        <div class="text-sm text-slate-500">等待上传...</div>
      {/if}
    </div>
  </div>
{/each}
