<script lang="ts">
  import db from "$lib/db";
  import { modalState } from "$lib/store.svelte";

  // 定义输入框的配置
  const inputConfigs = $state([
    {
      id: "bucketName",
      label: "Bucket Name",
      value: "",
    },
    {
      id: "accountId",
      label: "Account ID",
      value: "",
    },
    {
      id: "accessKey",
      label: "Access Key",
      value: "",
    },
    {
      id: "secretKey",
      label: "Secret Key",
      value: "",
    },
  ]);

  let newTarget = {
    name: "",
    description: "",
    bucketName: "",
    accountId: "",
    accessKey: "",
    secretKey: "",
  };

  // 上传目标管理功能
  async function addTarget() {
    const id = await db.uploadTargets.add({
      ...newTarget,
      type: "r2",
    });
    newTarget = {
      name: "",
      description: "",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
    };
  }

  function closeModal() {
    // 关闭模态框
    modalState.isShow = false;
  }

  function show() {
    modalState.children = content;
    modalState.isShow = true;
  }
</script>

{#snippet content()}
  <h3>Add bucket</h3>
  <div class="space-y-6">
    <!-- 动态生成输入框 -->
    {#each inputConfigs as config}
      <div class="input-container">
        <input
          bind:value={config.value}
          type="text"
          id={config.id}
          class="input-field"
        />
        <label
          for={config.id}
          class="input-label"
          class:input-label-active={config.value}
        >
          {config.label}
        </label>
      </div>
    {/each}
  </div>
  <div class="mt-8 flex justify-end space-x-2">
    <button onclick={closeModal} class="button button-primary">Cancel</button>
    <button onclick={addTarget} class="button button-primary">Save</button>
  </div>
{/snippet}

<button class="button button-primary" onclick={show}>Add New</button>

<style lang="postcss">
  .input-container {
    @apply relative mb-6;
  }

  .input-field {
    @apply w-full border-0 border-b border-gray-300 py-2 transition-colors outline-none;
  }

  .input-field:focus {
    @apply border-cyan-500;
  }

  .input-label {
    @apply pointer-events-none absolute top-2 left-0 opacity-30 transition-all;
  }

  .input-label-active {
    @apply -translate-y-5 text-sm text-slate-500 opacity-100;
  }
</style>
