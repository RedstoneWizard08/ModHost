<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { currentProject, editSaving } from "$lib/state";
    import Icon from "@iconify/svelte";
    import { Carta, MarkdownEditor } from "carta-md";
    import { client } from "$lib/api";
    import { unwrap } from "@modhost/api";

    const id = $derived($page.params.id);
    const editor = new Carta();

    let readme = $state("");

    onMount(() => {
        if (!$currentProject) return;

        readme = $currentProject.readme;
    });

    const save = async () => {
        $editSaving = true;

        const api = client.project(id);

        unwrap(
            await api.update({
                readme,
            }),
        );

        $currentProject = unwrap(await api.get());

        readme = $currentProject.readme;

        $editSaving = false;
    };
</script>

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:file-description" height="24" class="mr-2" />
    Edit Description
</p>

<div class="card variant-glass-surface w-full p-4">
    <MarkdownEditor carta={editor} bind:value={readme} mode="tabs" />
</div>

<button
    type="button"
    class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg"
    onclick={save}
>
    <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
    Save
</button>
