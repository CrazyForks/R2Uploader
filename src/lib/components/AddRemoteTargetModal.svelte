<script lang="ts">
  import db from "$lib/db";
  import { modalState } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";

  let {
    bucket = {
      type: "r2",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
      customDomain: "",
    },
  }: {
    bucket?: Bucket;
  } = $props();

  // 定义输入框的配置
  const inputConfigs = $state([
    {
      id: "bucketName",
      label: "Bucket Name",
      value: bucket.bucketName,
      focused: false,
    },
    {
      id: "accountId",
      label: "Account ID",
      value: bucket.accountId,
      focused: false,
    },
    {
      id: "accessKey",
      label: "Access Key",
      value: bucket.accessKey,
      focused: false,
    },
    {
      id: "secretKey",
      label: "Secret Key",
      value: bucket.secretKey,
      focused: false,
    },
    {
      id: "customDomain",
      label: "Custom Domain",
      value: bucket.customDomain,
      focused: false,
    },
  ]);

  // 上传目标管理功能
  async function saveBucket() {
    await db.buckets.put({
      ...bucket,
    });
    closeModal();
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
          onfocus={() => (config.focused = true)}
          onblur={() => (config.focused = false)}
        />
        <label
          for={config.id}
          class="input-label"
          class:input-label-active={config.focused || config.value}
        >
          {config.label}
        </label>
      </div>
    {/each}
  </div>
  <div class="mt-8 flex justify-end space-x-2">
    <button onclick={closeModal} class="button button-primary">Cancel</button>
    <button onclick={saveBucket} class="button button-primary">Save</button>
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
