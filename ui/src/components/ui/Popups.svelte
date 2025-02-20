<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import { user, userDownloads, userPreferencesStore } from "$lib/user";
    import Icon from "@iconify/svelte";
    import type { PopupSettings } from "@skeletonlabs/skeleton";
    import { onDestroy, onMount } from "svelte";
    import { locales } from "svelte-i18n";
    import { setToken } from "$lib/api";
    import { anchoredPopup } from "$lib/ui/anchoredPopup";
    import { siteConfig } from "$lib/config";
    import { page } from "$app/state";
    import { popupsDidMount } from "$lib/state";

    interface Theme {
        type: "theme";
        name: string;
        id: string;
    }

    interface Divider {
        type: "divider";
    }

    const themes: (Theme | Divider)[] = [
        { type: "theme", name: "ModHost", id: "modhost" },
        { type: "theme", name: "KJSPKG", id: "kjspkg" },
        { type: "theme", name: "Astro", id: "astro" },
        { type: "theme", name: "Murky", id: "murky" },
        { type: "divider" },
        { type: "theme", name: "Wintry", id: "wintry" },
        { type: "theme", name: "Crimson", id: "crimson" },
        { type: "theme", name: "Serenity", id: "serenity" },
        { type: "theme", name: "Hamlindigo", id: "hamlindigo" },
        { type: "theme", name: "Modern", id: "modern" },
        { type: "theme", name: "Rocket", id: "rocket" },
        { type: "theme", name: "Sahara", id: "sahara" },
        { type: "theme", name: "Seafoam", id: "seafoam" },
        { type: "theme", name: "Skeleton", id: "skeleton" },
        { type: "theme", name: "Vintage", id: "vintage" },
    ];

    const langsPopup: PopupSettings = {
        event: "focus-click",
        target: "langsPopup",
        placement: "bottom-end",
        middleware: {
            offset: {
                mainAxis: 10,
                crossAxis: -10,
            },
        },
    };

    const themePopup: PopupSettings = {
        event: "focus-click",
        target: "themePopup",
        placement: "bottom-end",
        middleware: {
            offset: {
                mainAxis: 10,
                crossAxis: -10,
            },
        },
    };

    const logout = (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        document.cookie = "";
        setToken(undefined);
        window.location.reload();
    };

    const getAnchor = () => {
        const authPopupTrigger = document.querySelector(
            '[data-headerPopupRoot="auth"]',
        ) as HTMLElement;

        const noAuthPopupTrigger = document.querySelector(
            '[data-headerPopupRoot="noAuth"]',
        ) as HTMLElement;

        return $user ? authPopupTrigger : noAuthPopupTrigger;
    };

    onMount(() => {
        $popupsDidMount = true;
    });

    onDestroy(() => {
        $popupsDidMount = false;
    });
</script>

<div class="card z-50 w-96 space-y-2 p-4 shadow-xl" data-popup="headerPopup">
    {#if $user}
        <a
            class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-between p-2 transition-all"
            href="/u/{$user.username}"
        >
            <div class="flex flex-row items-center justify-start space-x-4">
                <img
                    class="h-12 rounded-full"
                    src="https://avatars.githubusercontent.com/u/{$user.github_id}"
                    alt="avatar"
                />

                <div class="flex flex-col items-start justify-center">
                    <p class="text-xl">{$user.username}</p>

                    <p class="opacity-60">
                        {$userDownloads ?? 0}
                        {$userDownloads && $userDownloads == 1 ? "download" : "downloads"}
                    </p>
                </div>
            </div>

            <button
                class="btn-icon variant-soft-error hover:variant-filled-error p-2 transition-all"
                onclick={logout}
            >
                <Icon icon="tabler:logout" height="24" />
            </button>
        </a>

        <a
            href="/new"
            class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all"
        >
            <Icon icon="tabler:plus" height="24" />
            <p class="text-lg">{$_(`auth_icon.create.${siteConfig.type}`)}</p>
        </a>
    {:else}
        <a
            href="/api/v1/auth/github/login?redirect_uri={page.url}"
            class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all"
        >
            <Icon icon="tabler:login" height="24" />
            <p class="text-lg">{$_("auth_icon.login")}</p>
        </a>
    {/if}

    <button
        class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all"
        use:anchoredPopup={{
            anchor: getAnchor,
            ...langsPopup,
        }}
    >
        <Icon icon="tabler:world" height="24" />
        <p class="text-lg">{$_("auth_icon.language")}</p>
    </button>

    <button
        class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all"
        use:anchoredPopup={{
            anchor: getAnchor,
            ...themePopup,
        }}
    >
        <Icon icon="tabler:color-swatch" height="24" />
        <p class="text-lg">{$_("auth_icon.theme")}</p>
    </button>

    {#if $user && $user.admin}
        <a
            href="/admin"
            class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all"
        >
            <Icon icon="tabler:user-shield" height="24" />
            <p class="text-lg">Admin</p>
        </a>
    {/if}
</div>

<div
    class="card z-50 max-h-[85vh] w-72 space-y-2 overflow-y-scroll p-4 shadow-xl"
    data-popup="langsPopup"
>
    {#each $locales as lang}
        <button
            class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all duration-300"
            onclick={() => {
                $userPreferencesStore.locale = lang;
                $locale = lang;
            }}
        >
            {#if $userPreferencesStore.locale == lang}
                <Icon icon="tabler:check" height="24" />
            {:else}
                <IconBlank />
            {/if}

            <p class="text-lg">{$_("name", { locale: lang })}</p>
        </button>
    {/each}
</div>

<div
    class="card z-50 max-h-[85vh] w-72 space-y-2 overflow-y-scroll p-4 shadow-xl"
    data-popup="themePopup"
>
    {#each themes as theme}
        {#if theme.type == "divider"}
            <hr class="w-full" />
        {:else}
            <button
                class="card variant-glass-tertiary hover:variant-glass-primary flex w-full flex-row items-center justify-start space-x-2 p-2 transition-all duration-300"
                onclick={() => {
                    document.documentElement.classList.add("color-animated");
                    $userPreferencesStore.theme = theme.id;
                    document.body.dataset.theme =
                        $userPreferencesStore.theme ?? siteConfig.defaultTheme;
                }}
            >
                {#if $userPreferencesStore.theme == theme.id}
                    <Icon icon="tabler:check" height="24" />
                {:else}
                    <IconBlank />
                {/if}

                <p class="text-lg">{theme.name}</p>
            </button>
        {/if}
    {/each}
</div>
