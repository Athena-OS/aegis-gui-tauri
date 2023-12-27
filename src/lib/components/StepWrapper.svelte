<script lang="ts">
  import { createDialog } from "svelte-headlessui";
  import { useLocation } from "svelte-routing";

  import stepsConfig from "../../stepsConfig";

  import Dialog from "./Dialog.svelte";
  import Navigation from "./Navigation.svelte";

  import athenaLogo from "../../assets/athena-logo.svg";
  import infoIcon from "../../assets/icons/info-icon.svg";

  export let title = "";
  export let dialogTitle = "";
  export let dialogContent = "";

  export let prev = "";
  export let next = "";

  let dialog = createDialog({ label: dialogTitle });

  let location = useLocation();

  let index = stepsConfig.findIndex((x) => x.route === $location.pathname);
</script>

<Dialog {dialog} {dialogTitle}>
  <p class="text-sm text-neutral-500">
    {dialogContent}
  </p></Dialog
>
<main
  class="h-full p-4 space-y-4 absolute top-0 left-0 right-0 overflow-scroll"
>
  <div class="w-full flex flex-col items-center justify-center">
    <div class="flex w-full bg-blue justify-end px-4 py-2 -mb-8">
      <div class="flex space-x-2 items-center justify-center">
        <img src={athenaLogo} class="w-10" alt="">
        <h3 class="text-xl font-medium">Athena OS</h3>
      </div>
    </div>
    <div class="flex flex-row items-center mx-auto w-fit space-x-4">
      <h2 class="text-center py-2">{index}. {title}</h2>
      <button
        on:click={dialog.open}
        aria-label="Open Dialog"
        class="bg-transparent border-none focus:outline-none"
      >
        <img class="w-8" src={infoIcon} alt="" />
      </button>
    </div>
  </div>
  <div class="h-[calc(100%-160px)] w-full pb-10 overflow-scroll">
    <slot />
  </div>
  <Navigation {prev} {next} />
</main>
