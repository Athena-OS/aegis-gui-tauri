<script lang="ts">
  import Dialog from "./Dialog.svelte";
  import { createDialog } from "svelte-headlessui";
  import Navigation from "./Navigation.svelte";

  import infoIcon from "../../assets/icons/info-icon.svg";
  import { useLocation } from "svelte-routing";
  import stepsConfig from "../../stepsConfig";

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
  <div class="h-[calc(100%-160px)] pb-10 overflow-scroll">
    <slot />
  </div>
  <Navigation {prev} {next} />
</main>
