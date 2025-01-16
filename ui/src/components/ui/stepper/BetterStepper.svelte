<!-- Taken and modified from https://github.com/skeletonlabs/skeleton/blob/c96634a93dff4aa19340aae68f59261a096f682e/packages/skeleton/src/lib/components/Stepper/Stepper.svelte -->

<script lang="ts" module>
    import { fade } from "svelte/transition";
    import {
        type Transition,
        type TransitionParams,
        prefersReducedMotionStore,
    } from "@skeletonlabs/skeleton";

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    type FadeTransition = typeof fade;
    type TransitionIn = Transition;
    type TransitionOut = Transition;
</script>

<script
    lang="ts"
    generics="TransitionIn extends Transition = FadeTransition, TransitionOut extends Transition = FadeTransition"
>
    import { setContext, type Snippet } from "svelte";
    import { type Writable, writable } from "svelte/store";
    import { dynamicTransition } from "$lib/skeleton/transitions";

    // Types
    import type { CssClasses } from "@skeletonlabs/skeleton";
    import type { HTMLAttributes } from "svelte/elements";
    import type { StepperButton, StepperEvent, StepperState } from "./types";

    interface Props {
        /** Provide classes to style the stepper header gap. */
        gap?: CssClasses;
        /** Provide the verbiage that represents "Step". */
        stepTerm?: string;
        /** Provide classes to style the stepper header badges. */
        badge?: CssClasses;
        /** Provide classes to style the stepper header active step badge. */
        active?: CssClasses;
        /** Provide classes to style the stepper header border. */
        border?: CssClasses;
        /** Provide the initially selected step*/
        start?: number;
        /** Set the justification for the step navigation buttons. */
        justify?: CssClasses;
        // /** Provide arbitrary classes to style the back button. */
        // buttonBack?: CssClasses;
        // /** Set the type of the back button. */
        // buttonBackType?: StepperButton;
        // /** Provide the HTML label content for the back button. */
        // buttonBackLabel?: string;
        // /** Provide arbitrary classes to style the next button. */
        // buttonNext?: CssClasses;
        // /** Set the type of the next button. */
        // buttonNextType?: StepperButton;
        // /** Provide the HTML label content for the next button. */
        // buttonNextLabel?: string;
        // /** Provide arbitrary classes to style the complete button. */
        // buttonComplete?: CssClasses;
        // /** Set the type of the complete button. */
        // buttonCompleteType?: StepperButton;
        // /** Provide the HTML label content for the complete button. */
        // buttonCompleteLabel?: string;
        buttonBack?: Snippet<[boolean, () => void]>;
        buttonNext?: Snippet<[boolean, () => void]>;
        buttonComplete?: Snippet<[boolean, () => void]>;
        /** Provide arbitrary classes to the stepper header region. */
        regionHeader?: CssClasses;
        /** Provide arbitrary classes to the stepper content region. */
        regionContent?: CssClasses;
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
        /** Fires when the NEXT button is pressed per step. */
        next?: (event: StepperEvent["next"]) => any;
        /** Fires when a next/previous step occurs. */
        step?: (event: StepperEvent["step"]) => any;
        /** Fires when the BACK button is pressed per step. */
        back?: (event: StepperEvent["back"]) => any;
        /** Fires when the COMPLETE button is pressed. */
        complete?: (event: StepperEvent["complete"]) => any;
    }

    const {
        children,
        gap = "gap-4",
        stepTerm = "Step",
        badge = "variant-filled-surface",
        active = "variant-filled",
        border = "border-surface-400-500-token",
        start = 0,
        justify = "justify-between",
        // buttonBack = "veriant-ghost",
        // buttonBackType = "button",
        // buttonBackLabel = "&larr; Back",
        // buttonNext = "variant-filled",
        // buttonNextType = "button",
        // buttonNextLabel = "Next &rarr;",
        // buttonComplete = "variant-filled-primary",
        // buttonCompleteType = "button",
        // buttonCompleteLabel = "Complete",
        buttonBack = defaultButtonBack,
        buttonNext = defaultButtonNext,
        buttonComplete = defaultButtonComplete,
        regionHeader = "",
        regionContent = "",
        transitions = !$prefersReducedMotionStore,
        transitionIn = fade as TransitionIn,
        transitionInParams = { duration: 100 },
        transitionOut = fade as TransitionOut,
        transitionOutParams = { duration: 100 },
        next,
        step,
        back,
        complete,
        ...restProps
    }: Props & HTMLAttributes<HTMLElement> = $props();

    // Stores
    let stateStore: Writable<StepperState> = writable({ current: start, total: 0 });

    // Event Handlers
    async function onNext(locked: boolean, stepIndex: number) {
        // Allows any forms to submit before the Step is removed from the DOM:
        // https://github.com/skeletonlabs/skeleton/issues/1328
        await new Promise((resolve) => setTimeout(resolve));

        if (locked) return;
        $stateStore.current++;
        next?.({ step: stepIndex, state: $stateStore });
        step?.({ step: stepIndex, state: $stateStore });
    }

    function onBack(stepIndex: number) {
        $stateStore.current--;
        back?.({ step: stepIndex, state: $stateStore });
        step?.({ step: stepIndex, state: $stateStore });
    }

    function onComplete(stepIndex: number) {
        complete?.({ step: stepIndex, state: $stateStore });
    }

    function setupContext() {
        // Context
        setContext("state", stateStore);
        setContext("stepTerm", stepTerm);
        setContext("gap", gap);
        setContext("justify", justify);
        // ---
        setContext("onNext", onNext);
        setContext("onBack", onBack);
        setContext("onComplete", onComplete);
        // ---
        // setContext("buttonBack", buttonBack);
        // setContext("buttonBackType", buttonBackType);
        // setContext("buttonBackLabel", buttonBackLabel);
        // // ---
        // setContext("buttonNext", buttonNext);
        // setContext("buttonNextType", buttonNextType);
        // setContext("buttonNextLabel", buttonNextLabel);
        // // ---
        // setContext("buttonComplete", buttonComplete);
        // setContext("buttonCompleteType", buttonCompleteType);
        // setContext("buttonCompleteLabel", buttonCompleteLabel);
        setContext("buttonBack", buttonBack);
        setContext("buttonNext", buttonNext);
        setContext("buttonComplete", buttonComplete);
        // ---
        setContext("transitions", transitions);
        setContext("transitionIn", transitionIn);
        setContext("transitionInParams", transitionInParams);
        setContext("transitionOut", transitionOut);
        setContext("transitionOutParams", transitionOutParams);
    }

    setupContext();

    // Classes
    const cBase = "space-y-4";
    const cHeader = "flex items-center border-t mt-[15px]";
    const cHeaderStep = "-mt-[15px] transition-all duration-300";
    const cContent = "";

    // Reactive
    const classesBase = $derived(`${cBase} ${restProps.class ?? ""}`);
    const classesHeader = $derived(`${cHeader} ${border} ${gap} ${regionHeader}`);
    const classesHeaderStep = $derived(`${cHeaderStep}`);
    const classesContent = $derived(`${cContent} ${regionContent}`);
</script>

{#snippet defaultButtonBack(locked: boolean, clickHandler: () => void)}
    <button type="button" class="btn variant-ghost" onclick={clickHandler} disabled={locked}>
        &larr; Back
    </button>
{/snippet}

{#snippet defaultButtonNext(locked: boolean, clickHandler: () => void)}
    <button type="button" class="btn variant-filled" onclick={clickHandler} disabled={locked}>
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

        <span>Next &rarr;</span>
    </button>
{/snippet}

{#snippet defaultButtonComplete(locked: boolean, clickHandler: () => void)}
    <button
        type="button"
        class="btn variant-filled-primary"
        onclick={clickHandler}
        disabled={locked}
    >
        Complete
    </button>
{/snippet}

<div class="stepper {classesBase}" data-testid="stepper">
    <!-- Header -->
    {#if $stateStore.total}
        <header
            class="stepper-header {classesHeader}"
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
            {#each Array.from(Array($stateStore.total).keys()) as step}
                <div
                    class="stepper-header-step {classesHeaderStep}"
                    class:flex-1={step === $stateStore.current}
                >
                    <span class="badge {step === $stateStore.current ? active : badge}"
                        >{step === $stateStore.current ? `${stepTerm} ${step + 1}` : step + 1}</span
                    >
                </div>
            {/each}
        </header>
    {/if}

    <!-- Content -->
    <div class="stepper-content {classesContent}">
        {@render children?.()}
    </div>
</div>
