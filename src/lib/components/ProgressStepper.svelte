<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { SwitchEvent } from "../types";
  import { stepsConfig } from "../stepsConfig";
  import Button from "./Button.svelte";

  export let currentStep: number = 1;

  const dispatch = createEventDispatcher();

  function switchView(targetView: string): void {
    const eventDetail: SwitchEvent = { targetView };
    dispatch("switch", eventDetail);
  }

  function goToNextView(): void {
    if (currentStep < stepsConfig.length) {
      switchView(stepsConfig[currentStep].view);
    }
  }
</script>

<div class="fixed bottom-12 w-full flex items-center justify-center">
  <!-- Centered stepper dots -->
  <div class="flex justify-center items-center space-x-1.5 z-10">
    {#each Array(stepsConfig.length) as _, index}
      <div
        class={`rounded-full ${
          index + 1 === currentStep
            ? "h-3.5 w-3.5 bg-yellow-500"
            : "h-2.5 w-2.5 bg-gray-600"
        }`}
      />
    {/each}
  </div>

  <!-- Right side content -->
  <div class="absolute inset-0 mb-24 flex items-center justify-end mr-8">
    <div class="flex flex-col items-center">
      <div class="text-yellow-500 whitespace-nowrap mb-1 text-lg font-semibold">
        Step {currentStep} of {stepsConfig.length}
      </div>
      <div class="text-right whitespace-nowrap">
        <Button variant="primary" on:click={goToNextView}
          ><span>Continue</span><span><i class="ti ti-chevron-right" /></span
          ></Button
        >
      </div>
    </div>
  </div>
</div>
