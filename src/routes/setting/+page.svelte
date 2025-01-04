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

  // 新代理表单状态
  let newProxy = {
    host: "",
    port: 0,
    username: "",
    password: "",
  };

  // 添加代理
  function addProxy() {
    if (!newProxy.host || !newProxy.port) return;

    addCustomProxy({
      host: newProxy.host,
      port: newProxy.port,
      username: newProxy.username || undefined,
      password: newProxy.password || undefined,
    });

    // 重置表单
    newProxy = {
      host: "",
      port: 0,
      username: "",
      password: "",
    };

    showAddProxyModal = false;
  }

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

<div class="mx-auto max-w-4xl px-4 py-8">
  <h1 class="mb-8 text-2xl font-bold dark:text-white">设置</h1>

  <div class="config-card mb-8">
    <h2 class="mb-4 text-xl font-semibold dark:text-slate-200">代理设置</h2>

    <div class="mb-4 space-y-2">
      <label class="flex items-center space-x-2">
        <input
          type="radio"
          name="proxyType"
          value="system"
          bind:group={proxySettings.proxyType}
          class="form-radio"
        />
        <span class="dark:text-slate-300">使用系统代理</span>
      </label>

      <label class="flex items-center space-x-2">
        <input
          type="radio"
          name="proxyType"
          value="custom"
          bind:group={proxySettings.proxyType}
          class="form-radio"
        />
        <span class="dark:text-slate-300">使用自定义代理</span>
      </label>

      <label class="flex items-center space-x-2">
        <input
          type="radio"
          name="proxyType"
          value="none"
          bind:group={proxySettings.proxyType}
          class="form-radio"
        />
        <span class="dark:text-slate-300">禁用代理</span>
      </label>
    </div>

    {#if proxySettings.proxyType === "custom"}
      <div class="space-y-4">
        <button onclick={() => (showAddProxyModal = true)} class="form-button">
          添加代理
        </button>

        {#each proxySettings.customProxies as proxy}
          <div class="config-card">
            <div class="flex items-center justify-between">
              <div>
                <p class="dark:text-slate-300">{proxy.host}:{proxy.port}</p>
                {#if proxy.username}
                  <p class="text-sm dark:text-slate-400">
                    用户名：{proxy.username}
                  </p>
                {/if}
              </div>

              <div class="flex items-center space-x-2">
                <input
                  type="radio"
                  name="selectedProxy"
                  checked={proxySettings.selectedCustomProxyId === proxy.id}
                  onchange={() => selectCustomProxy(proxy.id)}
                  class="form-radio"
                />
                <button
                  onclick={() => removeCustomProxy(proxy.id)}
                  class="delete-button"
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

  <h2 class="mb-8 text-2xl font-bold dark:text-white">上传目标管理</h2>

  <div class="config-card mb-8">
    <AddRemoteTargetModal />
  </div>

  <div class="config-card">
    <h2 class="mb-4 text-xl font-semibold dark:text-slate-200">现有配置</h2>
    <div class="space-y-4">
      {#each targets as target}
        <div class="config-card">
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div>
              <h3 class="font-medium dark:text-slate-200">{target.name}</h3>
              <p class="text-sm text-slate-600 dark:text-slate-400">
                {target.description}
              </p>
            </div>
            <div>
              <p class="text-sm dark:text-slate-300">
                Bucket: {target.bucketName}
              </p>
              <p class="text-sm dark:text-slate-300">
                Account ID: {target.accountId}
              </p>
            </div>
          </div>
          <button
            class="delete-button mt-2"
            onclick={() => deleteTarget(target.id!)}
          >
            删除
          </button>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- 添加代理模态框 -->
{#if showAddProxyModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
  >
    <div class="config-card w-full max-w-md">
      <h2 class="mb-4 text-xl font-semibold dark:text-slate-200">添加代理</h2>

      <div class="space-y-4">
        <input
          bind:value={newProxy.host}
          placeholder="主机地址"
          class="form-input"
        />
        <input
          type="number"
          bind:value={newProxy.port}
          placeholder="端口"
          class="form-input"
        />
        <input
          bind:value={newProxy.username}
          placeholder="用户名 (可选)"
          class="form-input"
        />
        <input
          type="password"
          bind:value={newProxy.password}
          placeholder="密码 (可选)"
          class="form-input"
        />
      </div>

      <div class="mt-4 flex justify-end space-x-2">
        <button
          onclick={() => (showAddProxyModal = false)}
          class="delete-button"
        >
          取消
        </button>
        <button onclick={addProxy} class="form-button"> 添加 </button>
      </div>
    </div>
  </div>
{/if}

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
