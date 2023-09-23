<script lang="ts">
  import ProgressStepper from './lib/ProgressStepper.svelte';
  import InitialView from './lib/Welcome.svelte';
  import DesktopView from './lib/Desktop.svelte';

  let view = "initial";
  const totalSteps = 3; // adjust based on your total steps

  function switchView(targetView: string): void {
    view = targetView;
  }

  let currentStep: number;
  $: if (view === 'initial') {
    currentStep = 1;
  } else if (view === 'desktop') {
    currentStep = 2;
  } // add more conditions as you have more steps
</script>

<ProgressStepper steps={totalSteps} currentStep={currentStep} />

{#if view === "initial"}
  <InitialView {switchView} />
{:else if view === "desktop"}
  <DesktopView {switchView} />
{/if}
