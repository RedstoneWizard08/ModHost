<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { LoadingState } from "$lib/types";
    import { onMount } from "svelte";
    import { user as userStore, userPreferencesStore } from "$lib/user";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import PackageList from "$components/ui/PackageList.svelte";
    import { guessSortMode } from "$lib/util";
    import { contextMenu, type ContextMenuItem } from "$lib/ui";
    import TablerIconCheck from "$components/icons/TablerIconCheck.svelte";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import { afterNavigate, beforeNavigate, goto } from "$app/navigation";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";
    import { client } from "$lib/api";
    import { unwrapOrNull, type FullProject, type SortMode, type User } from "@modhost/api";

    const id = $derived($page.params.id);
    const toasts = getToastStore();
    const showDetails = $derived(($page.url.searchParams.get("showDetails") ?? "false") == "true");

    let loadingState = $state<LoadingState>("loading");
    let user = $state<User | null>(null);
    let packages = $state<FullProject[]>([]);

    const downloads = $derived(packages.reduce((a, b) => a + b.downloads, 0));

    onMount(async () => {
        loadingState = "loading";
        $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");
        user = unwrapOrNull(await client.user(id).get());

        if (user) {
            packages = unwrapOrNull(await client.user(id).projects()) ?? [];
            loadingState = "ready";
        } else {
            loadingState = "failed";
        }
    });

    beforeNavigate(() => {
        loadingState = "loading";
        user = null;
        packages = [];
    });

    // This is incredibly scuffed but it works
    afterNavigate(async ({ to }) => {
        if (to?.route.id == "/u/[id]") {
            loadingState = "loading";
            $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");
            user = unwrapOrNull(await client.user(id).get());

            if (user) {
                packages = unwrapOrNull(await client.user(id).projects()) ?? [];
                loadingState = "ready";
            } else {
                loadingState = "failed";
            }
        }
    });
</script>

<svelte:head>
    <title>{user?.username ?? $_("site.loading")} - {siteConfig.siteName}</title>
</svelte:head>

{#if loadingState == "loading"}
    <div class="placeholder m-2 mx-auto w-32 animate-pulse"></div>
{:else if loadingState == "ready" && user}
    <div class="h2 mb-1 flex w-full flex-row items-center justify-between font-bold">
        <div class="flex flex-row items-center justify-start">
            {#if user?.github_id == -1}
                <img
                    src="/modhost.png"
                    alt="author's profile"
                    class="rounded-token mr-4 aspect-square h-16"
                />
            {:else}
                <img
                    src="https://avatars.githubusercontent.com/u/{user?.github_id}"
                    alt="author's profile"
                    class="rounded-token mr-4 aspect-square h-16"
                />
            {/if}

            <span class="h2 font-bold">{user?.username}</span>
        </div>

        <div class="flex flex-row items-center justify-end">
            {#if user.github_id == -1}
                <span class="variant-filled-success badge">System</span>
            {/if}

            {#if user.admin}
                <span class="variant-filled-error badge ml-2">{$_("user.admin")}</span>
            {/if}

            {#if user.moderator}
                <span class="variant-filled-secondary badge ml-2">{$_("user.moderator")}</span>
            {/if}

            {#if $userStore && $userStore.github_id == user.github_id}
                <span class="variant-filled-primary badge ml-2">{$_("user.you")}</span>
            {/if}
        </div>
    </div>

    <div
        class="card mb-1 flex w-full select-text flex-row items-center justify-between gap-1 overflow-x-auto p-4 not-italic"
    >
        <div class="flex h-full w-full flex-col gap-1 overflow-x-auto">
            <span class="text-sm">
                <span
                    >{downloads}
                    {downloads == 1
                        ? $_("list.download_singluar")
                        : $_("list.download_plural")}</span
                >
            </span>
        </div>
    </div>

    <div class="card p-4">
        <dt class="mb-2 text-sm opacity-50">{$_(`user.${siteConfig.type}`)}</dt>

        <div class="flex flex-row items-center justify-between">
            <button
                class="variant-soft-secondary btn hover:variant-filled-primary mb-4 w-fit"
                onclick={() => ($userPreferencesStore.compact = !$userPreferencesStore.compact)}
            >
                <Icon icon="tabler:layout-dashboard" height="24" class="mr-2" />

                <span class="md:inline">
                    {$userPreferencesStore.compact
                        ? $_("search.use_view.list")
                        : $_("search.use_view.compact")}
                </span>
            </button>

            <div class="flex flex-wrap space-x-2">
                <button
                    class="anchor"
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
                                        action: () =>
                                            ($userPreferencesStore.sortBy = name as SortMode),
                                    }) as ContextMenuItem,
                            ),
                            { type: "SEPARATOR" },
                            {
                                type: "ITEM",
                                label: $_(`search.show_details`),
                                icon: showDetails ? TablerIconCheck : IconBlank,
                                action: () => {
                                    if (showDetails) $page.url.searchParams.delete("showDetails");
                                    else $page.url.searchParams.set("showDetails", "true");
                                    goto(`?${$page.url.searchParams.toString()}`);
                                },
                            } as ContextMenuItem,
                        ],
                    }}
                >
                    {$userPreferencesStore.sortBy != "none"
                        ? `${$_("search.sorted_by")} ${$_(`search.sorted_by.${$userPreferencesStore.sortBy}`)}`
                        : "Unsorted"}
                </button>
            </div>
        </div>

        <div
            class="grid grid-cols-1 gap-2"
            class:lg:grid-cols-2={!$userPreferencesStore.compact}
            class:md:grid-cols-2={$userPreferencesStore.compact}
            class:lg:grid-cols-3={$userPreferencesStore.compact}
        >
            <PackageList
                {showDetails}
                compact={$userPreferencesStore.compact}
                packages={{
                    results: packages,
                    hits: packages.length,
                    page: 1,
                    pages: 1,
                    total: packages.length,
                }}
            />
        </div>
    </div>
{:else if loadingState == "failed"}
    <!-- <p>Something went wrong (this package doesn't seem to exist)</p> -->
    {(() => {
        toasts.trigger({
            message: "User not found!",
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        history.back();

        return undefined;
    })() || "Please wait, redirecting..."}
{/if}
