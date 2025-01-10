<script lang="ts">
    import { _ } from "svelte-i18n";
    import { Drawer, getDrawerStore } from "@skeletonlabs/skeleton";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";

    const drawerStore = getDrawerStore();

    const closeDrawer = () => {
        drawerStore.close();
    };
</script>

<Drawer>
    {#if $drawerStore.id == "home"}
        <div class="flex flex-col p-4">
            <p class="flex items-center gap-2">
                <img
                    src="/favicon.png"
                    alt="logo"
                    class="rounded-token aspect-square w-8 min-w-8"
                />

                <span>{siteConfig.siteName}</span>

                {#if siteConfig.showBeta}
                    <span class="variant-filled-secondary badge">{$_("site.beta")}</span>
                {/if}
            </p>

            <div class="mt-4 flex flex-col space-y-4">
                <a
                    href="/"
                    class="btn-primary transition-duration-300 variant-soft-primary btn hover:variant-filled-secondary mr-4 flex flex-row items-center justify-start outline-none transition-all"
                    onclick={closeDrawer}
                >
                    <Icon icon="tabler:home" height="24" />
                    <span>{$_("search.home")}</span>
                </a>

                <a
                    href="/s"
                    class="btn-primary transition-duration-300 variant-soft-primary btn hover:variant-filled-secondary mr-4 flex flex-row items-center justify-start outline-none transition-all"
                    onclick={closeDrawer}
                >
                    <Icon icon="tabler:search" height="24" />
                    <span>{$_("search.browse")}</span>
                </a>
            </div>
        </div>
    {/if}
</Drawer>
