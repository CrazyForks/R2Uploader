<script lang="ts">
  import { Upload, X, Check, ChevronsUpDown } from "lucide-svelte";
  import { tick } from "svelte";
  import { dragHandleZone, dragHandle } from "svelte-dnd-action";
  import { flip } from "svelte/animate";

  let {
    files = $bindable([]),
    onFileSelect,
    onRemove,
    onRemoveAll,
  }: {
    files: {
      id: string;
      filename: string;
      remoteFilename: string;
      remoteFilenamePrefix: string;
      selected?: boolean;
    }[];
    onFileSelect: () => void;
    onRemove: (index: number) => void;
    onRemoveAll: () => void;
  } = $props();
  $inspect(files);

  let oldPrefix = $state("");
  let prefix = $state("");

  const flipDurationMs = 200;

  function handleSort(e: CustomEvent) {
    files = e.detail.items;
  }

  async function onChangePrefix() {
    await tick();
    files.forEach((file) => {
      if (file.remoteFilenamePrefix === oldPrefix) {
        file.remoteFilenamePrefix = prefix;
      }
    });
    oldPrefix = prefix;
  }
</script>

<div class="max-h-96 space-y-2 overflow-y-auto">
  <div class="flex gap-2">
    <button onclick={onFileSelect} class="btn btn-default flex-1">
      <Upload class="size-5" /> 选择文件
    </button>
    <button onclick={onRemoveAll} class="btn btn-danger"> 全部删除 </button>
  </div>

  <div class="flex items-center gap-2">
    <div class="relative flex-1">
      <input
        bind:value={prefix}
        oninput={onChangePrefix}
        class="w-full rounded-lg bg-slate-100 px-4 py-2 text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600"
        placeholder="输入全局前缀"
      />
      <ChevronsUpDown
        class="absolute top-1/2 right-3 size-4 -translate-y-1/2 text-slate-400"
      />
    </div>
  </div>
  <section
    use:dragHandleZone={{ items: files, flipDurationMs }}
    onconsider={handleSort}
    onfinalize={handleSort}
  >
    {#each files as file, index (file.id)}
      <div
        class="flex items-center gap-3 rounded-lg bg-slate-50 p-2 dark:bg-slate-700"
        animate:flip={{ duration: flipDurationMs }}
      >
        <div use:dragHandle>三</div>
        <input
          type="checkbox"
          bind:checked={file.selected}
          class="size-4 rounded border-slate-300 text-blue-600 focus:ring-blue-500 dark:border-slate-600 dark:bg-slate-700"
        />
        <div class="flex-1 space-y-1">
          <div class="flex items-center gap-2">
            <input
              bind:value={file.remoteFilenamePrefix}
              class="w-24 rounded bg-slate-100 px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600"
              placeholder="前缀"
            />
            <input
              bind:value={file.remoteFilename}
              class="flex-1 rounded bg-slate-100 px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600"
              placeholder="远程文件名"
            />
          </div>
          <div class="text-xs text-slate-500 dark:text-slate-400">
            {file.filename}
          </div>
        </div>
        <button
          onclick={() => onRemove(index)}
          class="rounded p-1 text-slate-500 hover:bg-slate-200 dark:text-slate-400 dark:hover:bg-slate-600"
        >
          <X class="size-4" />
        </button>
      </div>
    {/each}
  </section>
</div>
