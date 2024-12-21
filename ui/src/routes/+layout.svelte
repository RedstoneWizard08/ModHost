<script lang="ts">
    import "../app.pcss";
    import "highlight.js/styles/github-dark.min.css";

    import "carta-md/default.css";
    import {
        currentScrollPosition,
        updateTagsStore,
        updateUserStore,
        userPreferencesStore,
    } from "$lib/stores";
    import {
        Modal,
        Toast,
        storePopup,
        initializeStores,
        ProgressRadial,
        type ModalComponent,
    } from "@skeletonlabs/skeleton";
    import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
    import { afterNavigate, beforeNavigate, onNavigate } from "$app/navigation";
    import { onMount, type Snippet } from "svelte";
    import { fly } from "svelte/transition";
    import HeaderBar from "$components/ui/HeaderBar.svelte";
    import ContextMenu from "$components/ui/ContextMenu.svelte";
    import { page } from "$app/stores";
    import { setToken } from "$api";
    import { updateGameVersionsIfNeeded } from "$lib/versions";
    import Drawers from "$components/ui/Drawers.svelte";
    import { siteConfig } from "$lib/config";
    import { updateModLoadersIfNeeded } from "$lib/loaders";
    import ConfirmDeleteModal from "$components/modals/ConfirmDeleteModal.svelte";
    import ConfirmDeleteVersionModal from "$components/modals/ConfirmDeleteVersionModal.svelte";
    import ConfirmDeleteImageModal from "$components/modals/ConfirmDeleteImageModal.svelte";
    import type { NavigationTarget } from "@sveltejs/kit";
    import { editRoutes, pkgRoutes } from "$lib/routes";
    import ImageViewModal from "$components/modals/ImageViewModal.svelte";

    const { data, children }: { data: any; children: Snippet } = $props();
    let navigating = $state(false);

    const modalRegistry: Record<string, ModalComponent> = {
        confirmDelete: { ref: ConfirmDeleteModal },
        confirmDeleteImage: { ref: ConfirmDeleteImageModal },
        confirmDeleteVersion: { ref: ConfirmDeleteVersionModal },
        imageView: { ref: ImageViewModal },
    };

    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
    initializeStores();

    const handleScroll = (e: Event) =>
        ($currentScrollPosition = {
            x: (e.currentTarget as Element).scrollLeft,
            y: (e.currentTarget as Element).scrollTop,
        });

    onMount(async () => {
        if ($page.url.searchParams.has("token")) {
            const token = $page.url.searchParams.get("token");
            setToken(token!);
            $page.url.searchParams.delete("token");
            history.replaceState(null, "", $page.url.toString());
        }

        if ($userPreferencesStore.lightMode) document.documentElement.classList.remove("dark");

        document.body.dataset.theme = $userPreferencesStore.theme ?? siteConfig.defaultTheme;

        (async () => {
            await Promise.all([
                updateUserStore(),
                updateGameVersionsIfNeeded(),
                updateModLoadersIfNeeded(),
                updateTagsStore(),
            ]);
        })();
    });

    beforeNavigate(() => (navigating = true));
    afterNavigate(() => (navigating = false));

    const isPackageRoute = (to?: NavigationTarget, from?: NavigationTarget) =>
        pkgRoutes.includes(to?.route.id ?? "") && pkgRoutes.includes(from?.route.id ?? "");

    const isEditRoute = (to?: NavigationTarget, from?: NavigationTarget) =>
        editRoutes.includes(to?.route.id ?? "") && editRoutes.includes(from?.route.id ?? "");

    onNavigate((navigation) => {
        if (
            navigation.to?.route.id == navigation.from?.route.id ||
            isPackageRoute(navigation.to ?? undefined, navigation.from ?? undefined) ||
            isEditRoute(navigation.to ?? undefined, navigation.from ?? undefined)
        )
            return;

        if (!document.startViewTransition) return;

        return new Promise((resolve) => {
            document.startViewTransition(async () => {
                resolve();
                await navigation.complete;
            });
        });
    });
</script>

<svelte:head>
    <title>Loading - {siteConfig.siteName}</title>
    <meta property="og:title" content={siteConfig.siteName} />
    <meta property="og:type" content="website" />
    <meta property="og:image" content="/favicon.png" />
    <meta property="og:description" content={siteConfig.tagline} />
    <meta name="theme-color" content={siteConfig.themeColor} />
</svelte:head>

<Toast position="br" max={8} />
<ContextMenu />
<Drawers />
<Modal components={modalRegistry} />

{#if navigating}
    <div
        class="card variant-soft-success absolute bottom-8 right-8 z-40 flex w-16 p-4"
        in:fly={{ x: 40, delay: 200 }}
        out:fly={{ x: 40 }}
    >
        <ProgressRadial stroke={40} meter="stroke-success-500" track="stroke-success-500/30" />
    </div>
{/if}

<div class="flex h-full w-full flex-col overflow-hidden">
    <header class="flex-none">
        <HeaderBar />
    </header>

    <div
        class="flex h-full w-full flex-col overflow-x-hidden"
        style:scrollbar-gutter="auto"
        onscroll={handleScroll}
    >
        <main class="flex-auto">
            {#if $page.route.id == "/"}
                <div class="container flex min-h-full w-full max-w-full flex-col">
                    {#key data.href}
                        {@render children?.()}
                    {/key}
                </div>
            {:else}
                <div
                    class="container mx-auto flex min-h-full max-w-screen-lg flex-col space-y-2 p-4"
                >
                    {#key data.href}
                        {@render children?.()}
                    {/key}
                </div>
            {/if}
        </main>

        <footer class="flex w-full flex-row items-center justify-between p-2">
            <span class="hidden md:inline">
                <a
                    href="https://github.com/RedstoneWizard08/ModHost"
                    class="anchor no-underline"
                    target="_blank">GitHub</a
                >
                &bull;
                <a
                    href="https://github.com/RedstoneWizard08/ModHost/wiki"
                    class="anchor no-underline"
                    target="_blank">Wiki</a
                >
                &bull;
                <a href="/api/v1/docs/scalar" class="anchor no-underline" target="_blank"
                    >API Docs</a
                >
                &bull;
                <a
                    href="https://crowdin.com/editor/kjspkg"
                    class="anchor no-underline"
                    target="_blank">Crowdin</a
                >
                <!-- TODO: Move the crowdin's name to "ModHost" -->
            </span>

            <span class="mt-auto hidden text-sm opacity-50 md:inline">
                Powered by
                <a
                    href="https://github.com/RedstoneWizard08/ModHost"
                    class="anchor no-underline"
                    target="_blank">ModHost</a
                >
            </span>
        </footer>
    </div>
</div>
