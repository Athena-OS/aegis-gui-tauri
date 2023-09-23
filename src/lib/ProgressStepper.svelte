<script lang="ts">
    export let steps: number;
    export let currentStep: number = 1;
</script>
  
<div class="stepper">
{#each Array(steps) as _, index (index)}
    {#if index > 0}
    <div class="line"></div>
    {/if}
    <div class="nob {index + 1 === currentStep ? 'active' : ''}"></div>
{/each}
</div>

<style>
    .stepper {
      display: flex;
      position: absolute;
      top: 10px; /* Adjust as needed for vertical spacing */
      left: 50%;
      transform: translateX(-50%); /* Centers the element */
      align-items: center;
      z-index: 1000; /* To ensure it stays on top */
    }
  
    .line {
      flex-grow: 1;
      height: 2px;
      background: #ddd;
    }
  
    .nob {
      width: 10px;
      height: 10px;
      border-radius: 50%;
      background: #ddd;
      margin: 0 5px;
    }
  
    .nob.active {
      background: #007bff;
    }
</style>
