<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { onMount } from "svelte";
    import ProjectScroller from "$components/ui/ProjectScroller.svelte";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";

    const addCharDelay = 150;

    let tagline = siteConfig.tagline;
    let adding = $state(true);
    let text = $state("");

    const canAddChar = () => text.length < tagline.length;

    const addChar = () => {
        const index = text.length;

        if (canAddChar()) {
            text += tagline[index];
        }
    };

    const scheduleAddChar = () => {
        addChar();

        if (canAddChar()) {
            setTimeout(scheduleAddChar, addCharDelay);
        } else {
            setTimeout(() => {
                adding = false;
            }, addCharDelay);
        }
    };

    onMount(() => {
        scheduleAddChar();
    });

    locale.subscribe(() => {
        tagline = siteConfig.tagline;
        text = "";
        scheduleAddChar();
    });
</script>

<svelte:head>
    <title>Home - {siteConfig.siteName}</title>
</svelte:head>

<div
    class="from-primary-900 via-secondary-700 to-surface-900 m-0 flex w-full flex-col items-center justify-center bg-gradient-to-b from-10% to-90% p-10 py-40"
>
    <!-- When you can't decide what font to use: -->
    <h2 class="flex flex-row items-center justify-center text-3xl md:text-4xl">
        <span>{siteConfig.siteName}</span>

        {#if siteConfig.showBeta}
            <span class="variant-filled-primary badge ml-4">{$_("site.beta")}</span>
        {/if}
    </h2>

    <span class="animate-border-blink mt-2 border-r-2 pr-1 text-xl font-bold">
        {text}
    </span>

    <div class="mt-16 flex flex-col items-center justify-center space-y-4 md:flex-row md:space-y-0">
        <a href="/s" class="variant-filled-primary btn">
            <span><Icon icon="tabler:search" height="24" class="mr-2" /></span>
            <span>{$_(`site.browse.${siteConfig.type}`)}</span>
        </a>
    </div>

    <ProjectScroller />
</div>
