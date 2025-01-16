<script lang="ts">
    import { _ } from "svelte-i18n";
    import { onMount } from "svelte";
    import { currentProject, editSaving, updateSearchResults } from "$lib/state";
    import Icon from "@iconify/svelte";
    import { getToastStore, type AutocompleteOption } from "@skeletonlabs/skeleton";
    import { goto } from "$app/navigation";
    import { Carta } from "carta-md";
    import { siteConfig } from "$lib/config";
    import { ErrorResponse, type ProjectVisibility } from "@modhost/api";
    import { licenses } from "$lib/meta";
    import { client } from "$lib/api";
    import Step1 from "./Step1.svelte";
    import Step2 from "./Step2.svelte";
    import Step3 from "./Step3.svelte";
    import Step4 from "./Step4.svelte";
    import BetterStepper from "$components/ui/stepper/BetterStepper.svelte";

    let name = $state("");
    let slug = $state("");
    let readme = $state("");
    let description = $state("");
    let repo = $state("");
    let issues = $state("");
    let wiki = $state("");
    let license = $state("");
    let visibility = $state<ProjectVisibility>("Public");
    let allLicenses = $state<AutocompleteOption<string, string>[]>([]);

    const editor = new Carta();
    const toasts = getToastStore();
    const realRepo = $derived(repo != "" ? repo : undefined);
    const realIssues = $derived(issues != "" ? issues : undefined);
    const realWiki = $derived(wiki != "" ? wiki : undefined);
    const realLicense = $derived(license != "" ? license : undefined);

    onMount(async () => {
        if (!$currentProject) return;

        allLicenses = $licenses.map((v) => ({ value: v, label: v }));
    });

    const save = async () => {
        $editSaving = true;

        const data = await client.createProject({
            name,
            slug,
            visibility,
            source: realRepo,
            issues: realIssues,
            wiki: realWiki,
            license: realLicense,
            readme,
            description,
        });

        if (data instanceof ErrorResponse) {
            $editSaving = false;

            toasts.trigger({
                message: `Error creating your project: ${data} (${data.cause})`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        await updateSearchResults(true);

        $editSaving = false;
        goto(`/p/${data.slug}`);
    };
</script>

<svelte:head>
    <title>Create Project - {siteConfig.siteName}</title>
</svelte:head>

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:plus" height="24" class="mr-2" />
    Package Creation
</p>

{#snippet buttonComplete(locked: boolean, clickHandler: () => void)}
    <div class="flex flex-row items-center justify-start gap-2">
        <button
            type="button"
            class="variant-filled-primary btn hover:variant-ghost-primary hover:text-token mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
            onclick={clickHandler}
        >
            <Icon icon="tabler:plus" height="24" class="mr-2" />
            Create
        </button>
    </div>
{/snippet}

{#snippet buttonBack(locked: boolean, clickHandler: () => void)}
    <div class="flex flex-row items-center justify-start gap-2">
        <button
            type="button"
            class="variant-filled-secondary btn hover:variant-ghost-primary disabled:!variant-ghost-primary hover:text-token mt-2 flex flex-row items-center justify-center rounded-lg transition-all disabled:!text-white"
            onclick={clickHandler}
            disabled={locked}
        >
            <Icon icon="tabler:arrow-left" height="24" class="mr-2" />
            Back
        </button>
    </div>
{/snippet}

{#snippet buttonNext(locked: boolean, clickHandler: () => void)}
    <div class="flex flex-row items-center justify-start gap-2">
        <button
            type="button"
            class="variant-filled-secondary btn hover:variant-ghost-primary disabled:!variant-ghost-primary hover:text-token mt-2 flex flex-row items-center justify-center rounded-lg transition-all disabled:!text-white"
            onclick={clickHandler}
            disabled={locked}
        >
            Next
            <Icon icon="tabler:arrow-right" height="24" class="ml-2" />
        </button>
    </div>
{/snippet}

<div class="flex h-full w-full flex-row items-start justify-start">
    <div
        class="bg-primary-500 bg-opacity-20 mr-4 flex h-full w-[40%] max-w-[40%] flex-col items-start justify-start overflow-scroll rounded-xl p-2 px-3 space-y-1"
    >
        <p class="border-b-primary-700 mb-1 w-full border-b py-1 text-lg font-bold">
            Package Overview
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:tag" height="24" />
            <span class="font-bold">Display Name:</span>
            <span>{name == "" ? "Unset" : name}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:link" height="24" />
            <span class="font-bold">Slug:</span>
            <span>{slug == "" ? "Unset" : slug}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:info-circle" height="24" />
            <span class="font-bold">Description:</span>
            <span>{description == "" ? "Unset" : description}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:code" height="24" />
            <span class="font-bold">Repository:</span>
            <span>{repo == "" ? "Unset" : repo}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:exclamation-circle" height="24" />
            <span class="font-bold">Issues:</span>
            <span>{issues == "" ? "Unset" : issues}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:world" height="24" />
            <span class="font-bold">Wiki:</span>
            <span>{wiki == "" ? "Unset" : wiki}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:license" height="24" />
            <span class="font-bold">License:</span>
            <span>{license == "" ? "Unset" : license}</span>
        </p>
        <p class="flex flex-row items-center justify-start flex-wrap space-x-1">
            <Icon icon="tabler:eye" height="24" />
            <span class="font-bold">Visibility:</span>
            <span>{visibility}</span>
        </p>
    </div>

    <BetterStepper complete={save} {buttonBack} {buttonNext} {buttonComplete} class="w-full">
        <Step1 bind:name bind:slug bind:description />
        <Step2 bind:repo bind:issues bind:wiki />
        <Step3 bind:license bind:visibility bind:allLicenses />
        <Step4 {editor} bind:readme />
    </BetterStepper>
</div>
