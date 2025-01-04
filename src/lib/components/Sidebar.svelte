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

  const links = [
    { href: "/", icon: Plus, label: "common.upload" },
    { href: "/setting", icon: Settings, label: "common.setting" },
  ];
</script>

{#snippet Desktop()}
  <nav
    class="hidden min-h-dvh border-r border-slate-200 bg-white transition-all md:block dark:border-slate-700 dark:bg-slate-800"
    style="width: {appSettings.sidebarCollapsed ? '3rem' : '9rem'}"
  >
    <ul class="flex flex-col items-center space-y-2 overflow-hidden">
      <li
        class="flex w-full {appSettings.sidebarCollapsed
          ? 'justify-center'
          : 'justify-end'}"
      >
        <button
          onclick={() =>
            (appSettings.sidebarCollapsed = !appSettings.sidebarCollapsed)}
          class="sidebar-link gapped"
        >
          {#if appSettings.sidebarCollapsed}
            <PanelRightClose class="size-5" />
          {:else}
            <PanelRightOpen class="size-5" />
          {/if}
        </button>
      </li>
      {#each links as { href, icon: Icon, label }}
        <li>
          <a
            {href}
            class="sidebar-link gapped bg {appSettings.sidebarCollapsed
              ? 'rounded-none'
              : 'rounded-xl'}"
            class:min-w-28={!appSettings.sidebarCollapsed}
            aria-current={page.route.id === href ? "page" : null}
          >
            <Icon class="size-5" />
            {#if !appSettings.sidebarCollapsed}
              <span class="text-nowrap">{$t(label)}</span>
            {/if}
          </a>
        </li>
      {/each}
    </ul>
  </nav>
{/snippet}

{#snippet Mobile()}
  <div class="fixed inset-x-0 bottom-0 md:hidden">
    <nav
      class="flex items-center justify-around border-t border-slate-200 bg-white px-4 py-3 dark:border-slate-700 dark:bg-slate-700"
    >
      {#each links as { href, icon: Icon, label }}
        <a
          {href}
          class="sidebar-link flex-col gap-1"
          aria-current={page.route.id === href ? "page" : null}
        >
          <Icon class="size-5" />
          <span class="text-nowrap">{$t(label)}</span>
        </a>
      {/each}
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
