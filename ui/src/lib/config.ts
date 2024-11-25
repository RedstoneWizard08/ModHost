/// <reference path="../app.d.ts" />

import type { GameVersion } from "./versions";
import {
    PUBLIC_APP,
    PUBLIC_DEFAULT_THEME,
    PUBLIC_GAME_BETA_NAME,
    PUBLIC_PKG_FILE_FORMATS,
    PUBLIC_PKG_TYPE,
    PUBLIC_SHOW_BETA,
    PUBLIC_TAGLINE,
} from "$env/static/public";

export interface SiteConfig {
    siteName: string;
    tagline: string;
    showBeta: boolean;
    type: "mods" | "packages";
    defaultTheme: string;
    packageFileFormats: string[];
    betaName: "beta" | "snapshot";
    versionFetcher: () => Promise<GameVersion[]>;
}

export const siteConfig: SiteConfig = {
    siteName: PUBLIC_APP,
    tagline: PUBLIC_TAGLINE,
    showBeta: PUBLIC_SHOW_BETA == "true",
    type: PUBLIC_PKG_TYPE as "mods" | "packages",
    defaultTheme: PUBLIC_DEFAULT_THEME,
    packageFileFormats: PUBLIC_PKG_FILE_FORMATS.split(","),
    betaName: PUBLIC_GAME_BETA_NAME as "beta" | "snapshot",
    versionFetcher: async () => await (await fetch("/api/v1/meta/game_versions")).json(),
};
