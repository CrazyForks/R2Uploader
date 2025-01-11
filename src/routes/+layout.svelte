<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Alert from "$lib/components/Alert.svelte";
  import FileDrag from "$lib/components/FileDrag.svelte";
  let { children } = $props();
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import {
    globalState,
    initAppSettings,
    setAlert,
    setDragPaths,
    setIsDragging,
  } from "$lib/store.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import db from "$lib/db";
  import { parsePaths } from "$lib/tools";
  import type { UploadProgress } from "$lib/type";

  let unlistenDrag: UnlistenFn;
  let unlistenProgress: UnlistenFn;

  onMount(async () => {
    // initialize settings on load
    initAppSettings();

    // 监听拖拽事件
    unlistenDrag = await listen("tauri://drag-enter", async (event) => {
      setIsDragging(true);
    });

    // 监听上传进度事件
    unlistenProgress = await listen<UploadProgress>(
      "upload-progress",
      (event) => {
        console.log("event: ", event);
        //   const progress = event.payload;
        //   uploadStatusMap[progress.taskId] = progress;

        //   // 检查是否所有文件都已完成上传
        //   const allCompleted = Object.values(uploadStatusMap).every(
        //     (p) =>
        //       p.status.type === "success" ||
        //       p.status.type === "error" ||
        //       p.status.type === "cancelled",
        //   );

        //   if (allCompleted) {
        //     const hasError = Object.values(uploadStatusMap).some(
        //       (p) => p.status.type === "error",
        //     );
        //     uploadStatus = hasError ? "error" : "success";
        //     setAlert(hasError ? "部分文件上传失败" : "上传成功");
        //   }
      },
    );
  });

  onDestroy(() => {
    if (unlistenDrag) {
      unlistenDrag();
    }
    if (unlistenProgress) {
      unlistenProgress();
    }
  });

  $effect(() => {
    // 如果 appSettings 有变化，更新到数据库
    db.appSettings.put({
      id: 1,
      ...$state.snapshot(globalState.appSetting),
    });
    // 监听到拖拽事件后，解析路径，然后清空 dragState.paths
    if (globalState.drag.paths.length > 0) {
      parsePaths(globalState.drag.paths);
      setDragPaths([]);
    }
  });
</script>

<FileDrag />
<Alert />
<Modal />

<div class="flex h-full bg-slate-50 dark:bg-slate-900">
  <Sidebar />
  <main class="flex-1 overflow-hidden">
    {@render children()}
  </main>
</div>
