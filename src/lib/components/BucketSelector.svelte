<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import type { Bucket } from "$lib/type";
  import { Select, type Selected } from "bits-ui";
  import { ChevronsUpDown } from "lucide-svelte";

  let {
    buckets,
    selectedBucket,
    onSelectedChange,
  }: {
    buckets: Selected<Bucket>[];
    selectedBucket: Selected<Bucket> | undefined;
    onSelectedChange: (e: Selected<Bucket> | undefined) => void;
  } = $props();
</script>

<div class="mb-4 flex items-center gap-4">
  <p class="text-sm font-medium text-slate-700 dark:text-slate-300">
    {t().uploadTargetSelector.title}
  </p>
  <Select.Root items={buckets} selected={selectedBucket} {onSelectedChange}>
    <Select.Trigger class="select-trigger">
      <Select.Value placeholder={t().uploadTargetSelector.placeholder} />
      <ChevronsUpDown
        class="dark-text-slate-300 ml-auto size-4 text-slate-400"
      />
    </Select.Trigger>
    <Select.Content class="select-content">
      {#each buckets as target}
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
