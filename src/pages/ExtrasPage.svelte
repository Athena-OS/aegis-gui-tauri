<script lang="ts">
  import { createDialog } from "svelte-headlessui";
  import kernelIcon from "../assets/icons/kernel-yellow.svg";
  import terminalIcon from "../assets/icons/terminal-yellow.svg";
  import wrenchIcon from "../assets/icons/wrench-yellow.svg";
  import Dialog from "../lib/components/Dialog.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";
  import Switch from "../lib/components/Switch.svelte";

  let dialogKernal = createDialog({ label: "Kernal" });
  let dialogTerminal = createDialog({ label: "Terminal & Shells" });
  let dialogSystem = createDialog({ label: "System" });
</script>

<Dialog dialog={dialogKernal} dialogTitle="Kernal">
  <p class="text-sm text-neutral-500">Hello There</p></Dialog
>

<Dialog dialog={dialogTerminal} dialogTitle="Terminal & Shells">
  <p class="text-sm text-neutral-500">Hello There</p></Dialog
>

<Dialog dialog={dialogSystem} dialogTitle="Enable/Disable System Settings">
  <ul class=" divide-y divide-neutral-700 mt-4">
    <li>
      <span>Setting 1</span>
      <Switch />
    </li>
    <li>
      <span>Setting 2</span>
      <Switch />
    </li>
    <li>
      <span>Setting 3</span>
      <Switch />
    </li>
    <li>
      <span>Setting 4</span>
      <Switch />
    </li>
    <li>
      <span>Setting 5</span>
      <Switch />
    </li>
  </ul>
</Dialog>

<StepWrapper
  title="Extras"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="accounts"
  next="summary"
>
  <div class="flex flex-col items-center">
    <div class="flex flex-row flex-wrap gap-10 max-w-5xl">
      <button
        on:click={dialogKernal.open}
        class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer"
      >
        <img src={kernelIcon} alt="kernel" />
        <h3 class="font-semibold text-4xl">Kernel</h3>
      </button>
      <button
        on:click={dialogTerminal.open}
        class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer"
      >
        <img src={terminalIcon} alt="kernel" />
        <div>
          <h3 class="font-semibold text-4xl">Terminal</h3>
          <span class="font-semibold text-xl">& Shell</span>
        </div>
      </button>
      <button
        on:click={dialogSystem.open}
        class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer"
      >
        <img src={wrenchIcon} alt="kernel" />
        <h3 class="font-semibold text-4xl">System</h3>
      </button>
    </div>
  </div>
</StepWrapper>

<style>
  li {
    @apply flex items-center justify-between py-2;
  }
</style>
