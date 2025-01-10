<script lang="ts">
    import { page } from "$app/stores";
    import { cubicInOut } from "svelte/easing";
    import { crossfade } from "svelte/transition";

    interface Tab {
        routes: string[];
        url: string;
        text: string;
    }

    const { tabs }: { tabs: Tab[] } = $props();

    const [send, receive] = crossfade({
        duration: 250,
        easing: cubicInOut,
    });
</script>

<div class="card inline-flex w-full items-center justify-start gap-2 rounded-lg p-2">
    {#each tabs as tab}
        <a
            href={tab.url}
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md"
        >
            <div class="grid">
                {#if tab.routes.includes($page.route.id ?? "")}
                    <span
                        class="bg-primary-800 col-start-1 row-start-1 rounded-full transition-all"
                        in:send={{ key: "activetab" }}
                        out:receive={{ key: "activetab" }}
                    ></span>
                {/if}

                <span class="z-10 col-start-1 row-start-1 rounded-full px-3 py-1 transition-all"
                    >{tab.text}</span
                >
            </div>
        </a>
    {/each}
</div>
