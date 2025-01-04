<script lang="ts">
  let {
    textContent,
    remoteFileName,
  }: {
    textContent: string;
    remoteFileName: string;
  } = $props();
</script>

<div class="space-y-4">
  <div class="space-y-2">
    <div>
      <p
        class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
      >
        文本内容
      </p>
      <textarea
        bind:value={textContent}
        class="w-full rounded-lg bg-slate-50 p-3 focus:ring-2 focus:ring-cyan-500 focus:outline-none dark:bg-slate-700"
        placeholder="输入要上传的文本内容"
        rows="6"
      ></textarea>
    </div>
    <div>
      <p
        class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
      >
        远程文件名
      </p>
      <input
        bind:value={remoteFileName}
        class="w-full rounded-lg bg-slate-50 p-3 focus:ring-2 focus:ring-cyan-500 focus:outline-none dark:bg-slate-700"
        placeholder="输入远程文件名"
      />
    </div>
  </div>
</div>
