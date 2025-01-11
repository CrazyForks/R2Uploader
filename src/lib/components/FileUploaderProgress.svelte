<script lang="ts">
</script>

<div class="flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-lg">
  <!-- 功能条 -->
  <div
    class="flex items-center gap-2 rounded-t-lg bg-slate-200 p-1 shadow backdrop-blur-sm dark:bg-slate-700/80"
  >
    <div class="flex-1">上传进度</div>
  </div>

  <!-- 进度列表 -->
  <div class="min-h-0 flex-1 overflow-y-auto p-2">
    <!-- {#each filesState.files as file, index (file.id)}
      {@const progress = uploadStatusMap[file.id]}
      <div
        class="mb-2 flex items-center gap-2 rounded-lg bg-white p-2 shadow dark:bg-slate-700"
      >
        <div class="flex-1">
          <div class="mb-1 text-sm text-slate-500 dark:text-slate-400">
            {file.remoteFilename}
          </div>
          {#if progress?.status.type === "uploading"}
            <div class="h-2 w-full overflow-hidden rounded-full bg-slate-200">
              <div
                class="h-full bg-blue-500 transition-all"
                style="width: {progress.status.progress * 100}%"
              ></div>
            </div>
            <div class="mt-1 text-xs text-slate-500">
              {Math.floor(progress.status.progress * 100)}% -
              {(progress.status.bytesUploaded / 1024 / 1024).toFixed(2)}MB /
              {(progress.status.totalBytes / 1024 / 1024).toFixed(2)}MB
              {#if progress.status.speed > 0}
                - {(progress.status.speed / 1024 / 1024).toFixed(2)}MB/s
              {/if}
            </div>
          {:else if progress?.status.type === "success"}
            <div class="text-sm text-green-500">上传完成</div>
          {:else if progress?.status.type === "error"}
            <div class="text-sm text-red-500">
              上传失败: {progress.status.message}
            </div>
          {:else if progress?.status.type === "cancelled"}
            <div class="text-sm text-yellow-500">已取消</div>
          {:else}
            <div class="text-sm text-slate-500">等待上传...</div>
          {/if}
        </div>
      </div>
    {/each} -->
  </div>
</div>
