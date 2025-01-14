<script lang="ts">
  import FileUploaderProgress from "$lib/components/FileUploaderProgress.svelte";
  import { t } from "$lib/i18n.svelte";

  let activeTab: "all" | "in-progress" | "completed" = $state("all");

  const tabs: { id: "all" | "in-progress" | "completed"; label: string }[] = [
    { id: "all", label: t().transfer.tabs.all },
    { id: "in-progress", label: t().transfer.tabs.inProgress },
    { id: "completed", label: t().transfer.tabs.completed },
  ];
</script>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">{t().transfer.title}</h1>

  <div class="flex gap-2">
    {#each tabs as tab}
      <button
        class="nav-link gapped rounded-lg"
        class:bg={activeTab === tab.id}
        aria-current={activeTab === tab.id || undefined}
        onclick={() => (activeTab = tab.id)}
      >
        {tab.label}
      </button>
    {/each}
  </div>
  <div
    class="flex min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
  >
    <FileUploaderProgress {activeTab} />
  </div>
</div>

<style lang="postcss">
  .nav-link {
    @apply flex cursor-pointer items-center text-slate-700 transition-colors dark:text-slate-200;
  }

  .gapped {
    @apply px-4 py-2;
  }

  .nav-link[aria-current] {
    @apply text-cyan-600 dark:text-cyan-400;
  }

  .nav-link[aria-current].bg {
    @apply bg-cyan-50 dark:bg-cyan-900/30;
  }
</style>
