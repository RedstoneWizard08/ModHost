<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
    import { goto } from "$app/navigation";
    import { currentProject } from "$lib/state";
    import { client } from "$lib/api";
    import { unwrap } from "@modhost/api";

    const modals = getModalStore();
    const toasts = getToastStore();
    let loading = $state(false);

    const confirmDelete = async () => {
        loading = true;

        if (!$currentProject) {
            toasts.trigger({
                message: `Internal error: $currentProject is undefined!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            loading = false;

            return;
        }

        if (!$modals[0].meta || !("imageId" in $modals[0].meta)) {
            toasts.trigger({
                message: `Internal error: $modals[0].meta.imageId is undefined!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            loading = false;

            return;
        }

        const { imageId } = $modals[0].meta;

        unwrap(await client.project($currentProject.id).gallery().image(imageId).delete());

        loading = false;
        modals.close();
        goto(`/p/${$currentProject.id}/edit/gallery`, { invalidateAll: true });
    };
</script>

{#if $modals[0]}
    <div class="w-modal-slim bg-surface-500 relative rounded-lg p-8 shadow-xl">
        <header class="text-2xl font-bold">Confirm Deletion</header>

        <p>Are you sure you want to delete this gallery image?</p>

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-filled-error btn hover:variant-ghost-error mr-2 !outline-none transition-all"
                disabled={loading}
                onclick={confirmDelete}>Delete</button
            >

            <button
                class="variant-filled-secondary btn hover:variant-ghost-primary mr-2 !outline-none transition-all"
                disabled={loading}
                onclick={() => modals.close()}>{$_("action.cancel")}</button
            >
        </footer>
    </div>
{/if}
