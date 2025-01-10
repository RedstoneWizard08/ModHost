import "$lib/i18n";
import { browser } from "$app/environment";
import { locale, waitLocale } from "svelte-i18n";
import type { LayoutLoad } from "./$types";
import { userPreferencesStore } from "$lib/user";
import { get } from "svelte/store";

export const load: LayoutLoad = async () => {
    if (browser) {
        if (get(userPreferencesStore).locale) {
            locale.set(get(userPreferencesStore).locale);
        } else {
            locale.set(window.navigator.language);
        }
    }

    await waitLocale();
};
