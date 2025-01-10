<script lang="ts">
    import type { GalleryImage } from "@modhost/api";
    import { getModalStore } from "@skeletonlabs/skeleton";

    const { img }: { img: GalleryImage } = $props();
    const modals = getModalStore();

    const openViewModal = () => {
        modals.trigger({
            type: "component",
            component: "imageView",
            meta: { img },
        });
    };
</script>

<button
    type="button"
    onclick={openViewModal}
    class="card hover:variant-soft-primary flex h-full w-full cursor-pointer flex-col items-center justify-start gap-y-4 p-2 transition-all"
>
    <p class="w-[90%] overflow-clip text-ellipsis text-left text-lg font-bold">{img.name}</p>

    <img src={img.url} alt={img.name} class="aspect-square w-[90%] rounded-lg object-cover" />
</button>
