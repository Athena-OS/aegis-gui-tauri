<script lang="ts">
  import { Link, useLocation } from "svelte-routing";
  import { createDialog } from "svelte-headlessui";

  import stepsConfig from "../../stepsConfig";

  import Button from "./Button.svelte";
  import Dialog from "./Dialog.svelte";

  export let prev: string = "";
  export let next: string = "";

  let dialog = createDialog({ label: "Are you sure?" });

  let location = useLocation();

  let index = stepsConfig.findIndex((x) => x.route === $location.pathname);

  const filteredRoutes = stepsConfig.filter((step) => !step.exclude);
</script>

<Dialog {dialog}>
  <div class="w-full h-full space-y-4 text-center">
    <h3 class="font-medium text-xl mx-4">Do you want to start the installation process ?</h3>
    <h3 class="font-medium text-xl mx-4 text-red-500">This can't be undone</h3>
    <div class="flex space-x-2 w-full">
      <Button on:click={dialog.close} fullWidth variant="bordered">Cancel</Button>
      <Link to={next} class="w-full">
        <Button fullWidth variant="primary">Install</Button>
      </Link>
    </div>
  </div>
</Dialog>

<div
  class="flex justify-between items-center absolute bottom-0 left-0 right-0 w-full p-6"
>
  <div class="w-40">
    <Link to={prev}>
      <Button fullWidth>Previous</Button></Link
    >
  </div>
  <div class="flex items-center space-x-2">
    {#each filteredRoutes as step, currentIndex}
      <div
        class=" rounded-full {currentIndex + 1 === index
          ? 'h-4 w-4 bg-primary-500'
          : 'h-2 w-2 bg-gray-500'}"
      ></div>
    {/each}
  </div>
  <div
    class="flex flex-col items-center justify-center space-y-2 w-40 relative"
  >
    <div class="text-primary-500 font-medium absolute bottom-14">
      Step {index} of 9
    </div>
    {#if next === "/install"}
        <Button fullWidth on:click={dialog.open}>Next</Button>
    {:else if next !== ""}
      <Link class="w-full" to={next}>
        <Button fullWidth>Next</Button>
      </Link>
    {:else}
      <button
        class="rounded-full text-black font-medium flex items-center justify-center space-x-2 px-4 py-2 text-lg h-[50px] w-full bg-primary-800"
        >Next</button
      >
    {/if}
  </div>
</div>
