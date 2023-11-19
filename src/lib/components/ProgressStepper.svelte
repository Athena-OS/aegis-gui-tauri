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

<div class="flex justify-between items-center absolute bottom-4 w-full px-4">
  <Button on:click={handleStepPrevious}>Previous</Button>
  <Button on:click={handleStepNext}>Next</Button>
</div>
