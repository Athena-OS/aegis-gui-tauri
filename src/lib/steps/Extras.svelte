<script lang="ts">
  import kernelIcon from "../../assets/icons/kernel-yellow.svg";
  import terminalIcon from "../../assets/icons/terminal-yellow.svg";
  import wrenchIcon from "../../assets/icons/wrench-yellow.svg";
  import StepWrapper from "../components/StepWrapper.svelte";
</script>

<StepWrapper
  title="Select Keyboard"
  dialogTitle="Header Here"
  dialogContent="Your text here"
>
  <div class="flex flex-col items-center">
    <div class="flex flex-row flex-wrap mt-28 gap-10 max-w-5xl">
      <div
        class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer"
      >
        <img src={kernelIcon} alt="kernel" />
        <h3 class="font-semibold text-4xl">Kernel</h3>
      </div>
      <div
        class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer"
      >
        <img src={terminalIcon} alt="kernel" />
        <div>
          <h3 class="font-semibold text-4xl">Terminal</h3>
          <span class="font-semibold text-xl">& Shell</span>
        </div>
      </div>
      <div
        class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer"
      >
        <img src={wrenchIcon} alt="kernel" />
        <h3 class="font-semibold text-4xl">System</h3>
      </div>
    </div>
  </div>
</StepWrapper>
