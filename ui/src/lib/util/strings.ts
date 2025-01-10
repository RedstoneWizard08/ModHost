import { loaders } from "$lib/meta";
import { get } from "svelte/store";

export const fixLoaderName = (name: string) => {
    return get(loaders)?.find((v) => v.id.toLowerCase() == name.toLowerCase())?.name ?? "Unknown";
};

export const formatBytes = (bytes: number, decimals = 2) => {
    if (!+bytes) return "0 Bytes";

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
};

export const formatDate = (date: Date) => {
    return new Intl.DateTimeFormat(undefined, {
        year: "numeric",
        month: "numeric",
        day: "numeric",
        hour: "numeric",
        minute: "numeric",
        second: "numeric",
        hour12: true,
    }).format(date);
};

export const createSlug = (input: string) => {
    const slugRegex = /[^a-z0-9_-]/gm;

    return input.toLowerCase().replace(slugRegex, "-");
};

export const capText = (text: string, len: number) => {
    if (text.length < len) return text;

    return text.substring(0, len - 3) + "...";
};
