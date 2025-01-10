<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
    import { goto } from "$app/navigation";
    import { currentProject, updateSearchResults } from "$lib/state";
    import { client } from "$lib/api";

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

        await client.project($currentProject.id).delete();
        await updateSearchResults(true);

        loading = false;
        modals.close();
        goto("/s");
    };
</script>

{#if $modals[0]}
    <div class="w-modal-slim bg-surface-500 relative rounded-lg p-8 shadow-xl">
        <header class="text-2xl font-bold">Confirm Deletion</header>

        <p>Are you sure you want to delete your project, {$currentProject?.name}?</p>

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
