<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  
  let activationCode = "";

  function handleActivate() {
    // TODO: Implement activation logic
    console.log("Activation code:", activationCode);
  }
</script>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">{t().activate.title}</h1>

  <div class="flex min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 bg-slate-100/80 dark:border-slate-700 dark:bg-slate-800">
    <div class="flex w-full flex-col items-center justify-center p-4">
      <div class="w-full max-w-md rounded-lg bg-white p-6 shadow-md dark:bg-gray-800">
        <h2 class="mb-6 text-xl font-bold text-center text-slate-800 dark:text-slate-200">
          {t().activate.form.title}
        </h2>
        
        <div class="space-y-4">
          <div>
            <label for="activation-code" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              {t().activate.form.codeLabel}
            </label>
            <input
              type="text"
              id="activation-code"
              bind:value={activationCode}
              placeholder={t().activate.form.codePlaceholder}
              class="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-slate-800 focus:border-cyan-500 focus:outline-none focus:ring-1 focus:ring-cyan-500 dark:border-slate-600 dark:bg-slate-700 dark:text-slate-200"
            />
          </div>
          
          <button
            on:click={handleActivate}
            class="w-full rounded-md bg-cyan-500 px-4 py-2 text-white hover:bg-cyan-600 transition-colors"
          >
            {t().activate.form.submit}
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
