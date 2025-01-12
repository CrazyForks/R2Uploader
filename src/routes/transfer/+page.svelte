<script lang="ts">
  import FileUploaderProgress from "$lib/components/FileUploaderProgress.svelte";

  let activeTab: "all" | "in-progress" | "completed" = "all";
</script>

<div class="container mx-auto p-4">
  <div class="mb-4 flex gap-2">
    <button
      class="rounded-lg px-4 py-2 {activeTab === 'all'
        ? 'bg-cyan-50 text-cyan-600 dark:bg-cyan-900/30 dark:text-cyan-400'
        : 'bg-slate-100 dark:bg-slate-700'}"
      on:click={() => (activeTab = "all")}
    >
      全部
    </button>
    <button
      class="rounded-lg px-4 py-2 {activeTab === 'in-progress'
        ? 'bg-cyan-50 text-cyan-600 dark:bg-cyan-900/30 dark:text-cyan-400'
        : 'bg-slate-100 dark:bg-slate-700'}"
      on:click={() => (activeTab = "in-progress")}
    >
      传输中
    </button>
    <button
      class="rounded-lg px-4 py-2 {activeTab === 'completed'
        ? 'bg-cyan-50 text-cyan-600 dark:bg-cyan-900/30 dark:text-cyan-400'
        : 'bg-slate-100 dark:bg-slate-700'}"
      on:click={() => (activeTab = "completed")}
    >
      已完成
    </button>
  </div>

  {#if activeTab === "all"}
    <FileUploaderProgress />
  {:else if activeTab === "in-progress"}
    <FileUploaderProgress />
  {:else}
    <FileUploaderProgress />
  {/if}
</div>

<style>
  .container {
    max-width: 1200px;
  }
</style>
