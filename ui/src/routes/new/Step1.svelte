<script lang="ts">
    import BetterStep from "$components/ui/stepper/BetterStep.svelte";
    import { client } from "$lib/api";
    import { createSlug } from "$lib/util";
    import Icon from "@iconify/svelte";
    import { unwrapOrNull } from "@modhost/api";

    interface Props {
        name: string;
        slug: string;
        description: string;
    }

    let { name = $bindable(), slug = $bindable(), description = $bindable() }: Props = $props();
    let slugError = $state(false);

    const updateSlug = async () => {
        slugError = false;
        slug = createSlug(name);
        slugError = !!unwrapOrNull(await client.project(slug).get());
    };
</script>

{#snippet header()}
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:info-circle" height="24" class="mr-2" />
        General Information
    </p>
{/snippet}

<BetterStep {header}>
    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:eye" height="24" class="mr-2" />
            Display Name
        </p>

        <input
            type="text"
            placeholder="Example: My Package"
            class="input rounded-md"
            oninput={updateSlug}
            bind:value={name}
        />
    </div>

    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:link" height="24" class="mr-2" />
            Slug
        </p>

        <input
            type="text"
            placeholder="Example: my-package"
            class="input rounded-md"
            bind:value={slug}
        />

        {#if slugError}
            <p class="text-error-500 ml-1 mt-2">Project already exists!</p>
        {/if}
    </div>

    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:info-circle-filled" height="24" class="mr-2" />
            Summary
        </p>

        <input
            type="text"
            placeholder="A short description of your project"
            class="input rounded-md"
            bind:value={description}
        />
    </div>
</BetterStep>
