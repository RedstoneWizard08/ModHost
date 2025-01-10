<script lang="ts">
    import { fade, slide } from "svelte/transition";
    import { contextMenuStore } from "$lib/ui";
</script>

{#if $contextMenuStore}
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button
        class="bg-surface-backdrop-token absolute inset-0 z-[998] h-full w-full cursor-default"
        transition:fade
        oncontextmenu={(e) => {
            $contextMenuStore = undefined;
            e.preventDefault();

            setTimeout(() => {
                // Act like a right click at the mouse position, so that the context menu at that position opens.
                // Emulate the click, do not use this callback function - that position might not have the same context menu!
                let element = document.elementFromPoint(e.clientX, e.clientY);
                if (!element) return;
                element.dispatchEvent(e);
            }, 200);
        }}
        onclick={() => ($contextMenuStore = undefined)}
    ></button>

    <div
        class="card variant-glass-surface absolute z-[999] m-2 max-h-screen overflow-y-scroll p-1"
        style="left: {$contextMenuStore.x}px; top: {$contextMenuStore.y}px;"
        class:invisible={$contextMenuStore.invisible}
        role="menu"
        tabindex="-1"
        id="GLOBAL-ctxm"
        onkeypress={() => ($contextMenuStore = undefined)}
        onclick={() => ($contextMenuStore = undefined)}
        transition:slide={{ axis: "y", duration: 300 }}
    >
        <div class="flex flex-col gap-1" class:mb-16={$contextMenuStore.items.length > 12}>
            {#key $contextMenuStore}
                {#each $contextMenuStore.items as item, i}
                    {#if item.type == "SEPARATOR"}
                        {#if i > 0}
                            <hr />
                        {/if}

                        <dd class="px-2 pt-1 text-sm opacity-50" class:hidden={!item.header}>
                            {item.header}
                        </dd>
                    {:else if item.type == "ITEM"}
                        <button
                            onclick={async (ev) => item.type == "ITEM" && (await item.action(ev))}
                            class="rounded-container-token hover:variant-soft-primary flex items-center gap-2 overflow-hidden whitespace-nowrap p-2 pl-3 pr-12"
                        >
                            {#if item.icon}
                                <item.icon />
                            {/if}

                            <span>{item.label}</span>
                        </button>
                    {/if}
                {:else}
                    <span class="p-2 text-sm opacity-50">No items... :(</span>
                {/each}
            {/key}
        </div>
    </div>
{/if}
