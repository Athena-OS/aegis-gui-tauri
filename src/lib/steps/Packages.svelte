<script lang="ts">
  import PackageButton from "../components/PackageButton.svelte";

  import packagesIcon from "../../assets/icons/packages-icon.svg";
  import StepWrapper from "../components/StepWrapper.svelte";

  let gpuOptions = [
    { label: "Option 1", value: "option1" },
    { label: "Option 2", value: "option2" },
    { label: "Option 3", value: "option3" },
  ];

  let otherOptions = [
    { label: "Option 1", value: "option1" },
    { label: "Option 2", value: "option2" },
    { label: "Option 3", value: "option3" },
  ];

  let refundExpanded = false;
  let supportExpanded = false;

  export let switchView: (viewName: string) => void;
</script>

<StepWrapper
  title="Select Packages"
  dialogTitle="Header Here"
  dialogContent="Your text here"
>
  <div class="flex flex-col items-center justify-center h-full">
    <div
      class="flex flex-row bg-neutral-900 w-full h-full rounded-3xl border border-neutral-800"
    >
      <div class="flex flex-col w-72 p-4">
        <div class="flex flex-row items-center gap-2">
          <img src={packagesIcon} alt="packages" class="w-5" />
          <h3 class="font-medium text-md">Packages</h3>
        </div>
        <div class="h-0.5 bg-neutral-800 w-auto mt-3 mb-3" />
        <div class="w-full space-y-2">
          <PackageButton
            label="GPU utilities"
            options={gpuOptions}
            bind:expanded={refundExpanded}
          />
          <PackageButton
            label="Technical Support"
            options={otherOptions}
            bind:expanded={supportExpanded}
          />
        </div>
      </div>
      <div class="w-0.5 h-full bg-neutral-800" />
      <div class="flex flex-col px-4 py-5 gap-2">
        {#if refundExpanded}
          <h3 class="text-3xl font-medium text-[#FFB800]">GPU Utilities</h3>
          <p class="font-medium max-w-xl">
            GPU utilities refer to software and tools designed to monitor,
            manage, and optimize your Graphics Processing Unit (GPU). These
            utilities help users make the most of their GPU's capabilities,
            whether it's for gaming, content creation, machine learning, or
            other GPU-intensive tasks. Here are some key points and
            functionalities that come under GPU utilities:
          </p>
        {/if}
      </div>
    </div>
  </div>
</StepWrapper>
