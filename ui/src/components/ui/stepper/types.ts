// Taken and modified from https://github.com/skeletonlabs/skeleton/blob/c96634a93dff4aa19340aae68f59261a096f682e/packages/skeleton/src/lib/components/Stepper/types.ts

import type { EventDispatcher } from "svelte";

export interface StepperState {
    current: number;
    total: number;
}

export type StepperButton = "submit" | "reset" | "button";

export type StepperEvent = {
    next: { step: number; state: StepperState };
    step: { step: number; state: StepperState };
    back: { step: number; state: StepperState };
    complete: { step: number; state: StepperState };
};

export type StepperEventDispatcher = EventDispatcher<StepperEvent>;
