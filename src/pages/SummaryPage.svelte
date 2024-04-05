<script>
  // @ts-nocheck

  import StepWrapper from "../lib/components/StepWrapper.svelte";

  import keyboardSummary from "../assets/keyboard-summary.svg";
  import keyboardStore from "../lib/stores/keyboardStore";

  import environmentIcon from "../assets/icons/environment.svg";
  import themeIcon from "../assets/icons/theme.svg";
  import displayManagerIcon from "../assets/icons/desktop-manager.svg";
  import desktopStore from "../lib/stores/desktopStore";

  import packagesSummary from "../assets/packages-summary.svg";
  import packagesStore from "../lib/stores/packagesStore";
  import partitionStore from "../lib/stores/partitionStore";
  import globalStore from "../lib/stores/globalStore";
  import nix from "../assets/nix.svg";
  import arch from "../assets/arch.svg";
</script>

<StepWrapper
  title="Summary"
  dialogTitle="This is the summary of the installation option choices"
  dialogContent="Your text here"
  prev={(() => {
    const pm = $partitionStore.mode;
    if (pm === "auto") {
      return "/partition";
    } else if (pm == "replace-partition") {
      return "/replace-partition";
    }else if (pm == "install-along"){
      return "/configure-install-along"
    } else {
      return "/finalize-partition";
    }
  })()}
  next="/install"
>
  <div
    class="w-full h-full overflow-auto border-4 border-primary-400 rounded-xl"
  >
    <div class="w-full bg-transparent h-full py-4 justify-center">
      <div class="space-y-4 flex w-full h-full items-center">
        <div class="w-1/2 h-inherit items-center">
          {#if $globalStore.base == "arch"}
            <img src={arch} class="h-[50] relative z-50" alt="" />
          {:else}
            <img src={nix} class="h-[50px] relative z-50" alt="" />
          {/if}
        </div>
        <div
          class="grow h-inherit flex flex-col text-5xl space-y-8 pt-8 -ml-16"
        >
          <h1 class="text-primary-400 mb-6">Base OS</h1>

          <h4 class="">
            Base Operating System :- <span class="text-white"
              >&nbsp;{$globalStore.base}</span
            >
          </h4>
        </div>
      </div>
    </div>
    <div class="w-full bg-transparent h-full py-4 justify-center">
      <div class="space-y-4 flex w-full h-full overflow-hidden items-center">
        <div class="w-1/2 h-full flex items-center flex items-center p-10">
          <img src={keyboardSummary} class="" alt="" />
        </div>
        <div class="w-1/2 h-full flex flex-col text-5xl space-y-8 pt-8 -ml-16">
          <h1 class="text-primary-400 mb-6">Keyboard</h1>

          <h4 class="text-neutral-400 w-fit flex justify-between">
            Region :- <span class="text-white"
              >&nbsp;{$keyboardStore.region}</span
            >
          </h4>
          <h4 class="text-neutral-400 w-fit flex justify-between">
            Language :- <span class="text-white"
              >&nbsp;{$keyboardStore.language}</span
            >
          </h4>
          <h4 class="text-neutral-400 w-fit flex justify-between">
            Layout :- <span class="text-white"
              >&nbsp;{$keyboardStore.layout}</span
            >
          </h4>
        </div>
      </div>
    </div>
    <div class="w-full bg-transparent h-full py-4 justify-center">
      <div class="space-y-4 flex w-full h-full overflow-hidden items-center">
        <div class="w-1/2 h-full flex items-center p-10">
          <img
            src={$desktopStore.themeImage}
            class="summary-icons-dropshadow border-4 border-primary-400"
            alt=" 1"
          />
        </div>
        <div class="w-1/2 h-full flex flex-col text-4xl space-y-8 pt-8">
          <h1 class="text-primary-400 mb-6">Environment</h1>

          <h4 class="text-neutral-400 w-fit flex justify-between">
            <img src={environmentIcon} class="h-full pr-2" alt="" />Environment
            :- <span class="text-white">&nbsp;{$desktopStore.environment}</span>
          </h4>
          <h4 class="text-neutral-400 w-fit flex justify-between">
            <img src={themeIcon} class="h-full pr-2" alt="" />Theme :-
            <span class="text-white">&nbsp;{$desktopStore.theme}</span>
          </h4>
          <h4 class="text-neutral-400 w-fit flex justify-between">
            <img src={displayManagerIcon} class="h-full pr-2" alt="" />Display
            Manager :-
            <span class="text-white">&nbsp;{$desktopStore.displayManager}</span>
          </h4>
        </div>
      </div>
    </div>
    <div class="w-full bg-transparent h-full py-4 justify-center">
      <div class="space-y-4 flex w-full h-full overflow-hidden items-center">
        <div class="w-1/2 h-full flex items-center p-10">
          <img src={packagesSummary} class="" alt="Theme Theme 2" />
        </div>
        <div class="w-1/2 h-full flex flex-col text-4xl space-y-8 pt-8">
          <h1 class="text-primary-400 mb-6">Packages</h1>

          {#each Object.keys($packagesStore.packages) as packagesCategory}
            <h4 class="text-neutral-400 w-fit flex justify-between">
              {packagesCategory} :-
              <span class="text-white"
                >&nbsp;{$packagesStore.packages[packagesCategory][0]}</span
              >
            </h4>
          {/each}
        </div>
      </div>
    </div>
  </div>
</StepWrapper>
