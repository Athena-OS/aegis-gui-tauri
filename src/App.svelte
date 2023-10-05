<script lang="ts">
  import ProgressStepper from "./lib/ProgressStepper.svelte";
  import InitialView from "./lib/steps/Welcome.svelte";
  import DesktopView from "./lib/steps/Desktop.svelte";
  import KeyboardView from "./lib/steps/Keyboard.svelte";
  import PackageView from "./lib/steps/Packages.svelte";
  import PartitionView from "./lib/steps/Partition.svelte";
  import ManualPartition from "./lib/steps/ManualPartition.svelte";

  let view = "initial";
  const totalSteps = 3; // adjust based on your total steps

  function switchView(targetView: string): void {
    view = targetView;
  }

  let currentStep: number;
  $: if (view === "initial") {
    currentStep = 1;
  } else if (view === "keyboard") {
    currentStep = 2;
  } else if (view === "desktop") {
    currentStep = 3;
  } // add more conditions as you have more steps
</script>



{#if view === "initial"}
  <InitialView {switchView} />
{:else if view === "keyboard"}
  <KeyboardView {switchView} />
{:else if view === "desktop"}
  <DesktopView {switchView} />
{:else if view === "packages"}
  <PackageView {switchView} />
{:else if view === "partition"}
  <PartitionView {switchView} />
{:else if view === "manualPartition"}
  <ManualPartition {switchView} />
{/if} 
