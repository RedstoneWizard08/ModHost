import type { SortMode } from "@modhost/api";

export const guessSortMode = (input: string): SortMode => {
    if (["none", "name", "downloads", "published", "updated"].includes(input)) {
        return input as SortMode;
    } else {
        return "none";
    }
};
