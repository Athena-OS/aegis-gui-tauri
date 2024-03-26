<script lang="ts">
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import SegementedBar from "../../lib/components/SegementedBar.svelte";
  import partitionStore from "../../lib/stores/partitionStore";
  import { bytesToGB, bytesToMB } from "../../lib/utils/functions";

  let partitionNewData: any[] = [];
  let partitionCurrentData: any[] = [];
  const colorList = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];

  function gatherInfo() {
    $partitionStore.systemStorageInfo.map((diskData) => {
      if (diskData.displayName === $partitionStore.selectedDevice) {
        diskData.partitions.map((partition) => {
          partitionNewData.push(partition);
        });
      }
    });
    $partitionStore.systemStorageInfoCurrent.map((diskData) => {
      if (diskData.displayName === $partitionStore.selectedDevice) {
        diskData.partitions.map((partition) => {
          partitionCurrentData.push(partition);
        });
      }
    });

    partitionNewData.forEach((partition, index) => {
      partition.color = colorList[index % colorList.length];
    });
    partitionCurrentData.forEach((partition, index) => {
      partition.color = colorList[index % colorList.length];
    });
  }

  gatherInfo();
</script>

<StepWrapper
  title="Finalize Partition"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="configure-partition"
  next="summary"
>
  <div class="space-y-5">
    <div class="flex flex-col items-center mx-5 h-fit space-y-2">
      <div class="text-left w-full text-2xl font-semibold">
        1. Current Partition
      </div>
      <SegementedBar
        totalValue={parseFloat(
          bytesToMB(
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].totalStorage,
          ),
        )}
        items={partitionCurrentData}
      />
      <div
        class="rounded-2xl overflow-hidden w-full bg-[#1A1A1A] border-2 border-[#2F2F2F]"
      >
        <div class="max-h-[18.3em] overflow-auto w-full">
          {#if $partitionStore.systemStorageInfoCurrent.length > 0}
            <table class="min-w-full w-full">
              <thead class="bg-[#363636] sticky top-0">
                <tr>
                  <th class="w-1/6 text-left p-3">Block Device</th>
                  <th class="text-left p-3">Name</th>
                  <th class="text-left p-3">File System</th>
                  <th class="text-left p-3">Mount Point</th>
                  <th class="text-left p-3">Size</th>
                </tr>
              </thead>
              <tbody>
                {#each partitionCurrentData as row, index}
                  <tr class="border-t border-[#2F2F2F]">
                    <td
                      class="text-white font-semibold p-3 flex items-center gap-2"
                    >
                      <div class={`${colorList[index]} rounded-full w-3 h-3`} />
                      {row.partitionName}
                    </td>
                    <td class="text-[#B0B0B0] p-3">{row.name.toUpperCase()}</td>
                    <td class="text-[#B0B0B0] p-3"
                      >{row.fileSystem.toUpperCase()}</td
                    >
                    <td class="text-[#B0B0B0] p-3">{row.mountPoint}</td>
                    <td class="text-[#B0B0B0] font-semibold p-3"
                      >{bytesToMB(parseInt(row.size))} MB / {bytesToGB(
                        parseInt(row.size),
                      )} GB</td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>
    </div>
    <div class="flex flex-col items-center mx-5 h-fit space-y-2">
      <div class="text-left w-full text-2xl font-semibold">
        2. New Partition
      </div>
      <SegementedBar
        totalValue={parseFloat(
          bytesToMB(
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].totalStorage,
          ),
        )}
        items={partitionNewData}
      />

      <div
        class="rounded-2xl overflow-hidden w-full bg-[#1A1A1A] border-2 border-[#2F2F2F]"
      >
        <div class="max-h-[18.3em] overflow-auto w-full">
          {#if $partitionStore.systemStorageInfo.length > 0}
            <table class="min-w-full w-full">
              <thead class="bg-[#363636] sticky top-0">
                <tr>
                  <th class="w-1/6 text-left p-3">Block Device</th>
                  <th class="text-left p-3">Name</th>
                  <th class="text-left p-3">File System</th>
                  <th class="text-left p-3">Mount Point</th>
                  <th class="text-left p-3">Size</th>
                </tr>
              </thead>
              <tbody>
                {#each partitionNewData as row, index}
                  <tr class="border-t border-[#2F2F2F]">
                    <td
                      class="text-white font-semibold p-3 flex items-center gap-2"
                    >
                      <div class={`${colorList[index]} rounded-full w-3 h-3`} />
                      {row.partitionName}
                    </td>
                    <td class="text-[#B0B0B0] p-3">{row.name.toUpperCase()}</td>
                    <td class="text-[#B0B0B0] p-3"
                      >{row.fileSystem.toUpperCase()}</td
                    >
                    <td class="text-[#B0B0B0] p-3">{row.mountPoint}</td>
                    <td class="text-[#B0B0B0] font-semibold p-3"
                      >{bytesToMB(parseInt(row.size))} MB / {bytesToGB(
                        parseInt(row.size),
                      )} GB</td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>
    </div>
  </div>
</StepWrapper>
