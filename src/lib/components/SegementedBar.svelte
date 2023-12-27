<script lang="ts">
  import { bytesToMB } from "../utils/functions";
  import type { PartitionInfo } from "../utils/types";

  export let totalValue = 100;

  export let items: PartitionInfo[];

  const backgroundClasses = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];
</script>

<div class="w-full bg-neutral-800 h-10 flex overflow-hidden rounded-full">
  {#each items as item, index}
    <div
      class="h-full {backgroundClasses[index]}"
      style="width: {(parseFloat(bytesToMB(item.size)) / totalValue) * 100}%"
    ></div>
  {/each}
</div>
