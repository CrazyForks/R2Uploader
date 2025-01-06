<script lang="ts">
  import BucketSelector from "$lib/components/BucketSelector.svelte";
  import FileUploader from "$lib/components/FileUploader.svelte";
  import { t } from "$lib/i18n.svelte";
</script>

<div class="mx-auto flex h-full max-w-4xl flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
    {t().common.upload}
  </h1>

  <BucketSelector />

  <FileUploader />
</div>
