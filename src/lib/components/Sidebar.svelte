<script lang="ts">
  import { page } from "$app/state";
  import { appSettings } from "$lib/store.svelte";
  import { t } from "$lib/i18n/i18n";
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

{#snippet Desktop()}
  <nav
    class="hidden min-h-dvh border-r border-slate-200 bg-white transition-all md:block dark:border-slate-700 dark:bg-slate-800 {appSettings.sidebarCollapsed
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
          class="sidebar-link gapped bg {getRounded()}"
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
          class="sidebar-link gapped bg {getRounded()}"
          class:min-w-28={!appSettings.sidebarCollapsed}
          aria-current={page.route.id === "/" ? "page" : null}
        >
          <Plus class="size-5" />
          {#if !appSettings.sidebarCollapsed}
            <span class="text-nowrap">{$t("common.upload")}</span>
          {/if}
        </a>
      </li>
      <li>
        <a
          href="/setting"
          class="sidebar-link gapped bg {getRounded()}"
          class:min-w-28={!appSettings.sidebarCollapsed}
          aria-current={page.route.id === "/setting" ? "page" : null}
        >
          <Settings class="size-5" />
          {#if !appSettings.sidebarCollapsed}
            <span class="text-nowrap">{$t("common.setting")}</span>
          {/if}
        </a>
      </li>
    </ul>
  </nav>
{/snippet}

{#snippet Mobile()}
  <div class="fixed right-0 bottom-0 left-0 md:hidden">
    <nav
      class="flex items-center justify-around
      border-t border-slate-200 bg-white px-4 py-3 dark:border-slate-700 dark:bg-slate-700"
    >
      <a
        href="/"
        class="sidebar-link flex flex-col gap-1"
        aria-current={page.route.id === "/" ? "page" : null}
      >
        <Plus class="size-5" />
        <span class="text-nowrap">{$t("common.upload")}</span>
      </a>
      <a
        href="/setting"
        class="sidebar-link flex-col gap-1"
        aria-current={page.route.id === "/setting" ? "page" : null}
      >
        <Settings class="size-5" />
        <span class="text-nowrap">{$t("common.setting")}</span>
      </a>
    </nav>
  </div>
{/snippet}

{@render Desktop()}
{@render Mobile()}

<style lang="postcss">
  .sidebar-link {
    @apply flex cursor-pointer items-center text-slate-700 transition-colors dark:text-slate-200;
  }

  .gapped {
    @apply gap-3 px-4 py-3;
  }

  .sidebar-link[aria-current] {
    @apply text-cyan-600 dark:text-cyan-400;
  }

  .sidebar-link[aria-current].bg {
    @apply bg-cyan-50 dark:bg-cyan-900/30;
  }
</style>
