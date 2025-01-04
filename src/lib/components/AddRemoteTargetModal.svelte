<script lang="ts">
  import { modalState } from "$lib/store.svelte";
  import { Dialog } from "bits-ui";

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
    modalState.children = undefined;
  }

  function show() {
    modalState.children = content;
    modalState.isShow = true;
  }
</script>

{#snippet content()}
  <h3>add remote target</h3>
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
          class:invisible={!config.value}
          class="input-label"
          class:input-label-active={config.value}
        >
          {config.label}
        </label>
      </div>
    {/each}
  </div>
  <div class="mt-8 flex justify-end space-x-2">
    <button onclick={closeModal} class="button-secondary">取消</button>
    <button onclick={handleSubmit} class="button-primary">添加</button>
  </div>
{/snippet}

<button onclick={show}>添加</button>
