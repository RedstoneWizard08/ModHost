<script lang="ts">
    import { page } from "$app/state";
    import { user } from "$lib/user";
    import Icon from "@iconify/svelte";
    import { onMount } from "svelte";

    const { children } = $props();
    let mounted = $state(false);

    onMount(() => (mounted = true));
</script>

{#if mounted}
    {#if $user && $user.admin}
        <div class="flex h-full w-full flex-row items-start justify-start">
            <div class="card mr-4 flex h-full w-[35%] flex-col gap-2 self-baseline p-4">
                <p class="mb-2 flex flex-row items-center justify-start text-lg">
                    <Icon icon="tabler:user-shield" width="24" class="mr-2" />
                    Admin
                </p>

                <a
                    href="/admin"
                    class="btn btn-md variant-soft-primary hover:variant-filled-primary flex flex-row items-center justify-start rounded-md transition-all"
                    class:!variant-filled-primary={page.route.id == "/admin"}
                >
                    <Icon icon="tabler:chart-histogram" width="20" class="mr-2" />
                    Stats
                </a>

                <a
                    href="/admin/users"
                    class="btn btn-md variant-soft-primary hover:variant-filled-primary flex flex-row items-center justify-start rounded-md transition-all"
                    class:!variant-filled-primary={page.route.id?.startsWith("/admin/users")}
                >
                    <Icon icon="tabler:users" width="20" class="mr-2" />
                    Manage Users
                </a>

                <a
                    href="/admin/projects"
                    class="btn btn-md variant-soft-primary hover:variant-filled-primary flex flex-row items-center justify-start rounded-md transition-all"
                    class:!variant-filled-primary={page.route.id?.startsWith("/admin/projects")}
                >
                    <Icon icon="tabler:folders" width="20" class="mr-2" />
                    Manage Projects
                </a>
            </div>

            <div class="flex h-full w-full flex-row items-start justify-start">
                {@render children?.()}
            </div>
        </div>
    {:else}
        <div class="flex h-full w-full flex-col items-center justify-center space-y-4">
            <div class="flex flex-row items-center justify-center">
                <Icon icon="tabler:alert-triangle" width="24" class="mr-2" />
                <p class="text-lg">You don't have access to this page!</p>
            </div>

            <a
                href="/"
                class="btn btn-md variant-glass-error hover:variant-filled-error transition-all"
                >Back to Home</a
            >
        </div>
    {/if}
{:else}
    <p class="flex h-full w-full flex-col items-center justify-center">Loading...</p>
{/if}
