<script lang="ts">
  import { Upload, X } from "lucide-svelte";
  import { tick } from "svelte";

  let {
    files,
    onFileSelect,
    onRemove,
    onAddPrefix,
    onRemoveAll,
  }: {
    files: {
      filename: string;
      remoteFilename: string;
      remoteFilenamePrefix: string;
    }[];
    onFileSelect: () => void;
    onRemove: (index: number) => void;
    onAddPrefix: (prefix: string) => void;
    onRemoveAll: () => void;
  } = $props();

  let oldPrefix = $state("");
  let prefix = $state("");

  async function onChangePrefix() {
    await tick();
    files.forEach((file) => {
      if (file.remoteFilenamePrefix === oldPrefix) {
        file.remoteFilenamePrefix = prefix;
      }
    });
    oldPrefix = prefix;
    console.log("files: ", files);
  }
</script>

<div class="max-h-96 space-y-2 overflow-y-auto">
  <div class="flex gap-2">
    <button onclick={onFileSelect} class="btn btn-default flex-1">
      <Upload class="size-5" /> 选择文件
    </button>
    <button onclick={onRemoveAll} class="btn btn-danger"> 全部删除 </button>
  </div>

  <div class="flex gap-2">
    <input
      bind:value={prefix}
      oninput={onChangePrefix}
      class="flex-1 rounded bg-slate-100 px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600"
      placeholder="输入前缀"
    />
  </div>

  {#each files as file, index}
    <div
      class="flex items-center gap-3 rounded-lg bg-slate-50 p-2 dark:bg-slate-700"
    >
      <div class="flex-1">
        <input type="text" bind:value={file.remoteFilenamePrefix} />
        <input
          bind:value={file.remoteFilename}
          class="w-full rounded bg-slate-100 px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600"
          placeholder="输入远程文件名"
        />
      </div>
      <button
        onclick={() => onRemove(index)}
        class="rounded p-1 hover:bg-slate-200 dark:hover:bg-slate-600"
      >
        <X class="size-4" />
      </button>
    </div>
  {/each}
</div>
