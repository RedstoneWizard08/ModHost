import { browser } from "$app/environment";
import { unwrapOrNull, type User } from "@modhost/api";
import { get, writable } from "svelte/store";
import { siteConfig } from "./config";
import { locales } from "svelte-i18n";
import type { UserPreferences } from "./types";
import { persisted } from "svelte-persisted-store";
import { checkClientToken, client } from "./api";

export const user = writable<User | null>(null);

export const userPreferencesStore = persisted<UserPreferences>("preferences", {
    sortBy: "none",
    sortDir: "asc",
    perPage: 25,
    locale: browser && get(locales).includes(navigator.language) ? navigator.language : "en-US",
    theme: siteConfig.defaultTheme,
    lightMode: false,
    compact: false,
});

export const updateUser = async () => {
    checkClientToken();
    user.set(unwrapOrNull(await client.currentUser()));
};

export const updateTheme = () => {
    if (!browser) return;

    if (get(userPreferencesStore).lightMode) {
        document.documentElement.classList.remove("dark");
    } else {
        document.documentElement.classList.add("dark");
    }
};

userPreferencesStore.subscribe(updateTheme);
