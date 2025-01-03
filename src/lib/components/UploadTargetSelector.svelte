<script lang="ts">
  import type { UploadTarget } from "$lib/db";
  import { Select, type Selected } from "bits-ui";
  import { ChevronsUpDown } from "lucide-svelte";

  let {
    uploadTargets,
    selectedTarget,
    onSelectedChange,
  }: {
    uploadTargets: Selected<UploadTarget>[];
    selectedTarget: Selected<UploadTarget> | undefined;
    onSelectedChange: (e: Selected<UploadTarget> | undefined) => void;
  } = $props();
</script>

<div class="mb-4 flex items-center gap-4">
  <p class="text-sm font-medium text-slate-700 dark:text-slate-300">上传目标</p>
  <Select.Root
    items={uploadTargets}
    selected={selectedTarget}
    {onSelectedChange}
  >
    <Select.Trigger class="select-trigger">
      <Select.Value placeholder="选择上传目标" />
      <ChevronsUpDown
        class="dark-text-slate-300 ml-auto size-4 text-slate-400"
      />
    </Select.Trigger>
    <Select.Content class="select-content">
      {#each uploadTargets as target}
        <Select.Item
          value={target.value}
          label={target.label}
          class="select-item"
        >
          {target.label}
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>
