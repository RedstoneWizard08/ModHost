<script lang="ts">
    import { _ } from "svelte-i18n";
    import { base } from "$app/paths";
    import { page } from "$app/state";
    import { formatDate } from "$lib/util";
    import { onMount } from "svelte";
    import { unwrapOrNull, type FullProject } from "@modhost/api";
    import { client } from "$lib/api";

    interface Props {
        pkg: FullProject;
        customHeight?: number;
        compact?: boolean;
        showAvatar?: boolean;
        showDetails?: boolean;
        showName?: boolean;
        select?: (id: number) => void | Promise<void>;
    }

    const { pkg, customHeight, compact, showAvatar, showDetails, showName, select }: Props =
        $props();

    let img = $state<string | undefined>(undefined);

    onMount(async () => {
        const gallery = unwrapOrNull(await client.project(pkg.id).gallery().list());

        if (gallery && gallery.length > 0) img = gallery[0].url;
    });
</script>

<a
    href={`${base}/p/${pkg.slug}`}
    class="card hover:variant-soft-primary flex p-4"
    class:flex-col={compact}
    onclick={() => select?.(pkg.id)}
    class:!variant-filled-primary={page.url.searchParams.get("id") == pkg.name}
    style={customHeight != null ? `height: ${customHeight}rem` : ""}
>
    {#if showAvatar && !compact}
        {#if img}
            <img src={img} alt="package icon" class="my-auto mr-4 aspect-square h-16 rounded-lg" />
        {:else if pkg.authors[0].github_id == -1}
            <img
                src="/modhost.png"
                alt="author's profile avatar"
                class="rounded-token my-auto mr-4 aspect-square h-8"
            />
        {:else}
            <img
                src={`https://avatars.githubusercontent.com/u/${pkg.authors[0].github_id}`}
                alt="author's profile avatar"
                class="rounded-token my-auto mr-4 aspect-square h-8"
            />
        {/if}
    {/if}
    <dl class="my-auto">
        <dt class="mb-1 select-text font-bold">{pkg.name}</dt>
        <dd class="text-sm opacity-50">
            {#if showName}
                {$_("list.by")} <span class="select-text">{pkg.authors[0].username}</span>
            {/if}
        </dd>
        <dd class="text-sm opacity-50">
            <span
                >{pkg.downloads}
                {pkg.downloads == 1
                    ? $_("list.download_singluar")
                    : $_("list.download_plural")}</span
            >
        </dd>
        {#if showDetails}
            <dd class="text-sm opacity-50">
                {$_("list.published")}
                <span class="select-text">{formatDate(new Date(pkg.created_at))}</span>
            </dd>
            <dd class="text-sm opacity-50">
                {$_("list.updated")}
                <span class="select-text">{formatDate(new Date(pkg.updated_at))}</span>
            </dd>
        {/if}
    </dl>
</a>
