<script lang="ts">
  import { bytesToGB, bytesToMB } from "../utils/functions";
  import type { PartitionInfo } from "../utils/types";

  export let totalValue = 100;

  export let color: string = "";

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
      class="group flex justify-center items-center hover:border-2 hover:border-white h-full {color !==
      undefined
        ? color
        : backgroundClasses[index]}"
      style="width: {(parseFloat(bytesToMB(item.size)) / totalValue) * 100}%"
    >
      <h4
        class="absolute mt-20 group-hover:bg-gray-800 group-hover:border-2 group-hover:border-gray-700 font-bold px-2 py-1 rounded-lg text-transparent group-hover:text-white"
      >
        {bytesToMB(item.size)} MB / {bytesToGB(item.size)} GB
      </h4>
    </div>
  {/each}
</div>
