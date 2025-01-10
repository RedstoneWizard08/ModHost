<script lang="ts">
    import { client } from "$lib/api";
    import { capText } from "$lib/util";
    import { unwrapOrNull, type FullProject } from "@modhost/api";
    import { onMount } from "svelte";

    interface Props {
        pkg: FullProject;
        index: number;
        inHandler: (index: number) => () => void;
        outHandler: (index: number) => () => void;
    }

    const { pkg, index, inHandler, outHandler }: Props = $props();

    let img = $state<string | undefined>(undefined);

    onMount(async () => {
        const gallery = unwrapOrNull(await client.project(pkg.id).gallery().list());

        if (gallery && gallery.length > 0) img = gallery[0].url;
    });
</script>

<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<a
    class="border-surface-500 bg-surface-700 hover:bg-surface-500 flex cursor-pointer flex-row gap-4 rounded-xl border-[1px] p-4 transition-all"
    href="/p/{pkg.slug}"
    onmouseover={inHandler(index)}
    onmouseleave={outHandler(index)}
>
    {#if img}
        <img src={img} alt="package icon" class="my-auto mr-4 aspect-square h-12 rounded-lg" />
    {:else if pkg.authors[0].github_id == -1}
        <img
            src="/modhost.png"
            alt="author's profile avatar"
            class="rounded-token my-auto mr-1 aspect-square h-10"
        />
    {:else}
        <img
            src={`https://avatars.githubusercontent.com/u/${pkg.authors[0].github_id}`}
            alt="author's profile avatar"
            class="rounded-token my-auto mr-1 aspect-square h-10"
        />
    {/if}
    <div class="project-info flex flex-col">
        <span class="title font-bold">
            {pkg.name}
        </span>
        <span class="description">
            {capText(pkg.description, 40)}
        </span>
    </div>
</a>
