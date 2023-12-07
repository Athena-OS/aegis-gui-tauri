<script lang="ts">
  export let totalValue = 100;

  interface SegmentItem {
    device: string;
    name: string;
    fileSystem: string;
    mountPoint: string;
    size: string;
    sizeInMB: number;
  }

  export let items: SegmentItem[];

  const backgroundClasses = [
    "bg-red-500",
    "bg-blue-500",
    "bg-green-500",
    "bg-yellow-500",
    "bg-blue-500",
    "bg-purple-500",
    "bg-indigo-500",
    "bg-cyan-500",
  ];
</script>

<div class="w-full bg-neutral-800 h-10 flex overflow-hidden rounded-full">
  {#each items as item, index}
    <div
      class="h-full {backgroundClasses[index]}"
      style="width: {(item.sizeInMB / totalValue) * 100}%"
    ></div>
  {/each}
</div>
