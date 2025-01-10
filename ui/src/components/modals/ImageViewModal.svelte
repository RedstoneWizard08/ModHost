<script lang="ts">
    import { markdown } from "$lib/util";
    import type { GalleryImage } from "@modhost/api";
    import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";

    const modals = getModalStore();
    const toasts = getToastStore();
    let img: GalleryImage;

    onMount(() => {
        if (!$modals[0].meta || !("img" in $modals[0].meta)) {
            toasts.trigger({
                message: `Error: Missing property 'img' in $modals[0].meta!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            modals.close();

            return;
        }

        img = $modals[0].meta.img;
    });
</script>

{#if $modals[0] && img}
    <div class="bg-surface-500 relative rounded-lg p-8 shadow-xl overflow-scroll max-h-[95vh]">
        <header class="text-2xl font-bold">{img.name}</header>

        <img src={img.url} alt={img.name} class="my-4" />

        <div class="style-markdown flex select-text flex-col items-start *:select-text">
            {@html markdown(img.description ?? "")}
        </div>

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-filled-secondary btn hover:variant-ghost-primary mr-2 !outline-none"
                onclick={() => modals.close()}>Close</button
            >
        </footer>
    </div>
{/if}
