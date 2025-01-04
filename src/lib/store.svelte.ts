import type { Snippet } from "svelte";
import db from "./db";

export let alertMessage = $state({
  message: "",
});

export function setAlert(message: string) {
  alertMessage.message = message;
}

interface ModalState {
  isShow: boolean;
  children: Snippet | undefined;
}

export let modalState: ModalState = $state({
  isShow: false,
  children: undefined,
});

export let dragState: { isDragging: boolean; paths: string[] } = $state({
  isDragging: false,
  paths: [],
});

export function setIsDragging(isDragging: boolean) {
  dragState.isDragging = isDragging;
}

export function setDragPaths(paths: string[]) {
  dragState.paths = paths;
}

// app settings
export let appSettings = $state({
  sidebarCollapsed: false,
  useSystemProxy: true,
});

// initialize app settings from database
export async function initAppSettings() {
  const settings = await db.appSettings.get(1);
  if (settings) {
    appSettings.sidebarCollapsed = settings.sidebarCollapsed;
    appSettings.useSystemProxy = settings.useSystemProxy;
  }
}
