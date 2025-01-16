<script lang="ts">
    import BetterStep from "$components/ui/stepper/BetterStep.svelte";
    import Icon from "@iconify/svelte";
    import type { ProjectVisibility } from "@modhost/api";
    import {
        Autocomplete,
        popup,
        type AutocompleteOption,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";

    interface Props {
        license: string;
        visibility: ProjectVisibility;
        allLicenses: AutocompleteOption<string, string>[];
    }

    let {
        license = $bindable(),
        visibility = $bindable(),
        allLicenses = $bindable(),
    }: Props = $props();

    const onLicenseSelect = (ev: CustomEvent<AutocompleteOption<string, string>>) => {
        license = ev.detail.value;
    };

    const licensesPopup: PopupSettings = {
        event: "focus-click",
        target: "licensesAutocomplete",
        placement: "bottom",
    };
</script>

{#snippet header()}
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:adjustments" height="24" class="mr-2" />
        Extra Information
    </p>
{/snippet}

<BetterStep {header}>
    <div class="card variant-soft-secondary w-full p-4">
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
    </div>

    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:eye" height="24" class="mr-2" />
            Visibility
        </p>

        <select
            class="select variant-ghost-primary cursor-pointer !outline-none"
            bind:value={visibility}
        >
            <option value="Public">Public</option>
            <option value="Private">Private</option>
            <option value="Unlisted">Unlisted</option>
        </select>
    </div>
</BetterStep>

<div
    data-popup="licensesAutocomplete"
    class="bg-secondary-700 h-[50%] w-[50%] overflow-scroll rounded-lg p-2"
>
    <Autocomplete bind:input={license} bind:options={allLicenses} on:selection={onLicenseSelect} />
</div>
