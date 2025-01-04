<script lang="ts">
  import AddRemoteTargetModal from "$lib/components/AddBucket.svelte";
  import db from "$lib/db";
  import {
    addCustomProxy,
    proxySettings,
    removeCustomProxy,
    selectCustomProxy,
  } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { onMount } from "svelte";
  let showAddProxyModal = false;

  // 上传目标管理相关状态
  let buckets: Array<Bucket> = [];

  onMount(async () => {
    buckets = await db.buckets.toArray();
  });

  async function deleteTarget(id: number) {
    await db.buckets.delete(id);
    buckets = await db.buckets.toArray();
  }

  async function onAddBucketClose() {
    buckets = await db.buckets.toArray();
  }
</script>

<div class="settings-container">
  <h1 class="settings-title">Settings</h1>

  <div class="settings-section">
    <div class="flex items-center justify-between">
      <h2 class="section-title">Buckets</h2>
      <AddRemoteTargetModal onclose={onAddBucketClose} />
    </div>
    <div class="targets-list">
      {#each buckets as bucket}
        <div
          class="flex items-center justify-between border-b border-slate-600 px-2 py-1 last:border-b-0"
        >
          <div class="flex-1">
            <div class="target-details">
              <p>Bucket: {bucket.bucketName}</p>
              <p>Account ID: {bucket.accountId}</p>
            </div>
          </div>
          <button
            class="button button-danger text-sm"
            onclick={() => deleteTarget(bucket.id!)}
          >
            Delete
          </button>
        </div>
      {/each}
    </div>
  </div>

  <div class="settings-section">
    <h2 class="section-title">代理设置</h2>
    <div class="radio-group">
      <label class="radio-item">
        <input
          type="radio"
          name="proxyType"
          value="system"
          bind:group={proxySettings.proxyType}
        />
        <span>使用系统代理</span>
      </label>
      <label class="radio-item">
        <input
          type="radio"
          name="proxyType"
          value="custom"
          bind:group={proxySettings.proxyType}
        />
        <span>使用自定义代理</span>
      </label>
      <label class="radio-item">
        <input
          type="radio"
          name="proxyType"
          value="none"
          bind:group={proxySettings.proxyType}
        />
        <span>禁用代理</span>
      </label>
    </div>

    {#if proxySettings.proxyType === "custom"}
      <div class="proxy-list">
        <button class="add-button" onclick={() => (showAddProxyModal = true)}>
          添加代理
        </button>
        {#each proxySettings.customProxies as proxy}
          <div class="proxy-item">
            <div class="proxy-info">
              <p>{proxy.host}:{proxy.port}</p>
              {#if proxy.username}
                <p class="proxy-username">用户名：{proxy.username}</p>
              {/if}
            </div>
            <div class="proxy-actions">
              <input
                type="radio"
                name="selectedProxy"
                checked={proxySettings.selectedCustomProxyId === proxy.id}
                onchange={() => selectCustomProxy(proxy.id)}
              />
              <button
                class="delete-button"
                onclick={() => removeCustomProxy(proxy.id)}
              >
                删除
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style lang="postcss">
  .settings-container {
    @apply mx-auto max-w-3xl space-y-2 py-2;
  }

  .settings-title {
    @apply mb-3 text-xl font-bold dark:text-white;
  }

  .settings-section {
    @apply space-y-2 bg-white px-4 py-2 shadow dark:bg-slate-800;
  }

  .section-title {
    @apply font-semibold dark:text-slate-200;
  }

  .radio-group {
    @apply space-y-1;
  }

  .radio-item {
    @apply flex cursor-pointer items-center gap-2 py-1;
  }

  .proxy-list {
    @apply mt-2 space-y-1;
  }

  .proxy-item {
    @apply flex items-center justify-between rounded-lg p-1 hover:bg-slate-50 dark:hover:bg-slate-700;
  }

  .proxy-info {
    @apply text-sm;
  }

  .proxy-username {
    @apply text-xs text-slate-500 dark:text-slate-400;
  }

  .proxy-actions {
    @apply flex items-center gap-2;
  }

  .targets-list {
    @apply mt-2 space-y-1;
  }

  .target-details {
    @apply mt-1 text-xs text-slate-500 dark:text-slate-400;
  }
</style>
