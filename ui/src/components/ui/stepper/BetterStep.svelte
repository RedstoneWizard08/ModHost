<!-- Taken and modified from https://github.com/skeletonlabs/skeleton/blob/c96634a93dff4aa19340aae68f59261a096f682e/packages/skeleton/src/lib/components/Stepper/Step.svelte -->
<!-- Reference: https://dribbble.com/shots/16221169-Figma-Material-Ui-components-Steppers-and-sliders -->

<script lang="ts">
    import { getContext, hasContext, onDestroy, onMount, type Snippet } from "svelte";
    import type { Writable } from "svelte/store";
    import { dynamicTransition } from "$lib/skeleton/transitions";

    // Types
    import type { StepperState } from "./types";
    import type { CssClasses, Transition, TransitionParams } from "@skeletonlabs/skeleton";
    import type { HTMLAttributes } from "svelte/elements";
    type TransitionIn = $$Generic<Transition>;
    type TransitionOut = $$Generic<Transition>;

    interface Props {
        locked?: boolean;
        /** Provide arbitrary classes to the step header region. */
        regionHeader?: CssClasses;
        /** Provide arbitrary classes to the step content region. */
        regionContent?: CssClasses;
        /** Provide arbitrary classes to the step navigation region. */
        regionNavigation?: CssClasses;
        state?: Writable<StepperState>;
        stepTerm?: string;
        gap?: CssClasses;
        justify?: CssClasses;
        onNext?: (locked: boolean, stepIndex: number) => void;
        onBack?: (stepIndex: number) => void;
        onComplete?: (stepIndex: number) => void;
        // buttonBack?: CssClasses;
        // buttonBackType?: "submit" | "reset" | "button";
        // buttonBackLabel?: string;
        // buttonNext?: CssClasses;
        // buttonNextType?: "submit" | "reset" | "button";
        // buttonNextLabel?: string;
        // buttonComplete?: CssClasses;
        // buttonCompleteType?: "submit" | "reset" | "button";
        // buttonCompleteLabel?: string;
        buttonBack?: Snippet<[boolean, () => void]>;
        buttonNext?: Snippet<[boolean, () => void]>;
        buttonComplete?: Snippet<[boolean, () => void]>;
        /** Enable/Disable transitions */
        transitions?: boolean;
        /** Provide the transition to used on entry. */
        transitionIn?: TransitionIn;
        /** Transition params provided to `transitionIn`. */
        transitionInParams?: TransitionParams<TransitionIn>;
        /** Provide the transition to used on exit. */
        transitionOut?: TransitionOut;
        /** Transition params provided to `transitionOut`. */
        transitionOutParams?: TransitionParams<TransitionOut>;
        header?: Snippet;
        navigation?: Snippet;
    }

    let {
        locked = false,
        regionHeader = "",
        regionContent = "",
        regionNavigation = "",
        state = getContext("state")!,
        stepTerm = getContext("stepTerm")!,
        gap = getContext("gap")!,
        justify = getContext("justify")!,
        onNext = getContext("onNext")!,
        onBack = getContext("onBack")!,
        onComplete = getContext("onComplete")!,
        // buttonBack = getContext("buttonBack")!,
        // buttonBackType = getContext("buttonBackType")!,
        // buttonBackLabel = getContext("buttonBackLabel")!,
        // buttonNext = getContext("buttonNext")!,
        // buttonNextType = getContext("buttonNextType")!,
        // buttonNextLabel = getContext("buttonNextLabel")!,
        // buttonComplete = getContext("buttonComplete")!,
        // buttonCompleteType = getContext("buttonCompleteType")!,
        // buttonCompleteLabel = getContext("buttonCompleteLabel")!,
        buttonBack = getContext("buttonBack")!,
        buttonNext = getContext("buttonNext")!,
        buttonComplete = getContext("buttonComplete")!,
        transitions = getContext("transitions")!,
        transitionIn = getContext("transitionIn")!,
        transitionInParams = getContext("transitionInParams")!,
        transitionOut = getContext("transitionOut")!,
        transitionOutParams = getContext("transitionOutParams")!,
        header,
        children,
        navigation,
        ...restProps
    }: Props & HTMLAttributes<HTMLElement> = $props();

    // Register step on init (keep these paired)
    const stepIndex = $state.total;

    $state.total++;

    // Classes
    const cBase = "space-y-4";
    const cHeader = "text-2xl font-bold";
    const cContent = "space-y-4";
    const cNavigation = "flex";

    // Reactive
    const classesBase = $derived(`${cBase} ${restProps.class ?? ""}`);
    const classesHeader = $derived(`${cHeader} ${regionHeader}`);
    const classesContent = $derived(`${cContent} ${regionContent}`);
    const classesNavigation = $derived(`${cNavigation} ${justify} ${gap} ${regionNavigation}`);

    // Unregister step on destroy
    onDestroy(() => {
        $state.total--;
    });
</script>

{#if stepIndex === $state.current}
    <div class="step {classesBase}" data-testid="step">
        <!-- Slot: Header -->
        <header class="step-header {classesHeader}">
            {#if header}
                {@render header()}
            {:else}
                {stepTerm} {stepIndex + 1}
            {/if}
        </header>

        <!-- Slot: Default -->
        <div class="step-content {classesContent}">
            {#if children}
                {@render children()}
            {:else}
                ({stepTerm} {stepIndex + 1} Content)
            {/if}
        </div>

        <!-- Navigation -->
        {#if $state.total > 1}
            <div
                class="step-navigation {classesNavigation}"
                in:dynamicTransition|local={{
                    transition: transitionIn,
                    params: transitionInParams,
                    enabled: transitions,
                }}
                out:dynamicTransition|local={{
                    transition: transitionOut,
                    params: transitionOutParams,
                    enabled: transitions,
                }}
            >
                {#if stepIndex === 0 && navigation}
                    <!-- Slot: Navigation -->
                    <div class="step-navigation-slot">
                        {@render navigation()}
                    </div>
                {:else}
                    <!-- Button: Back -->
                    <!-- <button
                        type={buttonBackType}
                        class="btn {buttonBack}"
                        onclick={() => onBack(stepIndex)}
                        disabled={$state.current === 0}
                    >
                        {@html buttonBackLabel}
                    </button> -->
                    {@render buttonBack($state.current === 0, () => onBack(stepIndex))}
                {/if}

                {#if stepIndex < $state.total - 1}
                    <!-- Button: Next -->
                    <!-- <button
                        type={buttonNextType}
                        class="btn {buttonNext}"
                        onclick={() => onNext(locked, stepIndex)}
                        disabled={locked}
                    >
                        {#if locked}
                            <svg
                                class="aspect-square w-3 fill-current"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 448 512"
                            >
                                <path
                                    d="M144 144v48H304V144c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192V144C80 64.5 144.5 0 224 0s144 64.5 144 144v48h16c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V256c0-35.3 28.7-64 64-64H80z"
                                />
                            </svg>
                        {/if}

                        <span>{@html buttonNextLabel}</span>
                    </button> -->
                    {@render buttonNext(locked, () => onNext(locked, stepIndex))}
                {:else}
                    <!-- Button: Complete -->
                    <!-- <button
                        type={buttonCompleteType}
                        class="btn {buttonComplete}"
                        onclick={() => onComplete(stepIndex)}
                        disabled={locked}
                    >
                        {@html buttonCompleteLabel}
                    </button> -->
                    {@render buttonComplete(locked, () => onComplete(stepIndex))}
                {/if}
            </div>
        {/if}
    </div>
{/if}
