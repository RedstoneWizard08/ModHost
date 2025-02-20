<script lang="ts">
    import { page } from "$app/state";
    import { client } from "$lib/api";
    import { siteConfig } from "$lib/config";
    import { formatBytes } from "$lib/util";
    import Icon from "@iconify/svelte";
    import { AdminStatsSocketWrapper, ErrorResponse, unwrap, type AdminStats } from "@modhost/api";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { Duration } from "luxon";
    import { onMount } from "svelte";

    let uptime = $state<Duration>(Duration.fromObject({}));
    let stopLoop = true;
    let ws: AdminStatsSocketWrapper | undefined;
    let globalStats = $state<AdminStats | undefined>(undefined);

    const toasts = getToastStore();

    onMount(() => {
        const proto = page.url.protocol.startsWith("https") ? "wss" : "ws";

        ws = client.admin().statsSocket(page.url.host, proto);

        ws.onData((data) => {
            globalStats = data;

            uptime = Duration.fromObject({ seconds: data.uptime_secs });
            uptime = uptime.rescale();
        });
    });

    const getStats = async () => {
        const stats = await client.admin().stats();

        if (stats instanceof ErrorResponse) {
            toasts.trigger({
                message: stats.message,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            throw stats;
        }

        uptime = Duration.fromObject({ seconds: stats.uptime_secs });
        uptime = uptime.rescale();

        if (!stopLoop) setTimeout(addSecondLoop, 1000);

        globalStats = unwrap(stats);
    };

    const addSecondLoop = () => {
        uptime = uptime.plus({ seconds: 1 });
        uptime = uptime.rescale();

        if (!stopLoop) setTimeout(addSecondLoop, 1000);
    };
</script>

<svelte:head>
    <title>Admin - {siteConfig.siteName}</title>
</svelte:head>

{#await getStats()}
    <p class="flex h-full w-full flex-col items-center justify-center">Loading...</p>
{:then}
    <div class="flex w-full flex-col items-start justify-start">
        <p class="mx-2 mb-2 flex flex-row items-center justify-start text-lg font-bold">
            <Icon icon="tabler:chart-histogram" width="20" class="mr-2" />
            Instance Stats
        </p>

        <div class="grid h-full w-full grid-cols-3 gap-2">
            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:clock" width="20" class="mr-2" />
                    Uptime
                </p>

                <p>{uptime.toHuman()}</p>
            </div>
        </div>

        <hr class="my-4 mt-6 w-full" />

        <p class="mx-2 mb-2 flex flex-row items-center justify-start text-lg font-bold">
            <Icon icon="tabler:database" width="20" class="mr-2" />
            Database
        </p>

        <div class="grid h-full w-full grid-cols-3 gap-2">
            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:folders" width="20" class="mr-2" />
                    Projects
                </p>

                <p>{globalStats?.projects}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:versions" width="20" class="mr-2" />
                    Versions
                </p>

                <p>{globalStats?.versions}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:files" width="20" class="mr-2" />
                    Uploaded Files
                </p>

                <p>{globalStats?.files}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:library-photo" width="20" class="mr-2" />
                    Uploaded Images
                </p>

                <p>{globalStats?.images}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:users" width="20" class="mr-2" />
                    Users
                </p>

                <p>{globalStats?.users}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:search" width="20" class="mr-2" />
                    Indexed Projects
                </p>

                <p>{globalStats?.indexed_projects}</p>
            </div>
        </div>

        <hr class="my-4 mt-6 w-full" />

        <p class="mx-2 mb-2 flex flex-row items-center justify-start text-lg font-bold">
            <Icon icon="tabler:bucket" width="20" class="mr-2" />
            Storage
        </p>

        <div class="grid h-full w-full grid-cols-3 gap-2">
            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:folders" width="20" class="mr-2" />
                    Projects Bucket Size
                </p>

                <p>{formatBytes(globalStats?.projects_size_bytes ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:library-photo" width="20" class="mr-2" />
                    Gallery Bucket Size
                </p>

                <p>{formatBytes(globalStats?.gallery_size_bytes ?? 0)}</p>
            </div>
        </div>

        <hr class="my-4 mt-6 w-full" />

        <p class="mx-2 mb-2 flex flex-row items-center justify-start text-lg font-bold">
            <Icon icon="tabler:server" width="20" class="mr-2" />
            System
        </p>

        <div class="grid h-full w-full grid-cols-3 gap-2">
            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:cpu-2" width="20" class="mr-2" />
                    Kernel Version
                </p>

                <p>{globalStats?.sys_info.kernel}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:label" width="20" class="mr-2" />
                    System Name
                </p>

                <p>{globalStats?.sys_info.sys_name}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:device-desktop" width="20" class="mr-2" />
                    OS Version
                </p>

                <p>{globalStats?.sys_info.os_version}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:arrows-shuffle-2" width="20" class="mr-2" />
                    Total Memory
                </p>

                <p>{formatBytes(globalStats?.sys_info.total_mem ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:arrows-shuffle" width="20" class="mr-2" />
                    Used Memory
                </p>

                <p>{formatBytes(globalStats?.sys_info.used_mem ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:lock-open-2" width="20" class="mr-2" />
                    Free Memory
                </p>

                <p>{formatBytes(globalStats?.sys_info.free_mem ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:arrows-shuffle-2" width="20" class="mr-2" />
                    Total Swap
                </p>

                <p>{formatBytes(globalStats?.sys_info.total_swap ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:arrows-shuffle" width="20" class="mr-2" />
                    Used Swap
                </p>

                <p>{formatBytes(globalStats?.sys_info.used_swap ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:lock-open-2" width="20" class="mr-2" />
                    Free Swap
                </p>

                <p>{formatBytes(globalStats?.sys_info.free_swap ?? 0)}</p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:percentage" width="20" class="mr-2" />
                    Memory Usage
                </p>

                <p>
                    {Math.round(
                        ((globalStats?.sys_info.used_mem ?? 0) /
                            (globalStats?.sys_info.total_mem ?? 0)) *
                            100,
                    )}%
                </p>
            </div>

            <div class="card hover:bg-surface-600 cursor-pointer space-y-2 p-4 transition-all">
                <p class="flex flex-row items-center justify-start text-sm font-bold">
                    <Icon icon="tabler:percentage" width="20" class="mr-2" />
                    Swap Usage
                </p>

                <p>
                    {globalStats?.sys_info.total_swap == 0
                        ? "N/A"
                        : Math.round(
                              ((globalStats?.sys_info.used_swap ?? 0) /
                                  (globalStats?.sys_info.total_swap ?? 0)) *
                                  100,
                          ) + "%"}
                </p>
            </div>
        </div>
    </div>
{/await}
