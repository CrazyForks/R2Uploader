<script lang="ts">
  import BucketSelector from "$lib/components/BucketSelector.svelte";
  import FileUploader from "$lib/components/FileUploader.svelte";
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import { setAlert } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import type { Selected } from "bits-ui";
  import { Check } from "lucide-svelte";
  import { onMount } from "svelte";
  import clipboard from "tauri-plugin-clipboard-api";

  let uploadStatus = $state<"idle" | "uploading" | "success" | "error">("idle");
  let uploadStatusMap = $state<Record<string, string>>({});
  let intervalId = $state<number | undefined>();
  let clipboardFiles = $state<string[]>([]);
  let clipboardText = $state("");
  let clipboardHtml = $state("");
  let clipboardImage = $state("");
  let clipboardRtf = $state("");
  let buckets: Selected<Bucket>[] = $state([]);
  let selectedBucket: Selected<Bucket> | undefined = $state();
  let textContent = $state("");

  onMount(async () => {
    getBuckets();
    // await checkClipboardContent();
  });

  async function getBuckets() {
    await db.buckets.toArray().then((targets) => {
      buckets = targets.map((target) => ({
        value: target,
        label: target.bucketName,
      }));
    });
    if (buckets.length > 0) {
      selectedBucket = buckets[0];
    }
  }

  async function checkClipboardContent() {
    try {
      if (await clipboard.hasText()) {
        clipboardText = await clipboard.readText();
      }
      if (await clipboard.hasHTML()) {
        clipboardHtml = await clipboard.readHtml();
      }
      if (await clipboard.hasImage()) {
        clipboardImage = await clipboard.readImageBase64();
      }
      if (await clipboard.hasRTF()) {
        clipboardRtf = await clipboard.readRtf();
      }
      if (await clipboard.hasFiles()) {
        clipboardFiles = await clipboard.readFiles();
      }
    } catch (error: unknown) {
      console.error(error);
      setAlert(t().common.clipboardReadError);
    }
  }
</script>

<div class="mx-auto flex h-full max-w-4xl flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
    {t().common.upload}
  </h1>

  {#if buckets.length === 0}
    <div
      class="rounded-lg border border-yellow-300 bg-yellow-50 p-4 text-yellow-800 dark:border-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-200"
    >
      {t().common.noBucketWarning}
    </div>
  {:else}
    <BucketSelector
      {buckets}
      {selectedBucket}
      onSelectedChange={(e) => {
        if (e) {
          selectedBucket = {
            value: e.value,
            label: e.value.bucketName,
          };
        }
      }}
    />
  {/if}
  <div
    class="flex flex-1 items-center justify-center rounded-lg border border-slate-300 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
  >
    {#if buckets.length}
      <FileUploader selectedTarget={selectedBucket} />
    {:else}
      <p class="dark:text-slate-300">您尚未设置存储桶，无法操作</p>
    {/if}
  </div>
</div>
