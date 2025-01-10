<script lang="ts">
    import { goto } from "$app/navigation";
    import { base } from "$app/paths";
    import AuthRequired from "$components/auth/AuthRequired.svelte";
    import { user } from "$lib/user";

    let renderErrPage = false;

    setTimeout(() => {
        renderErrPage = true;
    }, 100);

    user.subscribe((user) => {
        if (user) {
            goto(`${base}/u/${user.username}`);
        }
    });
</script>

{#if renderErrPage}
    <AuthRequired />
{/if}
