<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { currentProject, editSaving } from "$lib/state";
    import Icon from "@iconify/svelte";
    import {
        Autocomplete,
        getModalStore,
        popup,
        type AutocompleteOption,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";
    import { unwrapOrNull, type ProjectVisibility } from "@modhost/api";
    import { licenses } from "$lib/meta";
    import { client } from "$lib/api";

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
        if (!$currentProject) return;

        slug = $currentProject.slug;
        name = $currentProject.name;
        repo = $currentProject.source ?? "";
        issues = $currentProject.issues ?? "";
        wiki = $currentProject.wiki ?? "";
        license = $currentProject.license ?? "";
        visibility = $currentProject.visibility;

        allLicenses = $licenses.map((v) => ({ value: v, label: v }));
    });

    const save = async () => {
        $editSaving = true;

        await client.project(id).update({
            name,
            visibility,
            source: realRepo,
            issues: realIssues,
            wiki: realWiki,
            license: realLicense,
        });

        $currentProject = unwrapOrNull(await client.project(id).get());

        slug = $currentProject?.slug ?? slug;
        name = $currentProject?.name ?? name;
        repo = $currentProject?.source ?? repo;
        issues = $currentProject?.issues ?? issues;
        wiki = $currentProject?.wiki ?? wiki;
        license = $currentProject?.license ?? license;
        visibility = $currentProject?.visibility ?? visibility;

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

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:settings" height="24" class="mr-2" />
    General Settings
</p>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
        class="bg-secondary-700 h-[50%] w-[40%] overflow-scroll rounded-lg p-2"
    >
        <Autocomplete bind:input={license} options={allLicenses} on:selection={onLicenseSelect} />
    </div>
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
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
        class="variant-filled-primary btn hover:variant-ghost-primary hover:text-token mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
        onclick={save}
    >
        <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
        Save
    </button>

    <button
        type="button"
        class="variant-filled-error btn hover:variant-ghost-error mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
        onclick={deleteProject}
    >
        <Icon icon="tabler:trash" height="24" class="mr-2" />
        Delete Project
    </button>
</div>
