<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/state";
    import { fly } from "svelte/transition";
    import { type Snippet } from "svelte";
    import { getToastStore, ProgressRadial } from "@skeletonlabs/skeleton";
    import { currentProject, editLoadingState, editSaving } from "$lib/state";
    import { siteConfig } from "$lib/config";
    import { vsprintf } from "sprintf-js";
    import EditNav from "$components/ui/edit/EditNav.svelte";

    const id = $derived(page.params.id);

    interface Props {
        children: Snippet;
    }

    const { children }: Props = $props();
</script>

<svelte:head>
    <title
        >{$currentProject
            ? vsprintf($_("site.editing"), [$currentProject.name])
            : $_("site.loading")} - {siteConfig.siteName}</title
    >
</svelte:head>

{#if $editLoadingState != "failed"}
    {#if $editSaving}
        <div
            class="bg-primary-900 fixed left-0 right-0 top-0 z-50 flex h-full w-full flex-row items-center justify-center bg-opacity-25 text-white"
            in:fly={{ y: 20 }}
            out:fly={{ y: 20 }}
        >
            <ProgressRadial width="w-20" />
        </div>
    {/if}

    <h1 class="h3 mx-4 mb-1 font-bold">
        <span class="h3 font-bold" in:fly={{ y: 20 }}
            >{$currentProject
                ? vsprintf($_("site.editing"), [$currentProject.name])
                : $_("site.loading")}</span
        >
    </h1>

    <div class="flex w-full flex-col gap-2 md:flex-row">
        <EditNav {id} />

        <section
            class="card flex h-full max-h-full w-full flex-col items-start justify-start gap-2 overflow-y-scroll p-4"
        >
            {#if $editLoadingState == "loading"}
                <div class="placeholder m-2 mx-auto w-32 animate-pulse"></div>
            {:else if $editLoadingState == "ready" && $currentProject}
                {@render children?.()}
            {/if}
        </section>
    </div>
{:else}
    <!-- <p>Something went wrong (this package doesn't seem to exist)</p> -->
    {(() => {
        getToastStore().trigger({
            message: `Mod/Package Broken`,
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        history.back();

        return undefined;
    })() || "Please wait, redirecting..."}
{/if}
