<script lang="ts">
    import { _ } from "svelte-i18n";
    import { afterNavigate, goto, replaceState } from "$app/navigation";
    import { base } from "$app/paths";
    import {
        currentSearchStore,
        packagesStore,
        userPreferencesStore,
        updatePackagesStore,
        emptySearchResults,
        tagsStore,
    } from "$lib/stores";
    import { vsprintf } from "sprintf-js";
    import type { Facet, LoadingState, Sort, SortMode } from "$lib/types";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { dedupe, guessSortMode } from "$lib/utils";
    import { contextMenu, type ContextMenuItem } from "$lib/contextMenu";
    import PackageList from "$components/ui/PackageList.svelte";
    import TablerIconCheck from "$components/icons/TablerIconCheck.svelte";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";
    import { searchPackages } from "$api";
    import { gameVersions } from "$lib/versions";
    import { modLoaders } from "$lib/loaders";

    let currentPage = $state(1);
    let perPage = $state(30);
    let loadingState: LoadingState = $state($packagesStore.hits == 0 ? "loading" : "ready");
    let loaderFilters = $state<string[]>([]);
    let versionFilters = $state<string[]>([]);
    let tagFilters = $state<string[]>([]);
    let versionSearch = $state("");
    let tagsSearch = $state("");

    const searchedVersions = $derived(
        ($gameVersions || []).filter((v) =>
            v.id.toLowerCase().includes(versionSearch.toLowerCase()),
        ),
    );

    const searchedTags = $derived(
        $tagsStore.filter(
            (v) =>
                v.id.toLowerCase().includes(tagsSearch.toLowerCase()) ||
                v.name.toLowerCase().includes(tagsSearch.toLowerCase()),
        ),
    );

    const showDetails = $derived(($page.url.searchParams.get("showDetails") ?? "false") == "true");

    onMount(async () => {
        const dir = $page.url.searchParams.get("dir");
        const curPage = $page.url.searchParams.get("page");
        const queryLoaderFilters = $page.url.searchParams.get("loaders");
        const queryVersionFilters = $page.url.searchParams.get("versions");
        const queryTagFilters = $page.url.searchParams.get("tags");

        loadingState = (await updatePackagesStore()) ? "ready" : "failed";
        $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");

        $userPreferencesStore.sortDir = dir
            ? dir == "asc"
                ? "asc"
                : dir == "desc"
                  ? "desc"
                  : $userPreferencesStore.sortDir
            : $userPreferencesStore.sortDir;

        if (
            $currentSearchStore == "" &&
            $page.url.searchParams.has("q") &&
            $page.url.searchParams.get("q") != ""
        ) {
            $currentSearchStore = $page.url.searchParams.get("q")!;
        }

        try {
            const val = curPage ? parseInt(curPage) : currentPage;

            currentPage = val;
        } catch (_) {}

        try {
            const val = queryLoaderFilters ? JSON.parse(queryLoaderFilters) : loaderFilters;

            loaderFilters = val;
        } catch (_) {}

        try {
            const val = queryVersionFilters ? JSON.parse(queryVersionFilters) : versionFilters;

            versionFilters = val;
        } catch (_) {}

        try {
            const val = queryTagFilters ? JSON.parse(queryTagFilters) : tagFilters;

            tagFilters = val;
        } catch (_) {}
    });

    $effect(() => {
        const facets: Facet<any>[] = [];

        if (loaderFilters.length > 0) {
            facets.push(["loaders", loaderFilters] as Facet<"loaders">);
        }

        if (versionFilters.length > 0) {
            facets.push(["game_versions", versionFilters] as Facet<"game_versions">);
        }

        if (tagFilters.length > 0) {
            facets.push(["tags", tagFilters] as Facet<"tags">);
        }

        searchPackages(
            $currentSearchStore,
            facets,
            $userPreferencesStore.sortBy,
            $userPreferencesStore.sortDir,
            currentPage,
            perPage,
        ).then((v) => ($packagesStore = v ?? emptySearchResults));
    });

    const updateQuery = async () => {
        if ($currentSearchStore != "") $page.url.searchParams.set("q", $currentSearchStore);
        else $page.url.searchParams.delete("q");

        if ($userPreferencesStore.sortBy != "none")
            $page.url.searchParams.set("sort", $userPreferencesStore.sortBy);
        else $page.url.searchParams.delete("sort");

        $page.url.searchParams.set("dir", $userPreferencesStore.sortDir);
        $page.url.searchParams.set("page", currentPage.toString());

        if (loaderFilters.length > 0)
            $page.url.searchParams.set("loaders", JSON.stringify(loaderFilters));
        else $page.url.searchParams.delete("loaders");

        if (versionFilters.length > 0)
            $page.url.searchParams.set("versions", JSON.stringify(versionFilters));
        else $page.url.searchParams.delete("versions");

        if (tagFilters.length > 0) $page.url.searchParams.set("tags", JSON.stringify(tagFilters));
        else $page.url.searchParams.delete("tags");

        replaceState($page.url, $page.state);
    };

    const prevPage = () => {
        currentPage = Math.max(1, Math.min(currentPage - 1, $packagesStore.pages));
        updateQuery();
    };

    const nextPage = () => {
        currentPage = Math.max(1, Math.min(currentPage + 1, $packagesStore.pages));
        updateQuery();
    };

    afterNavigate((nav) => {
        if (nav.to?.route.id == "/s") {
            updateQuery();
        }
    });

    const toggleLoaderFilter = (id: string) => {
        return () => {
            if (loaderFilters.includes(id)) loaderFilters = loaderFilters.filter((v) => v != id);
            else loaderFilters.push(id);

            updateQuery();
        };
    };

    const toggleVersionFilter = (id: string) => {
        return () => {
            if (versionFilters.includes(id)) versionFilters = versionFilters.filter((v) => v != id);
            else versionFilters.push(id);

            updateQuery();
        };
    };

    const toggleTagFilter = (id: string) => {
        return () => {
            if (tagFilters.includes(id)) tagFilters = tagFilters.filter((v) => v != id);
            else tagFilters.push(id);

            updateQuery();
        };
    };
</script>

<svelte:head>
    <title
        >{$currentSearchStore || $_(`search.title.${siteConfig.type}`)} - {siteConfig.siteName}</title
    >
</svelte:head>

<div class="flex h-full w-full flex-col items-start md:flex-row md:justify-between">
    <div
        class="card z-10 flex h-full w-full flex-col items-start justify-start space-y-4 p-3 py-4 md:sticky md:top-0 md:mr-4 md:w-80"
    >
        <p class="mx-2 my-0 text-lg">Search Filters</p>

        <hr class="w-full" />

        {#if $currentSearchStore || loaderFilters.length > 0 || versionFilters.length > 0}
            <button
                class="variant-soft-secondary btn w-fit hover:variant-filled-primary"
                onclick={() => {
                    $currentSearchStore = "";
                    loaderFilters = [];
                    versionFilters = [];
                    updateQuery();
                }}
            >
                <Icon icon="tabler:clear-all" height="24" class="mr-2" />
                {$_("search.clear_filters")}
            </button>

            <hr class="w-full" />
        {/if}

        <p class="mx-2">Filter Mod Loaders</p>

        <div class="flex w-full flex-col items-start justify-start space-y-2">
            {#each $modLoaders || [] as loader}
                <button
                    type="button"
                    class="variant-glass-primary btn w-full justify-start rounded-xl"
                    class:!variant-filled-primary={loaderFilters.includes(loader.id)}
                    onclick={toggleLoaderFilter(loader.id)}>{loader.name}</button
                >
            {/each}
        </div>

        <hr class="w-full" />

        <p class="mx-2">Filter Game Versions</p>

        <input
            type="text"
            bind:value={versionSearch}
            class="input rounded-md"
            placeholder="Search game versions..."
        />

        <div
            class="flex max-h-36 w-full flex-col items-start justify-start space-y-2 overflow-scroll md:max-h-60"
        >
            {#each searchedVersions as version}
                <button
                    type="button"
                    class="variant-glass-primary btn w-full justify-start rounded-xl"
                    class:!variant-filled-primary={versionFilters.includes(version.id)}
                    onclick={toggleVersionFilter(version.id)}>{version.id}</button
                >
            {/each}
        </div>

        {#if $tagsStore.length > 0}
            <hr class="w-full" />

            <p class="mx-2">Filter Tags</p>

            <input
                type="text"
                bind:value={tagsSearch}
                class="input rounded-md"
                placeholder="Search tags..."
            />

            <div
                class="flex max-h-36 w-full flex-col items-start justify-start space-y-2 overflow-scroll md:max-h-60"
            >
                {#each searchedTags as tag}
                    <button
                        type="button"
                        class="variant-glass-primary btn w-full justify-start rounded-xl"
                        class:!variant-filled-primary={tagFilters.includes(tag.id)}
                        onclick={toggleTagFilter(tag.id)}
                    >
                        <Icon icon={tag.icon} class="mr-2" width="20" />
                        {tag.name}
                    </button>
                {/each}
            </div>
        {/if}
    </div>

    <div class="flex h-full w-full flex-col items-center justify-start">
        <div
            class="sticky top-0 z-10 mb-2 flex w-full flex-col border-surface-600 bg-surface-900 p-2 backdrop-blur md:flex-row md:items-center md:justify-between"
        >
            <h1 class="mb-2 text-lg md:mb-0">
                {#if !$currentSearchStore}
                    {@html vsprintf($_("search.found_plural"), [$packagesStore.total])}
                    {$_(`search.plural.${siteConfig.type}`)}
                {:else}
                    {@html vsprintf(
                        $packagesStore.total == 1
                            ? $_("search.found_singular")
                            : $_("search.found_plural"),
                        [$packagesStore.total],
                    )}

                    <a href="{base}/s" class="anchor no-underline">
                        {$packagesStore.total == 1
                            ? $_(`search.singular.${siteConfig.type}`)
                            : $_(`search.plural.${siteConfig.type}`)}
                    </a>

                    {#if $currentSearchStore != ""}
                        {$_("search.matching")}
                        <button
                            class="transition-all hover:variant-filled-error hover:rounded hover:p-1 hover:px-2 hover:line-through"
                            onclick={() => ($currentSearchStore = "")}
                        >
                            {$currentSearchStore}
                        </button>
                    {/if}
                {/if}
            </h1>

            <div
                class="flex flex-col items-center justify-between space-y-2 md:flex-row md:justify-end md:space-y-0"
            >
                <div class="flex flex-row flex-wrap items-center space-x-1 md:mr-8 md:space-x-2">
                    <button
                        class="variant-glass-primary btn btn-sm text-center font-bold transition-all hover:variant-ghost-primary"
                        disabled={currentPage <= 1}
                        onclick={prevPage}><Icon height="24" icon="tabler:arrow-left" /></button
                    >

                    {#if $packagesStore.pages > 3}
                        {#if currentPage < $packagesStore.pages - 1}
                            <button
                                class="variant-filled-primary btn btn-icon-sm text-center font-bold transition-all"
                                >{currentPage}</button
                            >
                        {/if}

                        <button
                            class="variant-glass-primary btn btn-icon-sm text-center font-bold transition-all hover:variant-ghost-primary"
                            disabled>...</button
                        >

                        {#if currentPage >= $packagesStore.pages - 1}
                            <button
                                class="variant-glass-primary btn btn-icon-sm text-center font-bold transition-all hover:variant-ghost-primary"
                                class:!variant-filled-primary={currentPage ==
                                    $packagesStore.pages - 1}
                                onclick={() => {
                                    currentPage = $packagesStore.pages - 1;
                                    updateQuery();
                                }}>{$packagesStore.pages - 1}</button
                            >
                        {/if}

                        <button
                            class="variant-glass-primary btn btn-icon-sm text-center font-bold transition-all hover:variant-ghost-primary"
                            class:!variant-filled-primary={currentPage == $packagesStore.pages}
                            onclick={() => {
                                currentPage = $packagesStore.pages;
                                updateQuery();
                            }}>{$packagesStore.pages}</button
                        >
                    {:else}
                        {#each new Array($packagesStore.pages) as _, page}
                            <button
                                class="variant-glass-primary btn btn-icon-sm text-center font-bold transition-all hover:variant-ghost-primary"
                                class:!variant-filled-primary={currentPage == page + 1}
                                onclick={() => {
                                    currentPage = page + 1;
                                    updateQuery();
                                }}>{page + 1}</button
                            >
                        {/each}
                    {/if}

                    <button
                        class="variant-glass-primary btn btn-sm text-center font-bold transition-all hover:variant-ghost-primary"
                        disabled={currentPage >= $packagesStore.pages}
                        onclick={nextPage}
                    >
                        <Icon height="24" icon="tabler:arrow-right" /></button
                    >
                </div>

                <div
                    class="flex flex-row flex-wrap items-center justify-end space-x-1 md:space-x-2"
                >
                    <button
                        class="variant-soft-secondary btn rounded-full border border-transparent p-2 text-sm transition-all hover:border-secondary-500 md:text-base"
                        use:contextMenu={{
                            initiator: "left",
                            items: [
                                ...dedupe([5, 10, 15, 20, 25, 30, 35, perPage])
                                    .sort((a, b) => a - b)
                                    .map(
                                        (count) =>
                                            ({
                                                type: "ITEM",
                                                label: count.toString(),
                                                icon:
                                                    perPage == count ? TablerIconCheck : IconBlank,
                                                action: () => {
                                                    perPage = count;
                                                    updateQuery();
                                                },
                                            }) as ContextMenuItem,
                                    ),
                            ],
                        }}
                    >
                        <Icon icon="tabler:list" height="20" class="mr-2" />
                        {perPage}
                    </button>

                    <button
                        class="variant-soft-secondary btn btn-sm border border-transparent transition-all hover:border-secondary-500"
                        onclick={() =>
                            ($userPreferencesStore.compact = !$userPreferencesStore.compact)}
                    >
                        {#if $userPreferencesStore.compact}
                            <Icon icon="tabler:list-details" height="24" />
                        {:else}
                            <Icon icon="tabler:layout-dashboard" height="24" />
                        {/if}
                    </button>

                    <button
                        class="variant-soft-secondary btn w-[9rem] rounded-full border border-transparent p-2 text-sm transition-all hover:border-secondary-500 md:w-[9.5rem] md:text-base"
                        use:contextMenu={{
                            initiator: "left",
                            items: [
                                ...["name", "downloads", "published", "updated"].map(
                                    (name) =>
                                        ({
                                            type: "ITEM",
                                            label: $_(`search.sort_type.${name}`),
                                            icon:
                                                $userPreferencesStore.sortBy == name
                                                    ? TablerIconCheck
                                                    : IconBlank,
                                            action: () => {
                                                $userPreferencesStore.sortBy = name as Sort;
                                                updateQuery();
                                            },
                                        }) as ContextMenuItem,
                                ),
                                { type: "SEPARATOR" },
                                ...["asc", "desc"].map(
                                    (name) =>
                                        ({
                                            type: "ITEM",
                                            label: $_(`search.sort_mode.${name}`),
                                            icon:
                                                $userPreferencesStore.sortDir == name
                                                    ? TablerIconCheck
                                                    : IconBlank,
                                            action: () => {
                                                $userPreferencesStore.sortDir = name as SortMode;
                                                updateQuery();
                                            },
                                        }) as ContextMenuItem,
                                ),
                                { type: "SEPARATOR" },
                                {
                                    type: "ITEM",
                                    label: $_(`search.show_details`),
                                    icon: showDetails ? TablerIconCheck : IconBlank,
                                    action: () => {
                                        if (showDetails)
                                            $page.url.searchParams.delete("showDetails");
                                        else $page.url.searchParams.set("showDetails", "true");
                                        goto(`?${$page.url.searchParams.toString()}`);
                                    },
                                } as ContextMenuItem,
                            ],
                        }}
                    >
                        {#if $userPreferencesStore.sortDir == "asc"}
                            <Icon icon="tabler:sort-ascending" height="20" class="mr-2" />
                        {:else}
                            <Icon icon="tabler:sort-descending" height="20" class="mr-2" />
                        {/if}
                        {$userPreferencesStore.sortBy != "none"
                            ? $_(`search.sort_type.${$userPreferencesStore.sortBy}`)
                            : $_("search.unsorted")}
                    </button>
                </div>
            </div>
        </div>

        {#if loadingState == "loading"}
            <dl
                class="grid w-full grid-cols-1 gap-2"
                class:lg:grid-cols-2={!$userPreferencesStore.compact}
                class:md:grid-cols-2={$userPreferencesStore.compact}
                class:lg:grid-cols-3={$userPreferencesStore.compact}
            >
                {#each Array(6) as _}
                    <div
                        class="placeholder h-24 w-full animate-pulse rounded-container-token"
                    ></div>
                {/each}
            </dl>
        {:else if loadingState == "ready" && $packagesStore.total == 0}
            <p class="w-full text-center opacity-50">
                {$_(`errors.none_published.${siteConfig.type}`)}
            </p>
        {:else if loadingState == "ready"}
            <dl
                class="grid w-full grid-cols-1 gap-2"
                class:grid-cols-2={$userPreferencesStore.compact}
            >
                <PackageList
                    {showDetails}
                    compact={$userPreferencesStore.compact}
                    packages={$packagesStore}
                />
            </dl>
        {:else if loadingState == "failed"}
            <p class="w-full p-2 text-center">No projects found!</p>
        {/if}
    </div>
</div>
