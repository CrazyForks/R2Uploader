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
async function initAppSettings() {
  const settings = await db.appSettings.get(1);
  if (settings) {
    appSettings.sidebarCollapsed = settings.sidebarCollapsed;
    appSettings.useSystemProxy = settings.useSystemProxy;
  }
}

// save app settings to database
export async function saveAppSettings() {
  await db.appSettings.put({
    id: 1,
    ...appSettings,
  });
}

// initialize settings on load
initAppSettings();
