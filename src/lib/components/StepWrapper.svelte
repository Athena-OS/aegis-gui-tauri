<script lang="ts">
  import currentActive from "../stepsStore";
  import Dialog from "./Dialog.svelte";
  import { createDialog } from "svelte-headlessui";

  export let title = "";
  export let dialogTitle = "";
  export let dialogContent = "";

  let dialog = createDialog({ label: dialogTitle });
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
    <h2 class="text-center py-2">{$currentActive}. {title}</h2>
    <button
      on:click={dialog.open}
      aria-label="Open Dialog"
      class="bg-transparent border-none focus:outline-none"
    >
      <i class="ti text-accent-500 !text-3xl ti-info-hexagon-filled" />
    </button>
  </div>
  <div class="h-[calc(100%-140px)]">
    <slot />
  </div>
</main>
