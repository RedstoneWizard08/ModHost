<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import { formatDate, downloadFile } from "$lib/util";
    import Icon from "@iconify/svelte";
    import { unwrapOrNull, type ProjectVersion } from "@modhost/api";
    import { client } from "$lib/api";

    interface Props {
        version: ProjectVersion;
        pkg: string;
    }

    const { version, pkg }: Props = $props();
    const modals = getModalStore();

    let downloading = $state(false);
    let done = $state(false);

    const handleDelete = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        modals.trigger({
            type: "component",
            component: "confirmDeleteVersion",
            meta: { versionId: version.id },
        });
    };

    let doneTimeout: number | undefined;

    const directDownload = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        downloading = true;

        const file = version.files[0];

        await downloadFile(
            `/api/v1/projects/${pkg}/versions/${version.id}/download/${file.id}`,
            file.file_name,
        );

        downloading = false;
        done = true;

        if (doneTimeout) clearTimeout(doneTimeout);

        doneTimeout = setTimeout(() => {
            done = false;
        }, 1000) as any;
    };
</script>

<a
    href="/p/{pkg}/edit/versions/edit/{version.id}"
    class="rounded-container-token hover:variant-soft-primary flex w-full items-center gap-2 p-2 text-left transition-all"
>
    <button
        type="button"
        class="variant-filled-secondary btn hover:variant-outline-primary p-2 transition-all"
        onclick={directDownload}
    >
        {#if done}
            <Icon icon="tabler:check" height="24" />
        {:else if downloading}
            <Icon icon="tabler:loader-2" height="24" class="animate-spin" />
        {:else}
            <Icon icon="tabler:download" height="24" />
        {/if}
    </button>

    <span class="ml-1 flex-auto">
        <dt class="select-text font-bold">{version.name}</dt>
        <dd class="text-sm opacity-50">{formatDate(new Date(version.created_at))}</dd>
    </span>

    <!-- This has no onclick handler because it just passes through to the underlying link -->
    <button
        class="variant-glass-primary btn btn-sm hover:variant-filled-primary transition-all"
        type="button"
    >
        <Icon icon="tabler:pencil" height="24" />
    </button>

    <button
        class="variant-glass-error btn btn-sm hover:variant-filled-error transition-all"
        type="button"
        onclick={handleDelete}
    >
        <Icon icon="tabler:trash" height="24" />
    </button>
</a>
