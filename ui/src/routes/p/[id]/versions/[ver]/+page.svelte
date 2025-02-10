<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { fixLoaderName, markdown, downloadFile, copyText } from "$lib/util";
    import { onMount } from "svelte";
    import { currentProject } from "$lib/state";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { tryAggregateVersions } from "$lib/vers";
    import { unwrapOrNull, type ProjectVersion } from "@modhost/api";
    import { user } from "$lib/user";
    import { client } from "$lib/api";
    import VersionFile from "$components/ui/VersionFile.svelte";

    const maxVersions = 10;
    const id = $derived($page.params.id);
    const ver = $derived($page.params.ver);
    const toasts = getToastStore();

    let done = $state(false);
    let downloading = $state(false);
    let version = $state<ProjectVersion | null>(null);
    let name = $state("");
    let changelog = $state<string | undefined>(undefined);

    const loaders = $derived((version as ProjectVersion | undefined)?.loaders ?? []);
    const gameVersions = $derived((version as ProjectVersion | undefined)?.game_versions ?? []);
    const aggVersions = $derived(tryAggregateVersions(gameVersions));

    const canEdit = $derived(
        ($currentProject && $user && !!$currentProject.authors.find((v) => v.id == $user.id)) ||
            ($user && $user.admin),
    );

    onMount(async () => {
        version = unwrapOrNull(await client.project(id).versions().version(ver).get());

        if (!$currentProject || !version) return;

        name = version.name;
        changelog = version.changelog;
    });

    let doneTimeout: number | undefined;

    const directDownload = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        if (!version || !$currentProject) return;

        downloading = true;

        const file = version.files[0];

        await downloadFile(
            `/api/v1/projects/${$currentProject}/versions/${version.id}/download/${file.id}`,
            file.file_name,
        );

        downloading = false;
        done = true;

        if (doneTimeout) clearTimeout(doneTimeout);

        doneTimeout = setTimeout(() => {
            done = false;
        }, 1000) as any;
    };

    const copyVersionId = async () => {
        if (!version) return;

        await copyText(version.id.toString(), toasts);
    };
</script>

<svelte:head>
    <title>{version?.name ?? $_("site.loading")} - {siteConfig.siteName}</title>
</svelte:head>

<div class="card flex w-full flex-col items-start justify-start p-4">
    <div class="flex w-full flex-row items-center justify-between">
        <span class="text-xl font-bold">{name}</span>

        <div class="flex flex-row items-center justify-end gap-2">
            {#if canEdit}
                <a
                    aria-label="Edit"
                    href="/p/{id}/edit/versions/edit/{ver}"
                    class="hover:variant-filled-primary flex flex-row items-center justify-center rounded-full p-2 transition-all"
                >
                    <Icon icon="tabler:pencil" height="24" />
                </a>
            {/if}
        </div>
    </div>

    <div class="mt-2 flex w-full flex-row items-center justify-between">
        <span class="text-sm opacity-80">
            <span class="font-bold">{$_("package.version.prefix")}</span>
            {version?.version_number}
        </span>

        <span class="flex flex-row items-center justify-end text-sm opacity-50">
            <span
                >{version?.downloads}
                {version?.downloads == 1
                    ? $_("list.download_singluar")
                    : $_("list.download_plural")}</span
            >
            &bull;
            {$_("id.version")}&nbsp;
            <button
                class="anchor flex select-text flex-row items-center justify-end no-underline"
                onclick={copyVersionId}
                >{version?.id}
                <Icon icon="tabler:copy" class="ml-1" /></button
            >
        </span>
    </div>
</div>

<div class="card grid w-full grid-cols-[1fr_1fr] gap-2 p-4">
    <div class="flex w-full flex-col items-start justify-start gap-2">
        <dt class="text-sm opacity-50">{$_("package.loaders_title")}</dt>

        {#if loaders.length > 0}
            <dd class="flex flex-wrap gap-1">
                {#each loaders as loader}
                    <span class="variant-filled-primary badge select-text"
                        >{fixLoaderName(loader)}</span
                    >
                {/each}
            </dd>
        {:else}
            <dd class="flex flex-wrap gap-1">
                <span class="variant-filled-primary badge select-text">{$_("package.unknown")}</span
                >
            </dd>
        {/if}
    </div>

    <div class="flex w-full flex-col items-start justify-start gap-2">
        <dt class="text-sm opacity-50">{$_("package.available_for")}</dt>

        {#if gameVersions.length > 0}
            <dd class="flex flex-wrap gap-1">
                {#if aggVersions.length > maxVersions}
                    {#each aggVersions.slice(0, maxVersions) as version}
                        <span class="variant-filled-primary badge select-text">{version}</span>
                    {/each}
                    <span class="variant-filled-primary badge select-text">...</span>
                {:else}
                    {#each aggVersions as version}
                        <span class="variant-filled-primary badge select-text">{version}</span>
                    {/each}
                {/if}
            </dd>
        {:else}
            <dd class="flex flex-wrap gap-1">
                <span class="variant-filled-primary badge select-text">{$_("package.unknown")}</span
                >
            </dd>
        {/if}
    </div>
</div>

<section class="card h-fit w-full p-4">
    <dt class="mb-2 text-sm opacity-50">
        {$_("package.version.changelog")}
    </dt>

    <dd class="style-markdown flex select-text flex-col items-start *:select-text">
        {@html markdown(changelog ?? "")}
    </dd>
</section>

<section class="card h-fit w-full p-4">
    <dt class="mb-2 text-sm opacity-50">
        {$_("package.version.files")}
    </dt>

    <dd class="flex w-full gap-1">
        <dl class="list-dl w-full">
            {#each version?.files ?? [] as file}
                <VersionFile {file} pkg={id} version={version!} />
            {/each}
        </dl>
    </dd>
</section>
