<script lang="ts">
  import { Upload, X, GripVertical, Eye } from "lucide-svelte";
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

  let showBigMenu = $derived(!files.length);
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

<div
  class="max-h-96 space-y-2 overflow-y-auto rounded-2xl border border-white/20 bg-white/80 shadow-lg backdrop-blur-lg dark:border-slate-700/20 dark:bg-slate-800/80"
>
  {#if showBigMenu}
    <div class="flex gap-2">
      <button onclick={onFileSelect} class="btn btn-default flex-1">
        <Upload class="size-5" /> 选择文件
      </button>
    </div>
  {:else}
    <div class="">
      <div class="flex h-12 items-center gap-4 bg-slate-900">
        <div class="px-2">
          <input
            bind:value={prefix}
            oninput={onChangePrefix}
            class="input w-36"
            placeholder="全局路径"
          />
        </div>
      </div>
      <section
        use:dragHandleZone={{ items: files, flipDurationMs }}
        onconsider={handleSort}
        onfinalize={handleSort}
        class="space-y-2 px-2 py-4"
      >
        {#each files as file, index (file.id)}
          <div
            class="flex items-center gap-4 rounded-xl bg-slate-50 p-2 dark:bg-slate-700"
            animate:flip={{ duration: flipDurationMs }}
          >
            <div
              use:dragHandle
              class=" text-slate-400 hover:text-slate-500 dark:hover:text-slate-300"
            >
              <GripVertical class="size-4" />
            </div>
            <input
              type="checkbox"
              bind:checked={file.selected}
              class="size-4 rounded-lg border-slate-300 text-blue-600 focus:ring-blue-500 dark:border-slate-600 dark:bg-slate-700"
            />
            <div class="flex-1">
              <div class="flex items-center gap-2">
                <input
                  bind:value={file.remoteFilenamePrefix}
                  class="input w-24"
                  placeholder="远程路径"
                />
                <span>/</span>
                <input
                  bind:value={file.remoteFilename}
                  class="input flex-1"
                  placeholder="远程文件名"
                />
              </div>
            </div>
            <div>
              <button class="action-button">
                <Eye class="size-4" />
              </button>
              <button onclick={() => onRemove(index)} class="action-button">
                <X class="size-4" />
              </button>
            </div>
          </div>
        {/each}
      </section>
    </div>
  {/if}
</div>

<style lang="postcss">
  .input {
    @apply rounded-lg bg-slate-100 px-2 py-1 text-sm placeholder:text-slate-400 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-slate-600;
  }

  .action-button {
    @apply cursor-pointer rounded-lg p-1 text-slate-500 hover:bg-slate-200 dark:text-slate-400 dark:hover:bg-slate-600;
  }
</style>
