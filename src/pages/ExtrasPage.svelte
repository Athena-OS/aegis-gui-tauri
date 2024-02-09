<script lang="ts">
  import { createDialog } from "svelte-headlessui";

  import kernelIcon from "../assets/icons/kernel-yellow.svg";
  import terminalIcon from "../assets/icons/terminal-yellow.svg";
  import wrenchIcon from "../assets/icons/wrench-yellow.svg";

  import Dialog from "../lib/components/Dialog.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";
  import Switch from "../lib/components/Switch.svelte";
  import Dropdown from "../lib/components/Dropdown.svelte";
  import extrasStore from "../lib/stores/extrasStore";
  import InputBox from "../lib/components/InputBox.svelte";

  let kernelList = [
    { name: "Linux" },
    { name: "Linux LTS" },
    { name: "Linux Zen" },
    { name: "Linux Hardened" },
    { name: "Linux Real-Time" },
    { name: "Linux Real-Time LTS" },
    { name: "Linux Liquorix" },
    { name: "Linux Xanmod" },
  ];
  let terminalList = [
    { name: "Alacritty" },
    { name: "Cool Retro Term" },
    { name: "Foot" },
    { name: "GNOME Terminal" },
    { name: "Kitty" },
    { name: "Konsole" },
    { name: "Terminator" },
    { name: "Terminology" },
    { name: "Urxvt" },
    { name: "Xfce" },
    { name: "Xterm" },
  ];
  let shellsList = [{ name: "Bash" }, { name: "Fish" }, { name: "Zsh" }];

  function onChangeFunctionMaxJobs(e: any) {
    let value = parseInt(e.target.value);
    if (value >= 1) {
      e.target.parentElement.classList.remove("border-red-500");
      $extrasStore.maxjobs = value;
    } else {
      e.target.parentElement.classList.add("border-red-500");
    }
  }
  function onChangeFunctionCores(e: any) {
    let value = parseInt(e.target.value);
    if (value >= 0) {
      e.target.parentElement.classList.remove("border-red-500");
      $extrasStore.cores = value;
    } else {
      e.target.parentElement.classList.add("border-red-500");
    }
  }
</script>

<StepWrapper
  title="Extras"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/accounts"
  next={$extrasStore.kernel !== "default" &&
  $extrasStore.terminal !== "default" &&
  $extrasStore.shell !== "default"
    ? "/summary"
    : ""}
>
  <div class="flex w-full h-full space-x-4">
    <div class="w-1/3 h-full space-y-4">
      <div
        class="w-full flex flex-col min:h-1/3 bg-[#1A1A1A] px-8 pt-4 pb-8 space-y-4 rounded-2xl"
      >
        <div class="flex space-x-3 w-full justify-center items-center">
          <img src={kernelIcon} alt="kernel" />
          <h3 class="font-semibold text-4xl">Kernel</h3>
        </div>
        <Dropdown
          bind:items={kernelList}
          label="Available Kernel"
          on:select={(event) =>
            ($extrasStore.kernel = event.detail.selected.name)}
          defaultItem={{ name: "Select Kernel" }}
        />
      </div>

      <div
        class="w-full flex flex-col min:h-1/3 bg-[#1A1A1A] px-8 pt-4 pb-8 space-y-4 rounded-2xl"
      >
        <div class="flex space-x-3 w-full justify-center items-center">
          <img src={terminalIcon} alt="kernel" />
          <div>
            <h3 class="font-semibold text-4xl">Terminal</h3>
            <span class="font-semibold text-xl">& Shell</span>
          </div>
        </div>
        <Dropdown
          bind:items={terminalList}
          label="Available Terminals"
          on:select={(event) =>
            ($extrasStore.terminal = event.detail.selected.name)}
          defaultItem={{ name: "Select Terminal" }}
        />
        <Dropdown
          bind:items={shellsList}
          label="Available Shells"
          on:select={(event) =>
            ($extrasStore.shell = event.detail.selected.name)}
          defaultItem={{ name: "Select Shell" }}
        />
      </div>
    </div>
    <div class="h-full w-1/3 space-y-4">
      <div
        class="w-full h-fit flex flex-col bg-[#1A1A1A] px-8 pt-4 pb-8 space-y-4 rounded-2xl"
      >
        <div class="flex space-x-3 w-full justify-center items-center">
          <img src={wrenchIcon} alt="kernel" />
          <h3 class="font-semibold text-4xl">System</h3>
        </div>
        <div class="flex w-full justify-between items-center">
          <h4 class="text-xl font-medium">Snapper</h4>
          <Switch bind:value={$extrasStore.snapper}></Switch>
        </div>
        <div class="w-full h-[2px] bg-neutral-700"></div>
        <div class="flex w-full justify-between items-center">
          <h4 class="text-xl font-medium">Zram</h4>
          <Switch bind:value={$extrasStore.zram}></Switch>
        </div>
        <div class="w-full h-[2px] bg-neutral-700"></div>
        <div class="flex w-full justify-between items-center">
          <h4 class="text-xl font-medium">Hardening</h4>
          <Switch bind:value={$extrasStore.hardening}></Switch>
        </div>
      </div>
      <div
        class="w-full h-2/6 text-xl flex justify-center items-center bg-[#1A1A1A] px-8 pt-4 pb-8 space-y-4 rounded-2xl"
      >
        For something else in future
      </div>
    </div>
    <div class="h-full w-1/3 space-y-4">
      <div
        class="w-full h-fit text-xl flex flex-col justify-center items-center bg-[#1A1A1A] px-8 pt-4 pb-8 space-y-4 rounded-2xl"
      >
        <div class="flex space-x-3 w-full justify-center items-center">
          <img src={wrenchIcon} alt="kernel" />
          <h3 class="font-semibold text-3xl">Installer Additional Arguments</h3>
        </div>
        <div class="w-full h-[2px] bg-neutral-700"></div>
        <div class="flex w-full justify-between items-center">
          <h4 class="text-xl font-medium">--keep-going</h4>
          <Switch bind:value={$extrasStore.keepgoing}></Switch>
        </div>
        <div class="w-full h-[2px] bg-neutral-700"></div>
        <div class="flex w-full justify-between items-center">
          <h4 class="text-xl font-medium">--max-jobs</h4>
          <div class="w-1/2">
            <InputBox
              styleClass="text-base"
              label=""
              placeholderText="1"
              inputType={"number"}
              min={1}
              givenOnChangeValue={onChangeFunctionMaxJobs}
            ></InputBox>
          </div>
        </div>
        <div class="w-full h-[2px] bg-neutral-700"></div>
        <div class="flex w-full justify-between items-center">
          <h4 class="text-xl font-medium">--cores</h4>
          <div class="w-1/2">
            <InputBox
              styleClass="text-base"
              label=""
              placeholderText="0"
              inputType={"number"}
              min={0}
              givenOnChangeValue={onChangeFunctionCores}
            ></InputBox>
          </div>
        </div>
      </div>
      <div
        class="w-full h-1/4 text-xl flex justify-center items-center bg-[#1A1A1A] px-8 pt-4 pb-8 space-y-4 rounded-2xl"
      >
        For something else in future
      </div>
    </div>
  </div>
</StepWrapper>

<style>
  li {
    @apply flex items-center justify-between py-2;
  }
</style>
