<script lang="ts">
  import { Clipboard } from "lucide-svelte";

  let {
    clipboardText,
    clipboardHtml,
    clipboardImage,
    clipboardRtf,
    clipboardFiles,
    remoteFileName,
    onRefreshClipboard,
  }: {
    clipboardText: string;
    clipboardHtml: string;
    clipboardImage: string;
    clipboardRtf: string;
    clipboardFiles: string[];
    remoteFileName: string;
    onRefreshClipboard: () => void;
  } = $props();
</script>

<div class="space-y-4">
  <button onclick={onRefreshClipboard} class="btn btn-default w-full">
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
