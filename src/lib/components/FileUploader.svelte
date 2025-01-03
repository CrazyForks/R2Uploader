<script lang="ts">
  import { Upload } from "lucide-svelte";

  let {
    filePath,
    fileName,
    remoteFileName,
    onFileSelect,
  }: {
    filePath: string;
    fileName: string;
    remoteFileName: string;
    onFileSelect: () => void;
  } = $props();
</script>

<div class="space-y-4">
  <button onclick={onFileSelect} class="btn btn-default w-full">
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
