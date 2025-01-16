<script lang="ts">
    import BetterStep from "$components/ui/stepper/BetterStep.svelte";
    import Icon from "@iconify/svelte";

    interface Props {
        repo: string;
        issues: string;
        wiki: string;
    }

    let { repo = $bindable(), issues = $bindable(), wiki = $bindable() }: Props = $props();
</script>

{#snippet header()}
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:link" height="24" class="mr-2" />
        Links
    </p>
{/snippet}

<BetterStep {header}>
    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:code" height="24" class="mr-2" />
            Source Code
        </p>

        <input
            type="text"
            placeholder="Example: https://github.com/example/example"
            class="input rounded-md"
            bind:value={repo}
        />
    </div>

    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:exclamation-circle" height="24" class="mr-2" />
            Issue Tracker
        </p>

        <input
            type="text"
            placeholder="Example: https://github.com/example/example/issues"
            class="input rounded-md"
            bind:value={issues}
        />
    </div>

    <div class="card variant-soft-secondary w-full p-4">
        <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
            <Icon icon="tabler:world" height="24" class="mr-2" />
            Wiki
        </p>

        <input
            type="text"
            placeholder="Example: https://github.com/example/example/wiki"
            class="input rounded-md"
            bind:value={wiki}
        />
    </div>
</BetterStep>
