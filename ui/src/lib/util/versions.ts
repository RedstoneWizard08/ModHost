import type { ProjectVersion } from "@modhost/api";

export const getLoaders = (versions: ProjectVersion[]) => {
    const data: string[] = [];

    for (const version of versions) {
        for (const item of version.loaders) {
            if (!data.includes(item)) data.push(item);
        }
    }

    return data;
};

export const getGameVersions = (versions: ProjectVersion[]) => {
    const data: string[] = [];

    for (const version of versions) {
        for (const item of version.game_versions) {
            if (!data.includes(item)) data.push(item);
        }
    }

    return data;
};
