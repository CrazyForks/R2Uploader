import type { Snippet } from "svelte";
import db from "./db";
import { copyFieldsSimple } from "./tools";

export let alertMessage = $state({
  message: "",
});

export function setAlert(message: string) {
  alertMessage.message = message;
}

interface ModalState {
  isShow: boolean;
  children: Snippet | undefined;
  onClose?: () => void;
}

export let modalState: ModalState = $state({
  isShow: false,
  children: undefined,
  onClose: undefined,
});

export function closeModal() {
  modalState.isShow = false;
}

export function showModal(children: Snippet) {
  modalState.children = children;
  modalState.isShow = true;
}

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
interface AppSettings {
  sidebarCollapsed: boolean;
  useSystemProxy: boolean;
  locale: string;
  defaultBucketId: number | undefined;
}

export let appSettings = $state<AppSettings>({
  sidebarCollapsed: false,
  useSystemProxy: true,
  locale: "en",
  defaultBucketId: undefined,
});

// initialize app settings from database
export async function initAppSettings() {
  const settings = await db.appSettings.get(1);
  if (settings) {
    copyFieldsSimple(settings, appSettings);
  }
}
