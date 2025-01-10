import { unwrap, type GameVersion, type ModLoader, type Tag } from "@modhost/api";
import { writable } from "svelte/store";
import { client } from "./api";

export const tags = writable<Tag[]>([]);
export const loaders = writable<ModLoader[]>([]);
export const gameVersions = writable<GameVersion[]>([]);
export const licenses = writable<string[]>([]);
export const licensesUrl = "https://licenses.opendefinition.org/licenses/groups/all.json";

export const updateTags = async () => tags.set(unwrap(await client.meta().tags()));
export const updateLoaders = async () => loaders.set(unwrap(await client.meta().loaders()));

export const updateGameVersions = async () =>
    gameVersions.set(unwrap(await client.meta().gameVersions()));

export const updateLicenses = async () =>
    licenses.set(Object.keys(await (await fetch(licensesUrl)).json()));

export const initMeta = async () => {
    await Promise.all([updateTags(), updateLoaders(), updateGameVersions(), updateLicenses()]);
};
