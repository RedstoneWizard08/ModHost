<script lang="ts">
    import { _, locales, locale } from "svelte-i18n";
    import { page } from "$app/state";
    import TablerIconUpload from "$components/icons/TablerIconUpload.svelte";
    import {
        contextMenu,
        openContextMenu,
        type ContextMenuItem,
        type ContextMenuProps,
    } from "$lib/ui/contextMenu";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import TablerIconWorld from "$components/icons/TablerIconWorld.svelte";
    import TablerIconCheck from "$components/icons/TablerIconCheck.svelte";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import TablerIconBack from "$components/icons/TablerIconArrowLeft.svelte";
    import TablerIconColorSwatch from "$components/icons/TablerIconColorSwatch.svelte";
    import { goto } from "$app/navigation";
    import TablerIconLogin2 from "$components/icons/TablerIconLogin2.svelte";
    import TablerIconLogin from "$components/icons/TablerIconLogin.svelte";
    import { setToken } from "$lib/api";
    import TablerIconUser from "$components/icons/TablerIconUser.svelte";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";
    import { user, userPreferencesStore } from "$lib/user";

    const modals = getModalStore();

    const langContextMenu = $derived({
        initiator: "left" as const,
        items: [
            {
                type: "ITEM",
                label: $_("auth_icon.back"),
                icon: TablerIconBack,
                action: () => openContextMenu(userContextMenu),
            },

            { type: "SEPARATOR", header: $_("menu.language") },

            ...$locales.map((lang) => ({
                type: "ITEM",
                label: $_("name", { locale: lang }),
                icon: $userPreferencesStore.locale == lang ? TablerIconCheck : IconBlank,
                action: () => {
                    $userPreferencesStore.locale = lang;
                    $locale = lang;
                },
            })),
        ] as ContextMenuItem[],
    });

    const themeContextMenu = $derived({
        initiator: "left" as const,
        items: [
            {
                type: "ITEM",
                label: $_("auth_icon.back"),
                icon: TablerIconBack,
                action: () => openContextMenu(userContextMenu),
            },

            { type: "SEPARATOR", header: $_("menu.theme") },

            ...[
                { label: "ModHost", name: "modhost" },
                { label: "KJSPKG", name: "kjspkg" },
                { label: "Astro", name: "astro" },
                {},
                { label: "Wintry", name: "wintry" },
                { label: "Crimson", name: "crimson" },
                { label: "Serenity", name: "serenity" },
                { label: "Hamlindigo", name: "hamlindigo" },
                { label: "Modern", name: "modern" },
                { label: "Rocket", name: "rocket" },
                { label: "Sahara", name: "sahara" },
                { label: "Seafoam", name: "seafoam" },
                { label: "Skeleton", name: "skeleton" },
                { label: "Vintage", name: "vintage" },
            ].map(({ label, name }) => {
                if (!label || !name) return { type: "SEPARATOR" };

                return {
                    type: "ITEM" as const,
                    label,
                    icon: $userPreferencesStore.theme == name ? TablerIconCheck : IconBlank,
                    action: () => {
                        document.documentElement.classList.add("color-animated");
                        $userPreferencesStore.theme = name;
                        document.body.dataset.theme =
                            $userPreferencesStore.theme ?? siteConfig.defaultTheme;
                    },
                };
            }),
        ] as ContextMenuItem[],
    });

    const userContextMenu: ContextMenuProps = $derived({
        initiator: "left",
        items: (() => {
            const out: ContextMenuItem[] = [];

            if ($user) {
                out.push(
                    {
                        type: "ITEM",
                        label: $_("auth_icon.your_profile"),
                        icon: TablerIconUser,
                        action: () => goto(`/u/${$user.username}`),
                    },
                    {
                        type: "ITEM",
                        label: $_(`auth_icon.create.${siteConfig.type}`),
                        icon: TablerIconUpload,
                        action: () => goto("/new"),
                    },
                );
            } else {
                out.push({
                    type: "ITEM",
                    label: $_("auth_icon.login"),
                    icon: TablerIconLogin2,
                    action: () => goto(`/api/v1/auth/github/login?redirect_uri=${page.url}`),
                });
            }

            out.push(
                {
                    type: "SEPARATOR",
                },
                {
                    type: "ITEM",
                    label: $_("auth_icon.language"),
                    icon: TablerIconWorld,
                    action: () => openContextMenu(langContextMenu),
                },
                {
                    type: "ITEM",
                    label: $_("auth_icon.theme"),
                    icon: TablerIconColorSwatch,
                    action: () => openContextMenu(themeContextMenu),
                },
            );

            if ($user) {
                out.push(
                    {
                        type: "SEPARATOR",
                    },
                    {
                        type: "ITEM",
                        label: $_("auth_icon.logout"),
                        icon: TablerIconLogin,
                        action: () => {
                            document.cookie = "";
                            setToken(undefined);
                            window.location.reload();
                        },
                    },
                );
            }

            return out;
        })(),
    });
</script>

{#if !$user}
    <button class="variant-soft-primary btn-icon" use:contextMenu={userContextMenu}>
        <Icon icon="tabler:user" height="24" />
    </button>
{:else}
    <button
        class="btn-icon duration-300 hover:scale-110 hover:brightness-100"
        use:contextMenu={userContextMenu}
    >
        <img
            class="rounded-full"
            src="https://avatars.githubusercontent.com/u/{$user.github_id}"
            alt="avatar"
        />
    </button>
{/if}
