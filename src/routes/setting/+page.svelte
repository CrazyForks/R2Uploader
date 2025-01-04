<script lang="ts">
  import AddRemoteTargetModal from "$lib/components/AddBucket.svelte";
  import { Select } from "bits-ui";
  import db from "$lib/db";
  import { appSettings } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { onMount } from "svelte";
  import { ChevronsUpDown } from "lucide-svelte";
  import { t } from "$lib/i18n.svelte";

  const languages = [
    { value: "en", label: "English" },
    { value: "zh", label: "中文" },
  ];

  // 上传目标管理相关状态
  let buckets: Array<Bucket> = [];

  onMount(async () => {
    buckets = await db.buckets.toArray();
  });

  async function deleteTarget(id: number) {
    await db.buckets.delete(id);
    buckets = await db.buckets.toArray();
  }

  async function onAddBucketClose() {
    buckets = await db.buckets.toArray();
  }
</script>

<div class="settings-container">
  <h1 class="px-4 py-1 text-lg font-bold">{t().common.setting}</h1>

  <div class="settings-section">
    <div class="flex items-center justify-between">
      <h2 class="section-title">{t().settings.buckets}</h2>
      <AddRemoteTargetModal onclose={onAddBucketClose} />
    </div>
    <div class="targets-list">
      {#each buckets as bucket}
        <div
          class="flex items-center justify-between border-b border-slate-600 px-2 py-1 last:border-b-0"
        >
          <div class="flex-1">
            <div class="target-details">
              <p>{t().settings.bucketDetails.bucket}: {bucket.bucketName}</p>
              <p>{t().settings.bucketDetails.accountId}: {bucket.accountId}</p>
            </div>
          </div>
          <button
            class="button button-danger text-sm"
            onclick={() => deleteTarget(bucket.id!)}
          >
            {t().common.delete}
          </button>
        </div>
      {/each}
    </div>
  </div>

  <div class="settings-section">
    <div class="flex items-center justify-between">
      <span class="text-sm">{t().common.useSystemProxy}</span>
      <label class="relative inline-flex cursor-pointer items-center">
        <input
          type="checkbox"
          class="toggle border-slate-600 checked:border-cyan-600 checked:bg-cyan-500 checked:text-white"
          bind:checked={appSettings.useSystemProxy}
        />
      </label>
    </div>
  </div>

  <div class="settings-section">
    <div class="flex items-center justify-between">
      <span class="text-sm">{t().settings.language}</span>
      <Select.Root
        items={languages}
        onSelectedChange={(selected) => {
          if (selected) {
            appSettings.locale = selected.value;
          }
        }}
      >
        <Select.Trigger class="select-trigger">
          <Select.Value placeholder={t().common.selectLanguage} />
          <ChevronsUpDown
            class="dark-text-slate-300 ml-auto size-4 text-slate-400"
          />
        </Select.Trigger>
        <Select.Content class="select-content">
          {#each languages as lang}
            <Select.Item
              value={lang.value}
              label={lang.label}
              class="select-item"
            >
              {lang.label}
            </Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
  </div>
</div>

<style lang="postcss">
  .settings-container {
    @apply mx-auto max-w-3xl space-y-2 py-2;
  }

  .settings-section {
    @apply space-y-2 bg-white px-4 py-2 shadow dark:bg-slate-800;
  }

  .section-title {
    @apply font-semibold dark:text-slate-200;
  }

  .targets-list {
    @apply mt-2 space-y-1;
  }

  .target-details {
    @apply mt-1 text-xs text-slate-500 dark:text-slate-400;
  }
</style>
