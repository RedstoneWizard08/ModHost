<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { currentProject, editSaving } from "$lib/state";
    import Icon from "@iconify/svelte";
    import { client } from "$lib/api";
    import { unwrap } from "@modhost/api";

    const id = $derived($page.params.id);

    onMount(() => {
        if (!$currentProject) return;

        // Fields go here
    });

    const save = async () => {
        $editSaving = true;

        // Update code goes here

        $currentProject = unwrap(await client.project(id).get());

        // Field update code goes here

        $editSaving = false;
    };
</script>

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:tags" height="24" class="mr-2" />
    Manage Tags
</p>

<p class="h3">Coming Soon!</p>

<!-- <button
    type="button"
    class="btn variant-filled-primary mt-2 flex flex-row items-center justify-center rounded-lg"
    onclick={save}
>
    <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
    Save
</button> -->
