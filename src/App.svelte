<script lang="ts">
  import { fade } from "svelte/transition";
  import ProgressStepper from "./lib/components/ProgressStepper.svelte";
  import steps from "./lib/stepsConfig";
  import currentActive from "./lib/stepsStore";
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@2.36.0/tabler-icons.min.css"
  />
</svelte:head>

<main class="relative h-screen">
  {#each steps as step}
    {#key step.view === steps[$currentActive].view}
      <div in:fade={{ delay: 200, duration: 200 }} out:fade={{ duration: 200 }}>
        {#if step.view === steps[$currentActive].view}
          <svelte:component this={steps[$currentActive].component} />
        {/if}
      </div>
    {/key}
  {/each}
  {#if steps[$currentActive].view !== "initial"}
    <div class="absolute bottom-0 w-full" transition:fade>
      <ProgressStepper />
    </div>
  {/if}
</main>
