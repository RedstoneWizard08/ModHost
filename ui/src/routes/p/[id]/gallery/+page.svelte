<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { currentProject } from "$lib/state";
    import { client } from "$lib/api";
    import { unwrapOrNull, type GalleryImage as GalleryImageInfo } from "@modhost/api";
    import GalleryImage from "$components/ui/GalleryImage.svelte";

    const id = $derived($page.params.id);

    let images: GalleryImageInfo[] = $state([]);

    const sortedImages = $derived([...images].sort((a, b) => a.ordering - b.ordering).reverse());

    onMount(async () => {
        if (!$currentProject) return;

        images = unwrapOrNull(await client.project(id).gallery().list()) ?? [];
    });
</script>

{#if sortedImages.length > 0}
    <div class="card h-fit w-full space-y-2 p-4">
        <dt class="text-sm opacity-50">Gallery</dt>

        <dd class="grid w-full grid-cols-[1fr_1fr] gap-2 md:grid-cols-[1fr_1fr_1fr]">
            {#each sortedImages as img}
                <GalleryImage {img} />
            {/each}
        </dd>
    </div>
{:else}
    <div class="card flex h-fit w-full flex-col space-y-2 p-4">
        <dt class="text-sm opacity-50">Gallery</dt>
        <span class="w-full py-8 text-center">No images found!</span>
    </div>
{/if}
