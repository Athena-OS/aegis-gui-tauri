<script lang="ts">
  import stepsConfig from "../stepsConfig";
  import currentActive from "../stepsStore";
  import Button from "./Button.svelte";

  const handleStepPrevious = () => {
    if ($currentActive > 0) $currentActive--;
  };

  const handleStepNext = () => {
    if ($currentActive < stepsConfig.length - 1) $currentActive++;
  };
</script>

<div class="flex justify-between items-center w-full p-4">
  <Button on:click={handleStepPrevious}
    ><span><i class="ti ti-chevron-left" /></span><span>Previous</span></Button
  >
  <Button on:click={handleStepNext}
    ><span>Next</span><span><i class="ti ti-chevron-right" /></span></Button
  >
</div>
