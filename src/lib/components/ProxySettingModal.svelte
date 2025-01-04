<script lang="ts">
  import { addCustomProxy, modalState } from "$lib/store.svelte";
  // 新代理表单状态
  let newProxy = {
    host: "",
    port: 0,
    username: "",
    password: "",
  };

  function closeModal() {
    // 关闭模态框
    modalState.isShow = false;
  }

  function show() {
    modalState.children = content;
    modalState.isShow = true;
  }

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
    closeModal();
  }
</script>

<button>添加代理</button>
{#snippet content()}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-2"
  >
    <div class="config-card w-full max-w-sm p-3">
      <h2 class="mb-2 text-lg font-semibold dark:text-slate-200">添加代理</h2>

      <div class="space-y-2">
        <input
          bind:value={newProxy.host}
          placeholder="主机地址"
          class="form-input px-2 py-1 text-sm"
        />
        <input
          type="number"
          bind:value={newProxy.port}
          placeholder="端口"
          class="form-input px-2 py-1 text-sm"
        />
        <input
          bind:value={newProxy.username}
          placeholder="用户名 (可选)"
          class="form-input px-2 py-1 text-sm"
        />
        <input
          type="password"
          bind:value={newProxy.password}
          placeholder="密码 (可选)"
          class="form-input px-2 py-1 text-sm"
        />
      </div>

      <div class="mt-2 flex justify-end gap-1">
        <button onclick={closeModal} class="delete-button px-2 py-0.5 text-sm">
          取消
        </button>
        <button onclick={addProxy} class="form-button px-2 py-0.5 text-sm">
          添加
        </button>
      </div>
    </div>
  </div>
{/snippet}
