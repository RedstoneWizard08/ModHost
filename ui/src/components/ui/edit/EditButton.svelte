<script lang="ts">
    import { page } from "$app/state";
    import Icon from "@iconify/svelte";

    interface Props {
        id: string;
        icon: string;
        route: string;
        text: string;
        extraMatches?: string[];
    }

    const { id, route, icon, text, extraMatches = [] }: Props = $props();
    const urlPath = $derived(route.replace("[id]", id));
</script>

<a
    href={urlPath}
    class="variant-glass-surface hover:variant-soft-primary flex flex-row items-center justify-start rounded-md p-4 py-2 transition-all"
    class:!variant-glass-primary={page.route.id == route ||
        extraMatches.includes(page.route.id ?? "")}
    class:hover:!variant-outline-secondary={page.route.id == route ||
        extraMatches.includes(page.route.id ?? "")}
>
    <Icon {icon} height="24" class="mr-2" />
    {text}
</a>
