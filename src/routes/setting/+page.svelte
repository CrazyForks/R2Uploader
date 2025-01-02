<script lang="ts">
  import db from "$lib/db";
  import { onMount } from "svelte";

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

<div class="max-w-4xl mx-auto px-4 py-8">
  <h1 class="text-2xl font-bold mb-8 dark:text-white">上传目标管理</h1>

  <div class="config-card mb-8">
    <h2 class="text-xl font-semibold mb-4 dark:text-slate-200">
      添加新的 R2 配置
    </h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <input
        bind:value={newTarget.bucketName}
        required
        placeholder="Bucket Name"
        class="form-input"
      />
      <input
        bind:value={newTarget.accountId}
        required
        placeholder="Account ID"
        class="form-input"
      />
      <input
        bind:value={newTarget.accessKey}
        required
        placeholder="Access Key"
        class="form-input"
      />
      <input
        type="password"
        bind:value={newTarget.secretKey}
        required
        placeholder="Secret Key"
        class="form-input"
      />
    </div>
    <button type="submit" class="form-button mt-4" on:click={addTarget}
      >添加</button
    >
  </div>

  <div class="config-card">
    <h2 class="text-xl font-semibold mb-4 dark:text-slate-200">现有配置</h2>
    <div class="space-y-4">
      {#each targets as target}
        <div class="config-card">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
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
            on:click={() => deleteTarget(target.id!)}
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
    @apply w-full px-3 py-2 border rounded-lg bg-white dark:bg-slate-800 
           border-slate-300 dark:border-slate-600 focus:ring-2 focus:ring-blue-500 
           focus:border-blue-500 dark:focus:ring-blue-600 dark:focus:border-blue-600 
           transition-colors duration-200 text-slate-900 dark:text-slate-100;
  }

  .form-button {
    @apply px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 
           focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 
           dark:focus:ring-offset-slate-800 transition-colors duration-200;
  }

  .delete-button {
    @apply px-3 py-1 bg-red-600 text-white rounded-md hover:bg-red-700 
           focus:ring-2 focus:ring-red-500 focus:ring-offset-2 
           dark:focus:ring-offset-slate-800 transition-colors duration-200;
  }

  .config-card {
    @apply p-4 bg-white dark:bg-slate-800 rounded-lg shadow-sm 
           border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-slate-100;
  }
</style>
