<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { currentProject, editSaving } from "$lib/state";
    import Icon from "@iconify/svelte";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { unwrap, unwrapOrNull, type User } from "@modhost/api";
    import { client } from "$lib/api";

    const id = $derived($page.params.id);
    const toasts = getToastStore();

    let person = $state("");
    let checking = $state(false);
    let authors = $state<User[]>([]);
    let displayAuthors = $state<User[]>([]);
    let newAuthors = $state<number[]>([]);
    let removedAuthors = $state<number[]>([]);

    onMount(() => {
        if (!$currentProject) return;

        authors = [...$currentProject.authors];
        displayAuthors = [...authors];
    });

    const save = async () => {
        $editSaving = true;

        let newPkg = $currentProject;

        for (const user of $currentProject?.authors ?? []) {
            if (!authors.find((v) => v.id == user.id)) {
                newPkg = unwrap(await client.project(id).authors().remove(user.id));
            }
        }

        for (const user of authors) {
            if (!newPkg?.authors.find((v) => v.id == user.id)) {
                newPkg = unwrap(await client.project(id).authors().add(user.id));
            }
        }

        $currentProject = unwrap(await client.project(id).get());

        authors = $currentProject?.authors ?? authors;
        newAuthors = [];
        removedAuthors = [];
        displayAuthors = [...authors];

        $editSaving = false;
    };

    const removeMember = (userId: number) => {
        return async (ev: Event) => {
            ev.preventDefault();
            ev.stopPropagation();

            if ($editSaving) {
                toasts.trigger({
                    message: `Saving in progress, try again later.`,
                    hideDismiss: true,
                    timeout: 5000,
                    background: "variant-filled-error",
                });

                return;
            }

            if (authors.length <= 1) {
                toasts.trigger({
                    message: `A project must have one or more members!`,
                    hideDismiss: true,
                    timeout: 5000,
                    background: "variant-filled-error",
                });

                return;
            }

            authors = authors.filter((v) => v.id != userId);
            removedAuthors.push(userId);

            if (newAuthors.includes(userId)) {
                newAuthors = newAuthors.filter((v) => v != userId);
                displayAuthors = displayAuthors.filter((v) => v.id != userId);
            }
        };
    };

    const addPerson = async () => {
        if ($editSaving) {
            toasts.trigger({
                message: `Saving in progress, try again later.`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        if (checking) {
            toasts.trigger({
                message: `A user is already being fetched, try again later.`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        checking = true;

        try {
            const user = unwrapOrNull(await client.user(person).get());

            // trigger the catch block
            if (!user) throw new Error("No user found!");

            authors.push(user);
            newAuthors.push(user.id);

            if (!displayAuthors.find((v) => v.id == user.id)) displayAuthors.push(user);

            if (removedAuthors.includes(user.id)) {
                removedAuthors = removedAuthors.filter((v) => v != user.id);
            }
        } catch (_err) {
            toasts.trigger({
                message: `Could not find user with name ${person}!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });
        }

        person = "";
        checking = false;
    };
</script>

<p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
    <Icon icon="tabler:users" height="24" class="mr-2" />
    Manage Members
</p>

<div class="card variant-glass-surface w-full space-y-2 p-4">
    {#each displayAuthors as author}
        <a
            class="card hover:variant-soft-primary flex flex-row items-center justify-between p-2"
            href="/u/{author.username}"
        >
            <div
                class="flex flex-row items-center justify-start"
                class:!text-success-500={newAuthors.includes(author.id)}
                class:!font-bold={newAuthors.includes(author.id) ||
                    removedAuthors.includes(author.id)}
                class:!line-through={removedAuthors.includes(author.id)}
                class:!text-error-500={removedAuthors.includes(author.id)}
            >
                {#if author.github_id == -1}
                    <img
                        src="/modhost.png"
                        alt="author's profile afirst child cssvatar"
                        class="rounded-token my-auto mr-2 aspect-square h-8"
                    />
                {:else}
                    <img
                        src="https://avatars.githubusercontent.com/u/{author.github_id}"
                        alt="author's profile afirst child cssvatar"
                        class="rounded-token my-auto mr-2 aspect-square h-8"
                    />
                {/if}
                {author.username}
            </div>

            <button
                type="button"
                class="variant-soft-error btn hover:variant-filled-error transition-all"
                onclick={removeMember(author.id)}
            >
                <Icon icon="tabler:trash" height="24" />
            </button>
        </a>
    {/each}
</div>

<div class="card variant-glass-surface w-full space-y-2 p-4">
    <p class="text-primary-500 mb-2 flex flex-row items-center justify-start">
        <Icon icon="tabler:plus" height="24" class="mr-2" />
        Add People
    </p>

    <input
        type="text"
        placeholder="Type a username..."
        class="input rounded-md"
        bind:value={person}
        disabled={checking}
    />

    <button
        type="button"
        class="variant-ghost-secondary btn hover:variant-soft-primary flex flex-row items-center justify-center rounded-lg transition-all"
        onclick={addPerson}
        disabled={checking}
    >
        {#if checking}
            <Icon icon="tabler:loader-2" height="24" class="mr-2 animate-spin" />
        {:else}
            <Icon icon="tabler:plus" height="24" class="mr-2" />
        {/if}

        Add
    </button>
</div>

<button
    type="button"
    class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg"
    onclick={save}
>
    <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
    Save
</button>
