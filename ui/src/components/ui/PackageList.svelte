<script lang="ts">
    import PackageCard from "./PackageCard.svelte";
    import type { SearchResults } from "@modhost/api";

    interface Props {
        customHeight?: number;
        compact?: boolean;
        showAvatar?: boolean;
        showDetails?: boolean;
        showName?: boolean;
        select?: (id: number) => void | Promise<void>;
        packages: SearchResults;
    }

    const {
        customHeight,
        compact = false,
        showName = true,
        showDetails = true,
        showAvatar = true,
        select,
        packages,
    }: Props = $props();
</script>

{#each packages.results as pkg, _ (pkg)}
    <PackageCard {pkg} {customHeight} {compact} {showName} {showDetails} {showAvatar} {select} />
{/each}
