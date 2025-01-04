<script lang="ts">
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

  function handleSubmit() {
    const newTarget = inputConfigs.reduce(
      (acc, config) => {
        acc[config.id] = config.value;
        return acc;
      },
      {} as Record<string, string>,
    );

    // 你可以在这里处理提交逻辑，比如发送数据到服务器
    console.log(newTarget);
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
          placeholder={config.label}
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
    <button onclick={closeModal} class="button">取消</button>
    <button onclick={handleSubmit} class="button">添加</button>
  </div>
{/snippet}

<button onclick={show}>添加</button>

<style lang="postcss">
  .button {
    @apply cursor-pointer rounded-lg px-4 py-2 text-cyan-500 hover:bg-cyan-50;
  }

  .input-container {
    @apply relative mb-6;
  }

  .input-field {
    @apply w-full border-0 border-b border-gray-300 py-2 transition-colors outline-none;
  }

  .input-field:focus {
    @apply border-blue-500;
  }

  .input-label {
    @apply pointer-events-none absolute top-2 left-0 opacity-0 transition-all;
  }

  .input-label-active {
    @apply -translate-y-5 text-sm text-slate-500 opacity-100;
  }
</style>
