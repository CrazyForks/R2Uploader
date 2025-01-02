// biome-ignore lint/style/useConst: <explanation>
export let alertMessage = $state({
  message: "",
});

export function setAlert(message: string) {
  alertMessage.message = message;
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
