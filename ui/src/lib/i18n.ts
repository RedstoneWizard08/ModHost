/// <reference path="./vite-locales.d.ts" />

import { browser } from "$app/environment";
import { addMessages, init } from "svelte-i18n";
import { allLocales, localeData } from "@locales";

const defaultLocale = "en-US";

for (const locale of allLocales) {
    addMessages(locale, localeData[locale]);
}

init({
    fallbackLocale: defaultLocale,
    initialLocale: browser ? window.navigator.language : defaultLocale,
});
