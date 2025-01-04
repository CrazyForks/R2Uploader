<script lang="ts">
  import db from "$lib/db";
  import { modalState } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { ArrowLeft, HelpCircle } from "lucide-svelte";
  let showHelp = $state(false);

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

  const inputConfigs = $state([
    {
      id: "s3Api",
      label: "S3 API",
      focused: false,
      required: false,
    },
    {
      id: "bucketName",
      label: "Bucket Name",
      focused: false,
      required: true,
    },
    {
      id: "accountId",
      label: "Account ID",
      focused: false,
      required: true,
    },
    {
      id: "accessKey",
      label: "Access Key",
      focused: false,
      required: true,
    },
    {
      id: "secretKey",
      label: "Secret Key",
      focused: false,
      required: true,
    },
    {
      id: "customDomain",
      label: "Custom Domain",
      focused: false,
      required: false,
    },
  ]);

  async function saveBucket() {
    await db.buckets.put({
      ...bucket,
    });

    closeModal();
  }

  function closeModal() {
    if (onclose) {
      onclose();
    }
    modalState.isShow = false;
    bucket = {
      type: "r2",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
      customDomain: "",
    };
  }

  function show() {
    modalState.children = content;
    modalState.isShow = true;
  }
</script>

{#snippet content()}
  {#if showHelp}
    <div class="space-y-4">
      <div class="flex items-center gap-2">
        <button class="button" onclick={() => (showHelp = false)}>
          <ArrowLeft size={20} />
        </button>
        <p>如何使用</p>
      </div>
      <div class="space-y-2 text-sm text-gray-600">
        <p>
          1. 输入 S3 API
          URL，格式为：https://[accountId].r2.cloudflarestorage.com/[bucketName]
        </p>
        <p>2. 如果已有 Bucket Name 和 Account ID，可以直接填写</p>
        <p>3. 输入 Access Key 和 Secret Key 进行身份验证</p>
        <p>4. 可选：填写自定义域名（Custom Domain）</p>
        <p>5. 点击 Save 保存配置</p>
      </div>
    </div>
  {:else}
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <p>Add bucket</p>
        <button class="button" onclick={() => (showHelp = true)}>
          <HelpCircle size={20} />
        </button>
      </div>

      {#each inputConfigs as config}
        <div class="relative">
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
            {config.label}{config.required ? "*" : ""}
          </label>
        </div>
      {/each}
    </div>
    <div class="mt-8 flex justify-end space-x-2">
      <button onclick={closeModal} class="button button-primary">Cancel</button>
      <button onclick={saveBucket} class="button button-primary">Save</button>
    </div>
  {/if}
{/snippet}

<button class="button button-primary" onclick={show}>Add New</button>

<style lang="postcss">
  .input-field {
    @apply w-full border-0 border-b border-gray-300 py-1 transition-colors outline-none;
  }

  .input-field:focus {
    @apply border-cyan-500;
  }

  .input-label {
    @apply pointer-events-none absolute top-2 left-0 opacity-30 transition-all;
  }

  .input-label-active {
    @apply -translate-y-5 text-sm text-cyan-500 opacity-100;
  }
</style>
