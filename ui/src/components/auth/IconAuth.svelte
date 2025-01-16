<script lang="ts">
    import { popup, type PopupSettings } from "@skeletonlabs/skeleton";
    import Icon from "@iconify/svelte";
    import { user } from "$lib/user";
    import { popupsDidMount } from "$lib/state";

    const headerPopup: PopupSettings = {
        event: "focus-click",
        target: "headerPopup",
        placement: "bottom-end",
        middleware: {
            offset: {
                mainAxis: 10,
                crossAxis: -10,
            },
        },
    };
</script>

{#if $popupsDidMount}
    {#if !$user}
        <button
            class="variant-soft-primary btn-icon"
            use:popup={headerPopup}
            data-headerPopupRoot="noAuth"
        >
            <Icon icon="tabler:user" height="24" class="transition-all hover:scale-110" />
        </button>
    {:else}
        <button class="btn-icon duration-300" use:popup={headerPopup} data-headerPopupRoot="auth">
            <img
                class="rounded-full !transition-all duration-300 hover:scale-110"
                src="https://avatars.githubusercontent.com/u/{$user.github_id}"
                alt="avatar"
            />
        </button>
    {/if}
{/if}
