<script lang="ts">
    import { capText, splitToRows } from "$lib/util";
    import { onMount } from "svelte";
    import ScrollingProject from "./ScrollingProject.svelte";
    import { unwrapOrNull, type FullProject } from "@modhost/api";
    import { client } from "$lib/api";

    const rows = 3;
    const maxPkgs = 30;
    const rowElements: HTMLDivElement[] = [];

    let projects: FullProject[] = $state([]);
    let selected = $derived(splitToRows(projects, rows));

    onMount(async () => {
        const pkgs = unwrapOrNull(await client.search(undefined, 1, 100));

        if (pkgs) {
            projects = pkgs.hits >= maxPkgs ? pkgs.results.slice(0, maxPkgs) : pkgs.results;
        }
    });

    const inHandler = (idx: number) => {
        return () => {
            const el = rowElements[idx];

            if (el) {
                el.style.animationPlayState = "paused";
            }
        };
    };

    const outHandler = (idx: number) => {
        return () => {
            const el = rowElements[idx];

            if (el) {
                el.style.animationPlayState = "running";
            }
        };
    };
</script>

<div class="mt-16 flex flex-col space-y-4">
    {#each selected as items, index}
        <div
            class="hide-scrollbar flex w-screen select-none flex-row gap-6 overflow-hidden whitespace-nowrap"
        >
            <div
                class="animate-scroll flex min-w-full flex-shrink-0 gap-6 whitespace-nowrap"
                class:anim-reverse={!(index % 2 == 0)}
                class:anim-mid={!(index % 2 == 0)}
                bind:this={rowElements[index]}
            >
                {#each items as pkg}
                    <ScrollingProject {index} {pkg} {inHandler} {outHandler} />
                {/each}
            </div>
        </div>
    {/each}
</div>
