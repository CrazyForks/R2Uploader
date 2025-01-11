import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { sep } from "@tauri-apps/api/path";
import clipboard from "tauri-plugin-clipboard-api";
import { filesState } from "./files.svelte";
import { setAlert } from "./store.svelte";
import type { FileDetail } from "./type";

export function generateTimestamp() {
  const now = new Date();
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}_${String(now.getHours()).padStart(2, "0")}-${String(now.getMinutes()).padStart(2, "0")}-${String(now.getSeconds()).padStart(2, "0")}`;
}

export function generateTimestampTextFileName() {
  return `${generateTimestamp()}.txt`;
}

// 如果以 sep 开头，去掉 sep，如果 sep() 不是 /，替换为 /
function handleRelativePath(path: string) {
  const s = sep();
  return path.startsWith(s)
    ? path.slice(s.length).replaceAll(s, "/")
    : path.replaceAll(s, "/");
}

async function getFileDetails(path: string) {
  try {
    const details: Array<FileDetail> = await invoke("get_file_details", {
      path,
    });
    return details;
  } catch (e) {
    console.error(e);
    setAlert("获取文件详情失败");
  }
}

export async function parsePaths(paths: string[]) {
  paths.forEach(async (file) => {
    const details = await getFileDetails(file);
    if (details && details.length > 0) {
      details.forEach((detail) => {
        filesState.files.push({
          type: "file",
          id: detail.id,
          source: {
            filePath: detail.path,
          },
          remoteFilename: handleRelativePath(detail.relativePath),
          remoteFilenamePrefix: "",
        });
      });
    }
  });
}

export function addText(textContent: string, remoteFilename: string) {
  filesState.files.push({
    type: "text",
    id: Date.now().toString(),
    source: { fileContent: textContent },
    remoteFilename,
    remoteFilenamePrefix: "",
  });
}

export function addImage(imageContent: string) {
  let remoteFilename = generateTimestamp();
  if (imageContent.startsWith("iVBORw0KGgo")) {
    imageContent = `data:image/png;base64,${imageContent}`;
    remoteFilename += ".png";
  } else if (imageContent.startsWith("/9j/")) {
    imageContent = `data:image/jpeg;base64,${imageContent}`;
    remoteFilename += ".jpg";
  } else if (imageContent.startsWith("R0lGODlh")) {
    imageContent = `data:image/gif;base64,${imageContent}`;
    remoteFilename += ".gif";
  } else if (imageContent.startsWith("UklGR")) {
    imageContent = `data:image/webp;base64,${imageContent}`;
    remoteFilename += ".webp";
  } else {
    remoteFilename += ".png";
  }
  filesState.files.push({
    type: "image",
    id: Date.now().toString(),
    source: { fileContent: imageContent },
    remoteFilename,
    remoteFilenamePrefix: "",
  });
}

export async function checkClipboardContent() {
  try {
    if (await clipboard.hasText()) {
      const text = await clipboard.readText();
      addText(text, generateTimestampTextFileName());
    }
    if (await clipboard.hasImage()) {
      const image = await clipboard.readImageBase64();
      addImage(image);
    }
    if (await clipboard.hasFiles()) {
      const file = await clipboard.readFiles();
      await parsePaths(file);
    }
  } catch (error: unknown) {
    console.error("clipboard error:", error);
  }
}

export function copyFieldsSimple<T extends object>(source: T, target: T): T {
  Object.assign(target, source);
  return target;
}

// 拷贝没有的字段
export function copyNotExistsFields<T extends object>(source: T, target: T): T {
  for (const key in source) {
    if (!(key in target)) {
      target[key] = source[key];
    }
  }
  return target;
}

export async function openDialogForFiles() {
  const dialogFiles = await open({
    multiple: true,
    directory: false,
  });
  if (dialogFiles) {
    await parsePaths(dialogFiles);
  }
}

export async function openDialogForDir() {
  const dialogFiles = await open({
    multiple: true,
    directory: true,
  });
  if (dialogFiles) {
    await parsePaths(dialogFiles);
  }
}
