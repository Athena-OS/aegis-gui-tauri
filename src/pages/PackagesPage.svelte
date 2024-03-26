<script lang="ts">
  import packagesIcon from "../assets/icons/packages-icon.svg";

  import CheckButton from "../lib/components/CheckButton.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  import packagesStore from "../lib/stores/packagesStore";

  let gpuOptions = [
    {
      label: "Temperature Monitoring",
      description:
        "GPU utilities can provide real-time temperature monitoring, allowing users to keep an eye on the GPU's temperature to prevent overheating.",
      selected: false,
    },
    {
      label: "Fan Control",
      description:
        "Many GPU utilities allow users to manually or automatically control the GPU's fan speed to manage temperature and noise levels.",
      selected: false,
    },
    {
      label: "Overclocking",
      description:
        "Overclocking utilities enable users to increase the clock speed and memory speed of their GPU, potentially boosting performance. However, this should be done with caution to avoid damaging the GPU. (Be careful when enabling this feature)",
      selected: false,
      special: true,
    },
    {
      label: "GPU Usage Monitoring",
      description:
        "These tools show the percentage of GPU utilization by different processes or applications, helping users identify what's causing high GPU usage.",
      selected: false,
    },
  ];

  let technicalSupportOptions = [
    {
      label: "Option 1",
      description: "",
      selected: false,
    },
    {
      label: "Option 2",
      description: "",
      selected: false,
    },
    {
      label: "Option 3",
      description: "",
      selected: false,
    },
  ];

  let browsersOptions = [
    {
      label: "Firefox",
      description: "",
      selected: false,
    },
    {
      label: "Brave",
      description: "",
      selected: false,
    },
    {
      label: "Chromium",
      description: "",
      selected: false,
    },
  ];

  let gpuOptionsExpanded = true;
  let technicalSupportOptionsExpanded = false;
  let browsersOptionsExpanded = false;

  export let switchView: (viewName: string) => void;

  function saveGPUOptions() {
    let temp: string[] = [];
    gpuOptions.forEach((element) => {
      if (element.selected) temp.push(element.label);
    });
    $packagesStore.packages = {
      ...$packagesStore.packages,
      "GPU Utilities": temp,
    };
  }

  function saveTechnicalSupportOptions() {
    let temp: string[] = [];
    technicalSupportOptions.forEach((element) => {
      if (element.selected) temp.push(element.label);
    });
    $packagesStore.packages = {
      ...$packagesStore.packages,
      "Technical Support Options": temp,
    };
  }

  function saveBrowserOptions() {
    let temp: string[] = [];
    browsersOptions.forEach((element) => {
      if (element.selected) temp.push(element.label);
    });
    $packagesStore.packages = {
      ...$packagesStore.packages,
      Browsers: temp,
    };
  }

  $: gpuOptions, saveGPUOptions();
  $: technicalSupportOptions, saveTechnicalSupportOptions();
  $: browsersOptions, saveBrowserOptions();
</script>

<StepWrapper
  title="Select Packages"
  dialogTitle="Packages Page"
  dialogContent="Here you select the packages to install"
  prev="/desktop"
  next="/accounts"
>
  <div class="flex flex-col items-center justify-center h-full">
    <div
      class="flex flex-row bg-neutral-900 w-full h-full rounded-3xl border border-neutral-800"
    >
      <div class="flex flex-col w-1/4 p-4">
        <div class="flex flex-row items-center gap-2">
          <img src={packagesIcon} alt="packages" class="w-5" />
          <h3 class="font-medium text-md">Packages</h3>
        </div>
        <div class="h-0.5 bg-neutral-800 w-auto mt-3 mb-3" />
        <div class="w-full space-y-2 overflow-y-auto">
          <CheckButton
            label="GPU Utilities"
            bind:options={gpuOptions}
            bind:expanded={gpuOptionsExpanded}
          />
          <CheckButton
            label="Technical Support"
            bind:options={technicalSupportOptions}
            bind:expanded={technicalSupportOptionsExpanded}
          />
          <!--CheckButton
            label="Browsers"
            bind:options={browsersOptions}
            bind:expanded={browsersOptionsExpanded}
          /-->
        </div>
      </div>
      <div class="w-0.5 h-full bg-neutral-800" />
      <div
        class="flex flex-col px-4 py-5 gap-2 w-3/4 scroll-x-auto overflow-auto space-y-0"
      >
        {#if gpuOptionsExpanded}
          <h3 class="text-3xl font-medium text-[#FFB800]">GPU Utilities</h3>
          <p class="font-base w-full text-xl">
            GPU utilities refer to software and tools designed to monitor,
            manage, and optimize your Graphics Processing Unit (GPU). These
            utilities help users make the most of their GPU's capabilities,
            whether it's for gaming, content creation, machine learning, or
            other GPU-intensive tasks. Here are some key points and
            functionalities that come under GPU utilities:
          </p>
        {/if}
        <ul class="list-disc mx-4">
          {#each gpuOptions as option}
            {#if option.selected}
              <li class="font-base w-full text-lg">
                {#if option.hasOwnProperty("special")}
                  <span class={`font-medium text-red-500`}>{option.label}</span>
                {:else}
                  <span class={`font-medium text-[#FFB800]`}
                    >{option.label}</span
                  >
                {/if} : {option.description}
              </li>
            {/if}
          {/each}
        </ul>
        {#if technicalSupportOptionsExpanded}
          <div class="min-h-1 bg-neutral-800 w-auto mt-3 mb-3" />
          <h3 class="text-3xl font-medium text-[#FFB800]">Technical Support</h3>
          <p class="font-base w-full text-xl">
            GPU utilities refer to software and tools designed to monitor,
            manage, and optimize your Graphics Processing Unit (GPU). These
            utilities help users make the most of their GPU's capabilities,
            whether it's for gaming, content creation, machine learning, or
            other GPU-intensive tasks. Here are some key points and
            functionalities that come under GPU utilities:
          </p>
        {/if}
        <ul class="list-disc mx-4">
          {#each technicalSupportOptions as option}
            {#if option.selected}
              <li class="font-base w-full text-lg">
                {#if option.hasOwnProperty("special")}
                  <span class={`font-medium text-red-500`}>{option.label}</span>
                {:else}
                  <span class={`font-medium text-[#FFB800]`}
                    >{option.label}</span
                  >
                {/if} : {option.description}
              </li>
            {/if}
          {/each}
        </ul>
        <!--{#if browsersOptionsExpanded}
          <div class="min-h-1 bg-neutral-800 w-auto mt-3 mb-3" />
          <h3 class="text-3xl font-medium text-[#FFB800]">Browsers</h3>
          <p class="font-base w-full text-xl"></p>
        {/if}
        <ul class="list-disc mx-4">
          {#each browsersOptions as option}
            {#if option.selected}
              <li class="font-base w-full text-lg">
                {#if option.hasOwnProperty("special")}
                  <span class={`font-medium text-red-500`}>{option.label}</span>
                {:else}
                  <span class={`font-medium text-[#FFB800]`}
                    >{option.label}</span
                  >
                {/if} : {option.description}
              </li>
            {/if}
          {/each}
        </ul-->
      </div>
    </div>
  </div>
</StepWrapper>
