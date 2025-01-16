import { unwrapOrNull, type Facet, type FullProject, type SearchResults } from "@modhost/api";
import { get, writable } from "svelte/store";
import type { LoadingState, Vec2 } from "./types";
import { client } from "./api";
import { userPreferencesStore } from "./user";

export const emptySearchResults: SearchResults = {
    page: 1,
    pages: 0,
    hits: 0,
    total: 0,
    results: [],
};

export const editSaving = writable<boolean>(false);
export const editLoadingState = writable<LoadingState>("loading");
export const currentProject = writable<FullProject | null>(null);
export const currentScrollPosition = writable<Vec2>({ x: 0, y: 0 });
export const popupsDidMount = writable<boolean>(false);

export const currentPage = writable<number>(1);
export const currentQuery = writable<string>("");
export const currentFilters = writable<Facet[]>([]);
export const searchResults = writable<SearchResults>(emptySearchResults);

export const updateSearchResults = async (force = false) => {
    if (get(searchResults).hits == 0 || force)
        searchResults.set(unwrapOrNull(await performSearch()) ?? emptySearchResults);

    return get(searchResults).hits != 0;
};

export const performSearch = () => {
    const prefs = get(userPreferencesStore);
    const query = get(currentQuery);

    return client.search(
        query == "" ? undefined : query,
        get(currentPage),
        prefs.perPage,
        prefs.sortBy,
        prefs.sortDir,
        get(currentFilters),
    );
};
