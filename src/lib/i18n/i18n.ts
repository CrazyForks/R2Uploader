import i18n, { type Config } from "sveltekit-i18n";

const config: Config = {
  fallbackLocale: "en",
  loaders: [
    {
      locale: "en",
      key: "",
      loader: async () => (await import("./locales/en.json")).default,
    },
    {
      locale: "zh",
      key: "",
      loader: async () => (await import("./locales/zh.json")).default,
    },
  ],
};

export const { t, locale, locales, loading, loadTranslations } = new i18n(
  config,
);
