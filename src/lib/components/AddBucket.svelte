<script lang="ts">
  import db from "$lib/db";
  import { modalState } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";

  let {
    bucket = $bindable({
      type: "r2",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
      customDomain: "",
      s3Api: "",
    }),
    onclose,
  }: {
    bucket?: Bucket;
    onclose?: () => void;
  } = $props();

  // 解析 S3 API URL
  function parseS3ApiUrl(url: string) {
    if (!url) return;
    try {
      const urlObj = new URL(url);
      const regex =
        /^https:\/\/([a-zA-Z0-9]+)\.r2\.cloudflarestorage\.com\/([a-zA-Z0-9-]+)\/?$/;
      if (!regex.test(url)) {
        alert(
          "S3 API URL 格式不正确，请使用格式：https://{accountId}.r2.cloudflarestorage.com/{bucketName}",
        );
        return;
      }
      const accountId = urlObj.hostname.split(".")[0];
      const bucketName = urlObj.pathname.split("/")[1];
      bucket.accountId = accountId;
      bucket.bucketName = bucketName;
    } catch (e) {
      alert(
        "S3 API URL 格式不正确，请使用格式：https://{accountId}.r2.cloudflarestorage.com/{bucketName}",
      );
      console.error("Invalid S3 API URL");
    }
  }

  // 定义输入框的配置
  const inputConfigs = $state([
    {
      id: "s3Api",
      label: "S3 API",
      focused: false,
    },
    {
      id: "bucketName",
      label: "Bucket Name",
      focused: false,
    },
    {
      id: "accountId",
      label: "Account ID",
      focused: false,
    },
    {
      id: "accessKey",
      label: "Access Key",
      focused: false,
    },
    {
      id: "secretKey",
      label: "Secret Key",
      focused: false,
    },
    {
      id: "customDomain",
      label: "Custom Domain",
      focused: false,
    },
  ]);

  // 上传目标管理功能
  async function saveBucket() {
    await db.buckets.put({
      ...bucket,
    });
    bucket = {
      type: "r2",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
      customDomain: "",
    };
    closeModal();
  }

  function closeModal() {
    if (onclose) {
      onclose();
    }
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
          bind:value={bucket[config.id]}
          type="text"
          id={config.id}
          class="input-field"
          onfocus={() => (config.focused = true)}
          onblur={() => (config.focused = false)}
          onchange={(e: Event) => {
            if (config.id === "s3Api") {
              const target = e.target as HTMLInputElement;
              parseS3ApiUrl(target.value);
            }
          }}
        />
        <label
          for={config.id}
          class="input-label"
          class:input-label-active={config.focused || bucket[config.id]}
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
