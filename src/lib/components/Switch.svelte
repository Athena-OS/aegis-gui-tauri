<script lang="ts">
  import { createSwitch } from "svelte-headlessui";

  const sw = createSwitch({ label: "Set Preference" });

  export let value: boolean = false;

  $: $sw.checked, onClick();
  function onClick() {
    value = $sw.checked;
  }
</script>

<button
  class="relative inline-flex h-7 w-12 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 {$sw.checked
    ? 'bg-primary-500'
    : 'bg-neutral-500'}"
  use:sw.toggle
>
  <span class="sr-only">Use setting</span>
  <span
    aria-hidden="true"
    class="{$sw.checked
      ? 'translate-x-5'
      : 'translate-x-0'} pointer-events-none inline-block h-6 w-6 transform rounded-full bg-neutral-800 shadow-lg ring-0 transition duration-200 ease-in-out"
  />
</button>
