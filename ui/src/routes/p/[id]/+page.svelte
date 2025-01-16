<script lang="ts">
    import { _ } from "svelte-i18n";
    import { markdown } from "$lib/util";
    import { onMount } from "svelte";
    import { currentProject } from "$lib/state";

    let readme = $state("");

    onMount(async () => {
        if (!$currentProject) return;

        readme = $currentProject.readme;
    });
</script>

<section class="card h-fit w-full p-4">
    <dt class="mb-2 text-sm opacity-50">Description</dt>
    <dd
        class="style-markdown flex select-text flex-col items-start overflow-x-scroll *:select-text"
    >
        {@html markdown(readme)}
    </dd>
</section>
