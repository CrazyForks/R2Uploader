import type { Snippet } from "svelte";

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

// 代理相关状态
export const proxySettings = $state({
  proxyType: "system", // system | custom | none
  customProxies: [] as Array<{
    id: string;
    host: string;
    port: number;
    username?: string;
    password?: string;
  }>,
  selectedCustomProxyId: null as string | null,
});

export function addCustomProxy(proxy: {
  host: string;
  port: number;
  username?: string;
  password?: string;
}) {
  proxySettings.customProxies = [
    ...proxySettings.customProxies,
    {
      id: crypto.randomUUID(),
      ...proxy,
    },
  ];
}

export function removeCustomProxy(id: string) {
  proxySettings.customProxies = proxySettings.customProxies.filter(
    (p) => p.id !== id,
  );
  if (proxySettings.selectedCustomProxyId === id) {
    proxySettings.selectedCustomProxyId = null;
  }
}

export function selectCustomProxy(id: string) {
  proxySettings.selectedCustomProxyId = id;
}

export function setProxyType(type: "system" | "custom" | "none") {
  proxySettings.proxyType = type;
}
