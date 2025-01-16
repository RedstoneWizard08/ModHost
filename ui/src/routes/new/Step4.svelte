<script lang="ts">
    import BetterStep from "$components/ui/stepper/BetterStep.svelte";
    import Icon from "@iconify/svelte";
    import { Carta, MarkdownEditor } from "carta-md";

    interface Props {
        editor: Carta;
        readme: string;
    }

    let { editor, readme = $bindable() }: Props = $props();
</script>

{#snippet header()}
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:book" height="24" class="mr-2" />
        Readme
    </p>
{/snippet}

<BetterStep {header}>
    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:file-description" height="24" class="mr-2" />
            Description
        </p>

        <div class="card variant-soft-secondary w-full p-4">
            <MarkdownEditor carta={editor} bind:value={readme} mode="tabs" />
        </div>
    </div>
</BetterStep>
