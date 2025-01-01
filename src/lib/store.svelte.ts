// biome-ignore lint/style/useConst: <explanation>
export let alertMessage = $state({
  message: "",
});

export function setAlert(message: string) {
  alertMessage.message = message;
}
