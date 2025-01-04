<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import { Upload, Folder, FileText, Clipboard } from "lucide-svelte";

  let {
    activeTab,
    onTabChange,
  }: {
    activeTab: "file" | "folder" | "text" | "clipboard";
    onTabChange: (tab: "file" | "folder" | "text" | "clipboard") => void;
  } = $props();
</script>

<div class="mb-4 flex gap-2">
  <button
    class:btn-tab-active={activeTab === "file"}
    onclick={() => onTabChange("file")}
    class="btn-tab"
  >
    <Upload class="size-5" />
    {t().tabSwitcher.file}
  </button>
  <button
    class:btn-tab-active={activeTab === "folder"}
    onclick={() => onTabChange("folder")}
    class="btn-tab"
  >
    <Folder class="size-5" /> 上传文件夹
  </button>
  <button
    class:btn-tab-active={activeTab === "text"}
    onclick={() => onTabChange("text")}
    class="btn-tab"
  >
    <FileText class="size-5" /> 上传文本
  </button>
  <button
    class:btn-tab-active={activeTab === "clipboard"}
    onclick={() => onTabChange("clipboard")}
    class="btn-tab"
  >
    <Clipboard class="size-5" /> 剪贴板
  </button>
</div>
