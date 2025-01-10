import type { FullProject } from "./project";

export type SortMode = "none" | "name" | "published" | "updated" | "downloads";
export type SortDirection = "asc" | "desc";

export interface SearchResults {
    hits: number;
    page: number;
    pages: number;
    total: number;
    results: FullProject[];
}

type GameVersionFacet = ["game_versions", string[]];
type LoadersFacet = ["loaders", string[]];
type TagsFacet = ["tags", string[]];
type PublishedFacet = ["published", [Date, Date]];
type UpdatedFacet = ["updated", [Date, Date]];
type DownloadsFacet = ["downloads", [number, number]];

export type Facet =
    | GameVersionFacet
    | LoadersFacet
    | TagsFacet
    | PublishedFacet
    | UpdatedFacet
    | DownloadsFacet;

export type FacetType = Facet[0];

export type FacetSerializer<T> = (facet: T) => [FacetType, (string | number)[]];

const baseSerializer: FacetSerializer<
    GameVersionFacet | LoadersFacet | TagsFacet | DownloadsFacet
> = (facet: GameVersionFacet | LoadersFacet | TagsFacet | DownloadsFacet) => facet;

const dateSerializer: FacetSerializer<PublishedFacet | UpdatedFacet> = (
    facet: PublishedFacet | UpdatedFacet,
) => [facet[0], [facet[1][0].toUTCString(), facet[1][1].toUTCString()]];

export const serializeFacet: FacetSerializer<Facet> = (facet: Facet) =>
    facet[0] == "published" || facet[0] == "updated"
        ? dateSerializer(facet)
        : baseSerializer(facet);

export const serializeFacets = (facets: Facet[]) =>
    `[${facets
        .map(serializeFacet)
        .map((facet) => `["${facet[0]}", [${facet[1].map((v) => `"${v}"`).join(", ")}]]`)
        .join(",")}]`;
