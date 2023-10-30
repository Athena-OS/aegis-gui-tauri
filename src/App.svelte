<script lang="ts">
  import ProgressStepper from "./lib/components/ProgressStepper.svelte";
  import { stepsConfig } from "./lib/stepsConfig";
  import type { SwitchEvent } from "./lib/types";

  let currentComponent = stepsConfig[0].component;
  let currentStep = 1;

  function handleSwitch(event: CustomEvent<SwitchEvent>): void {
    const targetView = event.detail.targetView;
    const stepIndex = stepsConfig.findIndex(step => step.view === targetView);
    currentStep = stepIndex + 1;
    currentComponent = stepsConfig[stepIndex].component;
  }
</script>

{#if currentStep === 1}
    <svelte:component this={currentComponent} on:switch={handleSwitch} />
{:else}
<svelte:component this={currentComponent} />
    <ProgressStepper bind:currentStep={currentStep} on:switch={handleSwitch} />
{/if}
