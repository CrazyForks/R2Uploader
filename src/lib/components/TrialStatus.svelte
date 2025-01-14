<script lang="ts">
  import { goto } from "$app/navigation";
  import { t } from "$lib/i18n.svelte";
  import { globalState } from "$lib/store.svelte";
  import { onMount } from "svelte";

  let { showStartButton }: { showStartButton: boolean } = $props();

  let daysLeft = $state(0);
  let isExpired = $state(false);

  // 使用$derived 计算是否显示
  let shouldShow = $derived(!globalState.appSetting.activated);

  // 使用$derived 计算试用状态文本
  let trialText = $derived(() => {
    if (!globalState.appSetting.trialStartDate) {
      return t().activate.trial.start.replace(
        "{days}",
        globalState.appSetting.trialDays.toString(),
      );
    }
    return t().activate.trial.daysLeft.replace("{days}", daysLeft.toString());
  });

  function calculateDaysLeft(): number {
    if (!globalState.appSetting.trialStartDate) {
      return globalState.appSetting.trialDays;
    }

    const now = Date.now();
    const trialEnd =
      globalState.appSetting.trialStartDate +
      globalState.appSetting.trialDays * 24 * 60 * 60 * 1000;
    const remainingMs = trialEnd - now;
    return Math.max(0, Math.ceil(remainingMs / (24 * 60 * 60 * 1000)));
  }

  function startTrial() {
    globalState.appSetting.trialStartDate = Date.now();
    daysLeft = calculateDaysLeft();
  }

  function checkTrialStatus() {
    if (!shouldShow) return;

    if (!globalState.appSetting.trialStartDate) {
      daysLeft = globalState.appSetting.trialDays;
      isExpired = false;
      return;
    }

    daysLeft = calculateDaysLeft();
    isExpired = daysLeft <= 0;

    if (isExpired && window.location.pathname !== "/activate") {
      goto("/activate");
    }
  }

  onMount(() => {
    checkTrialStatus();
    const interval = setInterval(checkTrialStatus, 60 * 1000);
    return () => clearInterval(interval);
  });
</script>

{#if shouldShow}
  {#if !globalState.appSetting.trialStartDate && showStartButton}
    <button
      onclick={startTrial}
      class="w-full rounded-md bg-cyan-500 px-4 py-2 text-white transition-colors hover:bg-cyan-600"
    >
      {trialText}
    </button>
  {:else if isExpired}
    <div class="rounded-md bg-red-50 p-4 dark:bg-red-900/30">
      <div class="flex flex-col gap-1">
        <p class="font-medium text-red-800 dark:text-red-200">
          {t().activate.trial.expired}
        </p>
        <p class="text-sm text-red-700 dark:text-red-300">
          {t().activate.trial.needActivation}
        </p>
      </div>
    </div>
  {:else}
    <div class="rounded-md bg-blue-50 p-4 dark:bg-blue-900/30">
      <p class="text-sm font-medium text-blue-800 dark:text-blue-200">
        {trialText}
      </p>
    </div>
  {/if}
{/if}
