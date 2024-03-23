<script lang="ts">
  import replaceIcon from "../../assets/icons/replace-yellow.svg";
  import diskIcon from "../../assets/icons/disk.svg";
  import { createDialog } from "svelte-headlessui";
  import { bytesToGB, bytesToMB } from "../../lib/utils/functions";
  import partitionStore from "../../lib/stores/partitionStore";

  import SegementedBar from "../../lib/components/SegementedBar.svelte";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import Button from "../../lib/components/Button.svelte";
  import Dialog from "../../lib/components/Dialog.svelte";
  let storageDevicesList: any[] = [];
  let partitionData: any[] = [];
  const colorList = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];

  async function gatherInfo() {
    storageDevicesList = [];
    partitionData = [];

    $partitionStore.systemStorageInfo.map((diskData) => {
      storageDevicesList.push({
        name: diskData.displayName,
        selected:
          diskData.displayName === $partitionStore.selectedDevice
            ? true
            : false,
      });

      if (diskData.displayName === $partitionStore.selectedDevice) {
        diskData.partitions.map((partition) => {
          partitionData.push(partition);
        });
      }
    });

    partitionData.forEach((partition, index) => {
      partition.color = colorList[index % colorList.length];
    });
  }

  async function changeAllowCreation() {
    try {
      if (
        parseInt(
          bytesToGB(
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]?.availableStorage,
          ),
        ) > 1
      ) {
        allowCreation = true;
      } else {
        allowCreation = false;
      }
    } catch (_) {
      allowCreation = false;
    }
  }

  gatherInfo();
  $: $partitionStore.systemStorageInfo, gatherInfo();
  $: $partitionStore.selectedDevice, gatherInfo();
  $: $partitionStore.systemStorageInfo.filter(
    (item) => item.displayName === $partitionStore.selectedDevice,
  )[0]?.availableStorage,
    changeAllowCreation();

  let allowCreation = true;
  // replace partition
  let dialogReplacePartition = createDialog({ label: "replace-partition" });

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    const selectedDeviceStorageInfo = $partitionStore.systemStorageInfo.find(
      (item) => item.displayName === $partitionStore.selectedDevice,
    );
    const athenaOSPartitionSource = selectedDeviceStorageInfo?.partitions.find(
      (partition) => partition.name === "Athena OS",
    );
    if (athenaOSPartitionSource) {
      nextPage = "summary";
    }
  }

  $: $partitionStore, IsOkayToMoveNextPage();
</script>

<Dialog dialog={dialogReplacePartition}>
  <div class="w-full h-fit my-4 space-y-8 flex flex-col justify-between">
    <h4 class="text-2xl font-meidum">Replace Partition</h4>
    <Dropdown
      items={[
        ...partitionData
          .filter((p) => p.size > 2e10) // remove partitions below 20gb in size
          .map((partition) => {
            return { name: partition.partitionName, selected: false };
          }),
      ]}
      icon={diskIcon}
      label="Select Partition"
      on:select={(event) => {
        $partitionStore.replacedPartition = $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (item) => item.partitionName === event.detail.selected.name,
          )[0];
      }}
      defaultItem={{ name: "Select Partition" }}
    />

    <h4 class="text-xl my-4 font-meidum text-red-500">
      *The following partition will be replaced with Athena OS Partition
    </h4>
  </div>
  <div class="flex justify-between space-x-2">
    <Button
      variant="bordered"
      on:click={() => {
        IsOkayToMoveNextPage();
        dialogReplacePartition.close();
      }}
      fullWidth>Cancel</Button
    >
    <Button
      on:click={() => {
        console.log(
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  $partitionStore.replacedPartition.partitionName &&
                partition.size === $partitionStore.replacedPartition.size,
            )[0],
        );

        const selectedDeviceStorageInfo =
          $partitionStore.systemStorageInfo.find(
            (item) => item.displayName === $partitionStore.selectedDevice,
          );

        const athenaOSPartitionSource =
          selectedDeviceStorageInfo?.partitions.find(
            (partition) => partition.name === "Athena OS",
          );

        const athenaOSPartitionTarget =
          selectedDeviceStorageInfo?.partitions.find(
            (partition) => partition.name === "Athena OS",
          );

        if (athenaOSPartitionTarget && athenaOSPartitionSource) {
          athenaOSPartitionTarget.name =
            athenaOSPartitionSource.partitionName ??
            athenaOSPartitionTarget.name;
        }

        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
                $partitionStore.replacedPartition.partitionName &&
              partition.size === $partitionStore.replacedPartition.size,
          )[0].name = "Athena OS";

        console.log(
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  $partitionStore.replacedPartition.partitionName &&
                partition.size === $partitionStore.replacedPartition.size,
            )[0],
        );

        gatherInfo();
        changeAllowCreation();
        IsOkayToMoveNextPage();
        console.log(nextPage)
        dialogReplacePartition.close();
      }}
      fullWidth>Confirm</Button
    >
  </div>
</Dialog>

<StepWrapper
  title="Replace Partition"
  dialogTitle="About replace partition"
  dialogContent="Replace partition lets you select a partition to install Athena OS on"
  prev="/partition"
  next={nextPage}
>
  <div class="flex flex-col items-center mx-5 h-full space-y-6">
    <div class="flex flex-row items-center gap-4 w-full">
      <div class="max-w-md w-full">
        <Dropdown
          bind:items={storageDevicesList}
          label="Select Drive"
          icon={diskIcon}
          on:select={(event) =>
            ($partitionStore.selectedDevice = event.detail.selected.name)}
          defaultItem={{ name: "Select Drive" }}
        />
      </div>
      <div class="w-full">
        <p class="text-[#B0B0B0] text-left font-semibold mb-2">Partition</p>
        {#if $partitionStore.systemStorageInfo.length > 0}
          <SegementedBar
            totalValue={parseFloat(
              bytesToMB(
                $partitionStore.systemStorageInfo.filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0].totalStorage,
              ),
            )}
            items={partitionData}
          />
        {/if}
      </div>
    </div>
    <div class="w-full">
      <h3 class="font-semibold mb-2 text-[#B0B0B0]">New Partition Table</h3>
      <div
        class="rounded-2xl overflow-hidden bg-[#1A1A1A] border-2 border-[#2F2F2F]"
      >
        <div class="max-h-[18.3em] overflow-auto">
          {#if $partitionStore.systemStorageInfo.length > 0}
            <table class="min-w-full w-full">
              <thead class="bg-[#363636] sticky top-0">
                <tr>
                  <th class="w-1/6 text-left p-3">Block Device</th>
                  <th class="text-left p-3">Name</th>
                  <th class="text-left p-3">Type</th>
                  <th class="text-left p-3">File System</th>
                  <th class="text-left p-3">Mount Point</th>
                  <th class="text-left p-3">Size</th>
                </tr>
              </thead>
              <tbody>
                {#each partitionData as row}
                  <tr class="border-t border-[#2F2F2F]">
                    <td
                      class="text-white font-semibold p-3 flex items-center gap-2"
                    >
                      <div class={`${row.color} rounded-full w-3 h-3`} />
                      {row.partitionName}
                    </td>
                    <td class="text-[#B0B0B0] p-3">{row.name.toUpperCase()}</td>
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
    <div class="flex w-full justify-end space-x-4">
      <Button variant="bordered" on:click={dialogReplacePartition.open}>
        <img class="h-6" src={replaceIcon} alt="" />
        <span>Replace</span></Button
      >
    </div>
  </div>
</StepWrapper>
