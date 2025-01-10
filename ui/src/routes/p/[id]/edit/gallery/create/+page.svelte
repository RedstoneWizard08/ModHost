<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { currentProject, editSaving } from "$lib/state";
    import Icon from "@iconify/svelte";
    import { FileDropzone, getToastStore, popup, type PopupSettings } from "@skeletonlabs/skeleton";
    import { Carta, MarkdownEditor } from "carta-md";
    import { goto } from "$app/navigation";
    import { formatBytes } from "$lib/util";
    import { client } from "$lib/api";
    import { unwrap } from "@modhost/api";

    const id = $derived($page.params.id);
    const editor = new Carta();
    const toasts = getToastStore();

    const imageFormats = [".png", ".jpg", ".jpeg", ".gif", ".webp"];

    let name = $state("");
    let ordering = $state(-1);
    let description = $state("");
    let imageUrl = $state<string | undefined>(undefined);
    let files = $state<File[]>([]);

    const hasFile = $derived(files.length >= 1);

    const onFileChange = async (ev: Event & { target: HTMLInputElement }) => {
        files = ev.target.files ? [...ev.target.files] : [];

        if (files.length >= 1) {
            const data = await files[0].arrayBuffer();
            const blob = URL.createObjectURL(new Blob([data]));

            imageUrl = blob;
        }
    };

    const save = async () => {
        if (files.length < 1) {
            toasts.trigger({
                message: `You must choose an image before uploading!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        $editSaving = true;

        const res = await client
            .project(id)
            .gallery()
            .upload({
                name,
                description: description == "" ? undefined : description,
                ordering,
                file: files[0],
            });

        if (!res) {
            toasts.trigger({
                message: `Error uploading your image!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            $editSaving = false;
            return;
        }

        $currentProject = unwrap(await client.project(id).get());
        $editSaving = false;

        goto(`/p/${id}/edit/gallery`);
    };

    const orderingInfoPopup: PopupSettings = {
        event: "hover",
        target: "orderingInfoPopup",
        placement: "bottom",
    };
</script>

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:upload" height="24" class="mr-2" />
    Upload Image
</p>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:label" height="24" class="mr-2" />
        Name
    </p>

    <input type="text" placeholder="Example: My Image" class="input rounded-md" bind:value={name} />
</div>

<div class="card variant-glass-surface w-full p-4">
    <div class="flex w-full flex-row items-center justify-between">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:arrows-sort" height="24" class="mr-2" />
            Ordering
        </p>

        <div use:popup={orderingInfoPopup} class="flex flex-row items-center justify-end">
            <Icon
                icon="tabler:info-circle"
                height="24"
                class="text-success-500 pointer-events-none mr-2"
            />
        </div>

        <div class="bg-secondary-700 z-20 rounded-lg p-4" data-popup="orderingInfoPopup">
            A higher number will be displayed first, and a lower number last.
        </div>
    </div>

    <input type="number" placeholder="Example: -1" class="input rounded-md" bind:value={ordering} />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:file-description" height="24" class="mr-2" />
        Edit Description (Optional)
    </p>

    <MarkdownEditor carta={editor} bind:value={description} mode="tabs" />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:upload" height="24" class="mr-2" />
        Upload Image
    </p>

    <FileDropzone name="file" onchange={onFileChange} accept={imageFormats.join(",")}>
        {#snippet lead()}
            <div class="flex w-full flex-row items-center justify-center">
                {#if imageUrl}
                    <img src={imageUrl} alt="Uploaded File" />
                {:else}
                    <Icon icon="tabler:file-upload" height="24" />
                {/if}
            </div>
        {/snippet}

        {#snippet message()}
            {#if hasFile}
                <p>{files[0].name}</p>
                <p>{formatBytes(files[0].size)}</p>
            {:else}
                <b>Choose</b> or <b>drag and drop</b> your file here
            {/if}
        {/snippet}

        {#snippet meta()}
            {#if !hasFile}
                Accepts {imageFormats.join(", ")}
            {/if}
        {/snippet}
    </FileDropzone>
</div>

<div class="flex flex-row items-center justify-start gap-2">
    <button
        type="button"
        class="variant-filled-primary btn hover:variant-ghost-primary hover:text-token mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
        onclick={save}
    >
        <Icon icon="tabler:upload" height="24" class="mr-2" />
        Upload
    </button>

    <a
        href="/p/{id}/edit/gallery"
        class="variant-filled-secondary btn hover:variant-ghost-secondary mt-2 flex flex-row items-center justify-center rounded-lg transition-all"
    >
        <Icon icon="tabler:arrow-left" height="24" class="mr-2" />
        Back
    </a>
</div>
