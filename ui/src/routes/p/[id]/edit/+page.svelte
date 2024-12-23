<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getPackage, updatePackage } from "$api";
    import { currentPackage, editSaving } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import {
        Autocomplete,
        getModalStore,
        popup,
        type AutocompleteOption,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";
    import { getLicenses } from "$lib/licenses";
    import type { ProjectVisibility } from "$lib/types";

    const id = $derived($page.params.id);
    const modals = getModalStore();

    let slug = $state("");
    let name = $state("");
    let repo = $state("");
    let issues = $state("");
    let wiki = $state("");
    let license = $state("");
    let visibility = $state<ProjectVisibility>("Public");
    let allLicenses = $state<AutocompleteOption<string, string>[]>([]);

    const realRepo = $derived(repo != "" ? repo : undefined);
    const realIssues = $derived(issues != "" ? issues : undefined);
    const realWiki = $derived(wiki != "" ? wiki : undefined);
    const realLicense = $derived(license != "" ? license : undefined);

    onMount(async () => {
        if (!$currentPackage) return;

        slug = $currentPackage.slug;
        name = $currentPackage.name;
        repo = $currentPackage.source ?? "";
        issues = $currentPackage.issues ?? "";
        wiki = $currentPackage.wiki ?? "";
        license = $currentPackage.license ?? "";
        visibility = $currentPackage.visibility;

        allLicenses = (await getLicenses()).map((v) => ({ value: v, label: v }));
    });

    const save = async () => {
        $editSaving = true;

        await updatePackage(id, {
            name,
            visibility,
            source: realRepo,
            issues: realIssues,
            wiki: realWiki,
            license: realLicense,
        });

        $currentPackage = await getPackage(id);

        slug = $currentPackage?.slug ?? slug;
        name = $currentPackage?.name ?? name;
        repo = $currentPackage?.source ?? repo;
        issues = $currentPackage?.issues ?? issues;
        wiki = $currentPackage?.wiki ?? wiki;
        license = $currentPackage?.license ?? license;
        visibility = $currentPackage?.visibility ?? visibility;

        $editSaving = false;
    };

    const deleteProject = async () => {
        modals.trigger({
            type: "component",
            component: "confirmDelete",
        });
    };

    const licensesPopup: PopupSettings = {
        event: "focus-click",
        target: "licensesAutocomplete",
        placement: "bottom",
    };

    const onLicenseSelect = (ev: CustomEvent<AutocompleteOption<string, string>>) => {
        license = ev.detail.value;
    };
</script>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:settings" height="24" class="mr-2" />
    General Settings
</p>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:link" height="24" class="mr-2" />
        Slug
    </p>

    <input
        type="text"
        placeholder="Example: my-package"
        class="input rounded-md"
        bind:value={slug}
        disabled
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:eye" height="24" class="mr-2" />
        Display Name
    </p>

    <input
        type="text"
        placeholder="Example: My Package"
        class="input rounded-md"
        bind:value={name}
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:code" height="24" class="mr-2" />
        Source Code
    </p>

    <input
        type="text"
        placeholder="Example: https://github.com/example/example"
        class="input rounded-md"
        bind:value={repo}
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:exclamation-circle" height="24" class="mr-2" />
        Issue Tracker
    </p>

    <input
        type="text"
        placeholder="Example: https://github.com/example/example/issues"
        class="input rounded-md"
        bind:value={issues}
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:world" height="24" class="mr-2" />
        Wiki
    </p>

    <input
        type="text"
        placeholder="Example: https://github.com/example/example/wiki"
        class="input rounded-md"
        bind:value={wiki}
    />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:license" height="24" class="mr-2" />
        License
    </p>

    <input
        type="text"
        name="autocomplete-license"
        placeholder="Choose a license (or type your own)"
        class="autocomplete input rounded-md"
        bind:value={license}
        use:popup={licensesPopup}
    />

    <div
        data-popup="licensesAutocomplete"
        class="h-[50%] w-[40%] overflow-scroll rounded-lg bg-secondary-700 p-2"
    >
        <Autocomplete bind:input={license} options={allLicenses} on:selection={onLicenseSelect} />
    </div>
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:eye" height="24" class="mr-2" />
        Visibility
    </p>

    <select class="select cursor-pointer rounded-lg !outline-none" bind:value={visibility}>
        <option value="Public">Public</option>
        <option value="Private">Private</option>
        <option value="Unlisted">Unlisted</option>
    </select>
</div>

<div class="flex flex-row items-center justify-start gap-2">
    <button
        type="button"
        class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg transition-all hover:variant-ghost-primary hover:text-token"
        onclick={save}
    >
        <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
        Save
    </button>

    <button
        type="button"
        class="variant-filled-error btn mt-2 flex flex-row items-center justify-center rounded-lg transition-all hover:variant-ghost-error"
        onclick={deleteProject}
    >
        <Icon icon="tabler:trash" height="24" class="mr-2" />
        Delete Project
    </button>
</div>
