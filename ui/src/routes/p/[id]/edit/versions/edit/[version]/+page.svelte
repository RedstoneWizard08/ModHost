<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/state";
    import { onDestroy, onMount } from "svelte";
    import { currentProject, editSaving } from "$lib/state";
    import Icon from "@iconify/svelte";
    import {
        Autocomplete,
        getModalStore,
        InputChip,
        popup,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";
    import { siteConfig } from "$lib/config";
    import { gameVersions as allGameVersions, loaders as allLoaders } from "$lib/meta";
    import { elementPopup, type PopupControls } from "$lib/ui/popups";
    import { Carta, MarkdownEditor } from "carta-md";
    import { unwrap, unwrapOrNull, type ProjectVersion } from "@modhost/api";
    import { client } from "$lib/api";

    const id = $derived(page.params.id);
    const verId = $derived(page.params.version);
    const editor = new Carta();
    const modals = getModalStore();

    let snapshots = $state(false);
    let versionsOpen = $state(false);

    const availGameVersions: [string, boolean][] = $derived(
        ($allGameVersions || []).map((v: { id: string; beta: boolean }) => [v.id, !v.beta]),
    );

    const destroyHandlers: (() => void)[] = [];
    const releaseVersions = $derived(availGameVersions.filter((v) => v[1]).map((v) => v[0]));
    const betaVersions = $derived(availGameVersions.map((v) => v[0]));
    const shownGameVersions = $derived(snapshots ? betaVersions : releaseVersions);

    let versionChips = $state("");
    let versionsInput: InputChip = $state(null!);
    let popupRef: PopupControls | undefined = $state(undefined);

    const versionsAutocomplete: PopupSettings = {
        event: "focus-click",
        target: "versionsAutocomplete",
        placement: "bottom",
    };

    let ver = $state<ProjectVersion | null>(null);
    let versionNumber = $state("");
    let name = $state("");
    let changelog = $state("");
    let loaders = $state<string[]>([]);
    let gameVersions = $state<string[]>([]);

    const lowerLoaders = $derived(loaders.map((v) => v.toLowerCase()));

    onMount(async () => {
        if (!$currentProject || !verId) return;

        ver = unwrapOrNull(await client.project(id).versions().version(verId).get());

        versionNumber = ver?.version_number ?? "";
        name = ver?.name ?? "";
        changelog = ver?.changelog ?? "";
        loaders = ver?.loaders ?? [];
        gameVersions = ver?.game_versions ?? [];

        const el = document.querySelector("[data-ref=versionInputChip]") as HTMLElement | null;

        el?.addEventListener("focus", () => (versionsOpen = true));
        el?.addEventListener("blur", () => (versionsOpen = false));

        if (el) {
            popupRef = elementPopup(el, versionsAutocomplete);

            destroyHandlers.push(popupRef.destroy);
        }
    });

    onDestroy(() => {
        for (const handler of destroyHandlers) handler();
    });

    const save = async () => {
        $editSaving = true;

        await client.project(id).versions().version(verId).update({
            changelog,
            game_versions: gameVersions,
            loaders,
            name,
            version_number: versionNumber,
        });

        $currentProject = unwrap(await client.project(id).get());
        ver = unwrapOrNull(await client.project(id).versions().version(verId).get());

        versionNumber = ver?.version_number ?? "";
        name = ver?.name ?? "";
        changelog = ver?.changelog ?? "";
        loaders = ver?.loaders ?? [];
        gameVersions = ver?.game_versions ?? [];

        $editSaving = false;
    };

    const deleteVersion = async () => {
        modals.trigger({
            type: "component",
            component: "confirmDeleteVersion",
            meta: { versionId: verId },
        });
    };

    const versionNumberInfoPopup: PopupSettings = {
        event: "hover",
        target: "versionNumberInfoPopup",
        placement: "bottom",
    };

    const toggleLoader = (loader: string) => {
        if (lowerLoaders.includes(loader.toLowerCase())) {
            loaders = loaders.filter((l) => l.toLowerCase() !== loader.toLowerCase());
        } else {
            loaders = [...loaders, loader];
        }
    };
</script>

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:pencil" height="24" class="mr-2" />
    Edit Version
</p>

<div class="card variant-glass-surface w-full p-4">
    <div class="flex w-full flex-row items-center justify-between">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:hash" height="24" class="mr-2" />
            Version Number
        </p>

        <div use:popup={versionNumberInfoPopup} class="flex flex-row items-center justify-end">
            <Icon
                icon="tabler:info-circle"
                height="24"
                class="text-success-500 pointer-events-none mr-2"
            />
        </div>

        <div class="bg-secondary-700 z-20 rounded-lg p-4" data-popup="versionNumberInfoPopup">
            This must be in
            <a href="https://semver.org/" class="anchor">SemVer</a>
            format.
        </div>
    </div>

    <input
        type="text"
        placeholder="Example: v0.1.0"
        class="input rounded-md"
        bind:value={versionNumber}
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:eye" height="24" class="mr-2" />
        Display Name
    </p>

    <input
        type="text"
        placeholder="Example: 1.0.0 - The Greatest Version"
        class="input rounded-md"
        bind:value={name}
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:file-power" height="24" class="mr-2" />
        Mod Loaders
    </p>

    <div class="flex flex-row items-center lg:m-2 lg:mt-4">
        {#each $allLoaders ?? [] as loader}
            <button
                type="button"
                class="chip mx-1 text-base !outline-none {lowerLoaders.includes(
                    loader.id.toLowerCase(),
                )
                    ? 'variant-filled-primary'
                    : 'variant-soft-primary'}"
                onclick={() => toggleLoader(loader.id)}>{loader.name}</button
            >
        {/each}
    </div>
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:versions" height="24" class="mr-2" />
        Game versions
    </p>
    <div class="mt-4 grid w-full grid-cols-[1fr_auto] overflow-hidden transition duration-200">
        <InputChip
            bind:this={versionsInput}
            bind:input={versionChips}
            bind:value={gameVersions}
            name="chips"
            class="max-h-40 !min-w-fit overflow-scroll !outline-none"
            placeholder={$_("modal.upload_version.placeholder.game")}
            whitelist={betaVersions}
            data-ref="versionInputChip"
        />

        <button
            type="button"
            class="variant-form-material border-surface-500 ml-1 flex max-h-[calc(24px+1rem)] items-center justify-between px-4 !outline-none"
            onclick={() =>
                (
                    document.querySelector("[data-ref=versionInputChip]") as HTMLElement | null
                )?.focus()}
        >
            <Icon icon="tabler:caret-down" height="24" rotate={versionsOpen ? 180 : 0} />
        </button>
    </div>

    <div class="my-2 flex flex-row items-center justify-between">
        <div class="mr-2 flex flex-row items-center justify-start">
            <input class="checkbox variant-soft-primary" type="checkbox" bind:checked={snapshots} />
            <p class="ml-2">{$_(`modal.upload_version.checkbox.${siteConfig.betaName}`)}</p>
        </div>

        <div class="flex flex-row items-center justify-end">
            <button
                type="button"
                class="variant-filled-primary btn btn-sm !outline-none"
                onclick={() => (gameVersions = shownGameVersions)}
            >
                {$_("modal.upload_version.select_all")}
            </button>

            <button
                type="button"
                class="variant-filled-error btn btn-sm ml-2 !outline-none"
                onclick={() => (gameVersions = [])}
            >
                {$_("modal.upload_version.clear")}
            </button>
        </div>
    </div>
</div>

<div
    class="card variant-filled-secondary z-20 ml-7 max-h-48 w-[40%] overflow-y-auto rounded-md p-2"
    tabindex="-1"
    data-popup="versionsAutocomplete"
>
    {#if availGameVersions.length > 0}
        <Autocomplete
            bind:input={versionChips}
            options={shownGameVersions.map((v) => ({ value: v, label: v }))}
            denylist={gameVersions}
            on:selection={(ev) => versionsInput.addChip(ev.detail.value)}
        />
    {/if}
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:file-description" height="24" class="mr-2" />
        Edit Changelog
    </p>

    <MarkdownEditor carta={editor} bind:value={changelog} mode="tabs" />
</div>

<div class="flex flex-row items-center justify-start gap-2">
    <button
        type="button"
        class="variant-filled-primary btn hover:variant-ghost-primary hover:text-token mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
        onclick={save}
    >
        <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
        Save
    </button>

    <button
        type="button"
        class="variant-filled-error btn hover:variant-ghost-error mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
        onclick={deleteVersion}
    >
        <Icon icon="tabler:trash" height="24" class="mr-2" />
        Delete Version
    </button>
</div>
