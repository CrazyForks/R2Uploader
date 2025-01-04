<script lang="ts">
  import { page } from "$app/state";
  import { appSettings } from "$lib/store.svelte";
  import {
    PanelRightClose,
    PanelRightOpen,
    Plus,
    Settings,
  } from "lucide-svelte";

  function getRounded() {
    return appSettings.sidebarCollapsed ? "rounded-none" : "rounded-xl";
  }
</script>

<nav
  class="min-h-dvh border-r border-slate-200 bg-white transition-all dark:border-slate-700 dark:bg-slate-800 {appSettings.sidebarCollapsed
    ? 'w-12'
    : 'w-36'}"
>
  <ul
    class="flex flex-col items-center justify-center space-y-2 overflow-hidden"
  >
    <li
      class="flex w-full items-center {appSettings.sidebarCollapsed
        ? 'justify-center'
        : 'justify-end'}"
    >
      <button
        onclick={() =>
          (appSettings.sidebarCollapsed = !appSettings.sidebarCollapsed)}
        class="sidebar-link {getRounded()}"
      >
        {#if !appSettings.sidebarCollapsed}
          <PanelRightOpen class="size-5" />
        {:else}
          <PanelRightClose class="size-5" />
        {/if}
      </button>
    </li>
    <li>
      <a
        href="/"
        class="sidebar-link {getRounded()}"
        aria-current={page.route.id === "/" ? "page" : null}
      >
        <Plus class="size-5" />
        {#if !appSettings.sidebarCollapsed}
          <span class="text-nowrap">Upload</span>
        {/if}
      </a>
    </li>
    <li>
      <a
        href="/setting"
        class="sidebar-link {getRounded()}"
        aria-current={page.route.id === "/setting" ? "page" : null}
      >
        <Settings class="size-5" />
        {#if !appSettings.sidebarCollapsed}
          <span class="text-nowrap">Setting</span>
        {/if}
      </a>
    </li>
  </ul>
</nav>

<style lang="postcss">
  .sidebar-link {
    @apply flex cursor-pointer items-center gap-3 px-4 py-3 text-slate-700 transition-colors dark:text-slate-200;
  }

  .sidebar-link[aria-current] {
    @apply bg-cyan-50 text-cyan-600 dark:bg-cyan-900/30 dark:text-cyan-400;
  }
</style>
