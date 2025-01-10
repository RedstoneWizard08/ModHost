<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { fly } from "svelte/transition";
    import Icon from "@iconify/svelte";
    import { siteConfig } from "$lib/config";
</script>

<svelte:head>
    <title>Authentication Required - {siteConfig.siteName}</title>
</svelte:head>

<div class="flex flex-1 flex-col items-center justify-center" transition:fly={{ y: 40 }}>
    <Icon
        icon="tabler:alert-triangle"
        class="h-36 w-auto text-[rgb(var(--color-primary-500))] lg:h-60"
    />
    <h1 class="h3 w-64 text-center lg:w-96">{$_("author.auth_required")}</h1>

    <a
        class="card hover:variant-soft-primary mt-4 flex gap-2 px-5 py-3"
        href="/api/v1/auth/github/login?redirect_uri={$page.url}"
    >
        <Icon icon="tabler:brand-github" />
        {$_("author.auth_required.login")}
    </a>
</div>
