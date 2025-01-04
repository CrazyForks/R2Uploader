// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
import { loadTranslations } from "$lib/i18n/i18n";
import { appSettings } from "$lib/store.svelte.js";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export const load = async ({ url }) => {
  const { pathname } = url;

  const initLocale = appSettings.locale; // get from cookie, user session, ...

  await loadTranslations(initLocale, pathname); // keep this just before the `return`

  return {};
};
