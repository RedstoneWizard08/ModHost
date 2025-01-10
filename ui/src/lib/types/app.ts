import type { SortDirection, SortMode } from "@modhost/api";

export interface UserPreferences {
    sortBy: SortMode;
    sortDir: SortDirection;
    perPage: number;
    locale: string;
    theme: string;
    lightMode: boolean;
    compact: boolean;
}

export type LoadingState = "loading" | "ready" | "failed";
