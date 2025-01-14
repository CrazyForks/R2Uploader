<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import { globalState } from "$lib/store.svelte";
  import confetti from "canvas-confetti";
  import { onMount } from "svelte";
  import TrialStatus from "$lib/components/TrialStatus.svelte";

  let activationCode = $state("");
  let error = $state("");
  let isActivated = $state(false);

  onMount(() => {
    isActivated = globalState.appSetting.activated || false;
  });

  function triggerConfetti() {
    const duration = 3 * 1000;
    const end = Date.now() + duration;

    (function frame() {
      confetti({
        particleCount: 7,
        angle: 60,
        spread: 55,
        origin: { x: 0 },
        colors: ["#00b4d8", "#0077b6", "#48cae4"],
      });
      confetti({
        particleCount: 7,
        angle: 120,
        spread: 55,
        origin: { x: 1 },
        colors: ["#00b4d8", "#0077b6", "#48cae4"],
      });

      if (Date.now() < end) {
        requestAnimationFrame(frame);
      }
    })();
  }

  async function handleActivate() {
    error = "";
    // TODO: Implement actual activation logic
    if (activationCode === "test123") {
      isActivated = true;
      globalState.appSetting.activated = true;
      triggerConfetti();
    } else {
      error = t().activate.error;
    }
  }
</script>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
    {t().activate.title}
  </h1>

  <div
    class="flex min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 bg-slate-100/80 dark:border-slate-700 dark:bg-slate-800"
  >
    {#if isActivated}
      <div
        class="flex w-full flex-col items-center justify-center gap-6 p-8 text-center"
      >
        <div class="rounded-full bg-cyan-100 p-4 dark:bg-cyan-900">
          <svg
            class="h-12 w-12 text-cyan-500"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        </div>
        <div class="space-y-2">
          <h2 class="text-3xl font-bold text-slate-800 dark:text-slate-200">
            {t().activate.activated.title}
          </h2>
          <p class="text-xl text-slate-600 dark:text-slate-300">
            {t().activate.activated.message}
          </p>
          <p class="text-base text-slate-500 dark:text-slate-400">
            {t().activate.activated.subtitle}
          </p>
        </div>
      </div>
    {:else}
      <div class="flex w-full flex-col items-center justify-center p-4">
        <div class="w-full max-w-md space-y-4">
          <TrialStatus showStartButton={true} />

          <div class="rounded-lg bg-white p-6 shadow-md dark:bg-gray-800">
            <h2
              class="mb-6 text-center text-xl font-bold text-slate-800 dark:text-slate-200"
            >
              {t().activate.form.title}
            </h2>

            <div class="space-y-4">
              <div>
                <label
                  for="activation-code"
                  class="mb-2 block text-sm font-medium text-slate-700 dark:text-slate-300"
                >
                  {t().activate.form.codeLabel}
                </label>
                <input
                  type="text"
                  id="activation-code"
                  bind:value={activationCode}
                  placeholder={t().activate.form.codePlaceholder}
                  class="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-slate-800 focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 focus:outline-none dark:border-slate-600 dark:bg-slate-700 dark:text-slate-200"
                />
                {#if error}
                  <p class="mt-1 text-sm text-red-500">{error}</p>
                {/if}
              </div>

              <button
                onclick={handleActivate}
                class="w-full rounded-md bg-cyan-500 px-4 py-2 text-white transition-colors hover:bg-cyan-600"
              >
                {t().activate.form.submit}
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
