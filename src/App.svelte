<script lang="ts">
  import { fade } from "svelte/transition";
  import ProgressStepper from "./lib/components/ProgressStepper.svelte";
  import steps from "./lib/stepsConfig";

  let currentActive = 0;
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@2.36.0/tabler-icons.min.css"
  />
</svelte:head>

{#each steps as step}
  {#key step.view === steps[currentActive].view}
    <div
      in:fade={{ delay: 200, duration: 200 }}
      out:fade={{ duration: 200 }}
      class="absolute top-0 left-0 right-0"
    >
      {#if step.view === steps[currentActive].view}
        <svelte:component this={steps[currentActive].component} />
      {/if}
    </div>
  {/key}
{/each}
<ProgressStepper bind:currentActive />
