<script lang="ts">
  import AddRemoteTargetModal from "$lib/components/AddRemoteTargetModal.svelte";
  import db from "$lib/db";
  import {
    addCustomProxy,
    proxySettings,
    removeCustomProxy,
    selectCustomProxy,
  } from "$lib/store.svelte";
  import { onMount } from "svelte";
  let showAddProxyModal = false;
  let showAddTargetModal = false;

  // 上传目标管理相关状态
  let targets: Array<{
    id?: number;
    name: string;
    description: string;
    bucketName: string;
    accountId: string;
    accessKey: string;
    secretKey: string;
  }> = [];

  let newTarget = {
    name: "",
    description: "",
    bucketName: "",
    accountId: "",
    accessKey: "",
    secretKey: "",
  };

  onMount(async () => {
    targets = await db.uploadTargets.toArray();
  });

  // 上传目标管理功能
  async function addTarget() {
    const id = await db.uploadTargets.add({
      ...newTarget,
      type: "r2",
    });
    targets = await db.uploadTargets.toArray();
    newTarget = {
      name: "",
      description: "",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
    };
  }

  async function deleteTarget(id: number) {
    await db.uploadTargets.delete(id);
    targets = await db.uploadTargets.toArray();
  }
</script>

<div class="mx-auto max-w-3xl px-3 py-4">
  <h1 class="mb-4 text-xl font-bold dark:text-white">设置</h1>

  <div class="config-card mb-4">
    <h2 class="mb-2 text-lg font-semibold dark:text-slate-200">代理设置</h2>

    <div class="space-y-1">
      <label class="flex items-center gap-2 py-1">
        <input
          type="radio"
          name="proxyType"
          value="system"
          bind:group={proxySettings.proxyType}
          class="form-radio"
        />
        <span class="text-sm dark:text-slate-300">使用系统代理</span>
      </label>

      <label class="flex items-center gap-2 py-1">
        <input
          type="radio"
          name="proxyType"
          value="custom"
          bind:group={proxySettings.proxyType}
          class="form-radio"
        />
        <span class="text-sm dark:text-slate-300">使用自定义代理</span>
      </label>

      <label class="flex items-center gap-2 py-1">
        <input
          type="radio"
          name="proxyType"
          value="none"
          bind:group={proxySettings.proxyType}
          class="form-radio"
        />
        <span class="text-sm dark:text-slate-300">禁用代理</span>
      </label>
    </div>

    {#if proxySettings.proxyType === "custom"}
      <div class="space-y-2">
        <button
          onclick={() => (showAddProxyModal = true)}
          class="form-button text-sm px-3 py-1"
        >
          添加代理
        </button>

        {#each proxySettings.customProxies as proxy}
          <div class="config-card p-2">
            <div class="flex items-center justify-between gap-2">
              <div class="text-sm">
                <p class="dark:text-slate-300">{proxy.host}:{proxy.port}</p>
                {#if proxy.username}
                  <p class="text-xs dark:text-slate-400">
                    用户名：{proxy.username}
                  </p>
                {/if}
              </div>

              <div class="flex items-center gap-1">
                <input
                  type="radio"
                  name="selectedProxy"
                  checked={proxySettings.selectedCustomProxyId === proxy.id}
                  onchange={() => selectCustomProxy(proxy.id)}
                  class="form-radio"
                />
                <button
                  onclick={() => removeCustomProxy(proxy.id)}
                  class="delete-button text-sm px-2 py-0.5"
                >
                  删除
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <h2 class="mb-4 text-xl font-bold dark:text-white">上传目标管理</h2>

  <div class="config-card mb-4 p-2">
    <AddRemoteTargetModal />
  </div>

  <div class="config-card p-2">
    <h2 class="mb-2 text-lg font-semibold dark:text-slate-200">现有配置</h2>
    <div class="space-y-2">
      {#each targets as target}
        <div class="config-card p-2">
          <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
            <div>
              <h3 class="text-sm font-medium dark:text-slate-200">{target.name}</h3>
              <p class="text-xs text-slate-600 dark:text-slate-400">
                {target.description}
              </p>
            </div>
            <div>
              <p class="text-xs dark:text-slate-300">
                Bucket: {target.bucketName}
              </p>
              <p class="text-xs dark:text-slate-300">
                Account ID: {target.accountId}
              </p>
            </div>
          </div>
          <button
            class="delete-button mt-1 text-sm px-2 py-0.5"
            onclick={() => deleteTarget(target.id!)}
          >
            删除
          </button>
        </div>
      {/each}
    </div>
  </div>
</div>


<style lang="postcss">
  .form-input {
    @apply w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-slate-900 transition-colors duration-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-500 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 dark:focus:border-blue-600 dark:focus:ring-blue-600;
  }

  .form-button {
    @apply rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors duration-200 hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-slate-800;
  }

  .delete-button {
    @apply rounded-md bg-red-600 px-3 py-1 text-white transition-colors duration-200 hover:bg-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-slate-800;
  }

  .config-card {
    @apply rounded-lg border border-slate-200 bg-white p-4 text-slate-900 shadow-sm dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100;
  }
</style>
