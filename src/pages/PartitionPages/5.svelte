<script lang="ts">
  import replaceIcon from "../../assets/icons/replace-yellow.svg";
  import diskIcon from "../../assets/icons/disk.svg";
  import { createDialog } from "svelte-headlessui";
  import { bytesToGB, bytesToMB } from "../../lib/utils/functions";
  import partitionStore from "../../lib/stores/partitionStore";
  import { invoke } from "@tauri-apps/api/core";
  import SegementedBar from "../../lib/components/SegementedBar.svelte";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import Button from "../../lib/components/Button.svelte";
  import Dialog from "../../lib/components/Dialog.svelte";
  import {
    type StorageDevice,
    type InstallAlongPartition,
  } from "../../lib/utils/types";
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
  let dialogBootPartition = createDialog({ label: "boot-partition" });
  let nextPage = "";
  function IsOkayToMoveNextPage() {
    /*const selectedDeviceStorageInfo = $partitionStore.systemStorageInfo.find(
      (item) => item.displayName === $partitionStore.selectedDevice,
    );
    const athenaOSPartitionSource = selectedDeviceStorageInfo?.partitions.find(
      (partition) => partition.name === "Athena OS",
    );*/
    if ($partitionStore.replacedPartition.partitionName != "" && $partitionStore.bootPartition.partitionName != "") {
      nextPage = "summary";
    }
  }

 async function refreshPartitions(){
  console.log('refresh called')
  await invoke("get_partitions").then((partitions) => {
      let p = JSON.parse(partitions as string)?.blockdevices;
      for (let i = 0; i < p.length; i++) {
        let disk: StorageDevice = {
          diskModel: p[i].model,
          logicalName: p[i].model,
          displayName: p[i].kname,
          totalStorage: p[i].size,
          availableStorage: 0, // This will be calculated later
          disklabelType: p[i].pptype,
          kind: "",
          isRemovable: p[i].rm,
          partitions: [],
        };

        let children = p[i].children ?? [];
        children.sort((p1: any, p2: any) => p1.start - p2.start);
        // Calculate spaces between partitions and at the end
        let lastEnd = children[0]?.start;
        children.forEach((part: any, index: any) => {
          let start = parseInt(part.start, 10);
          let size = parseInt(part.size, 10);
          let end = start + size / 512;

          // Calculate space before this partition (if any)
          // TODO: Come up with a better way of checking the space at the beginning.
          if (index == 0 && part.start > 4096) {
            disk.partitions.push({
              partitionName: "free-space-" + index,
              size: (part.start - 4096) * 512,
              fileSystem: "",
              mountPoint: "",
              availableStorage: start - lastEnd,
              name: "free",
              start: 4096,
              end: part.start,
              resized: false,
              action: "none",
            });
          } else if (start - lastEnd > 0) {
            // Insert a free space partition object before this partition
            disk.partitions.push({
              partitionName: "free-space-" + index,
              size: (start - lastEnd) * 512,
              fileSystem: "",
              mountPoint: "",
              availableStorage: (start - lastEnd) * 512,
              name: "free",
              start: lastEnd,
              end: start,
              resized: false,
              action: "none",
            });
          }

          // Add the current partition
          disk.partitions.push({
            partitionName: part.name,
            size: part.size,
            fileSystem: part.parttypename,
            mountPoint: part.mountpoint,
            availableStorage: part.fsavail,
            name: part.kname,
            start: part.start,
            end: part.start + part.size / 512,
            resized: false,
            action: "none",
          });

          lastEnd = end;
        });

        // Check for space at the end of the disk
        let diskTotalSize = disk.totalStorage;
        if (diskTotalSize / 512 - lastEnd > 0) {
          disk.partitions.push({
            partitionName: "free-space-end",
            size: diskTotalSize - lastEnd * 512,
            fileSystem: "",
            mountPoint: "",
            availableStorage: diskTotalSize - lastEnd,
            name: "free",
            start: lastEnd,
            end: diskTotalSize / 512,
            resized: false,
            action: "none",
          });
        }

        // Now that we've added all free spaces, calculate availableStorage for the disk
        disk.availableStorage = disk.partitions
          .filter((part) => part.name === "free")
          .reduce((acc, curr) => acc + curr.size, 0);
        let temp_disk_data = JSON.parse(JSON.stringify(disk));
        $partitionStore.systemStorageInfoCurrent.push({ ...temp_disk_data });
        $partitionStore.systemStorageInfo = []
        $partitionStore.systemStorageInfo.push(disk);
      }
    });
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
        $partitionStore.replacedPartition.partitionName = $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (item) => item.partitionName === event.detail.selected.name,
          )[0].partitionName;
      }}
      defaultItem={{ name: "Select Partition" }}
    />
    <Dropdown
      items={[
        {name: "none", selected:true},
        {name:"/"},
        {name:"/boot"},
        {name:"/home"},
        {name:"/opt"},
        {name:"/usr"},
        {name:"/var"}
      ]}
      icon={diskIcon}
      label="Select MountPoint"
      on:select={(event) => {
        $partitionStore.replacedPartition.mountPoint = event.detail.selected.name
      }}
      defaultItem={{ name: "Select MountPoint" }}
    />
    <Dropdown
      items={[
        {name: "don't format", selected:true},
        {name:"btrfs"},
        {name:"ext4"},
        {name:"f2fs"},
        {name:"vfat"},
        {name:"xfs"}
      ]}
      icon={diskIcon}
      label="Select FileSystem"
      on:select={(event) => {
        $partitionStore.replacedPartition.fileSystem = event.detail.selected.name;
      }}
      defaultItem={{ name: "Select FileSystem" }}
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
                $partitionStore.replacedPartition.partitionName
          )[0].mountPoint =$partitionStore.replacedPartition.mountPoint;
          $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
                $partitionStore.replacedPartition.partitionName
          )[0].fileSystem =$partitionStore.replacedPartition.fileSystem;
          $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
                $partitionStore.replacedPartition.partitionName
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
<Dialog dialog={dialogBootPartition}>
  <div class="w-full h-fit my-4 space-y-8 flex flex-col justify-between">
    <h4 class="text-2xl font-meidum">Replace Partition</h4>
    <Dropdown
      items={[
        ...partitionData
          .filter((p) => p.size > 5.12e+8) // 
          .map((partition) => {
            return { name: partition.partitionName, selected: false };
          }),
      ]}
      icon={diskIcon}
      label="Select Partition"
      on:select={(event) => {
        $partitionStore.bootPartition.partitionName = event.detail.selected.name
      }}
      defaultItem={{ name: "Select Partition" }}
    />
    <Dropdown
    items={[
      {name: "don't format", selected:true},
      {name:"fat"},
      {name:"vfat"},
      
    ]}
    icon={diskIcon}
    label="Select FileSystem"
    on:select={(event) => {
      $partitionStore.bootPartition.fileSystem = event.detail.selected.name;
    }}
    defaultItem={{ name: "Select FileSystem" }}
  />
    <h4 class="text-xl my-4 font-meidum text-red-500">
      *The following partition will be used as boot Partition
    </h4>
  </div>
  <div class="flex justify-between space-x-2">
    <Button
      variant="bordered"
      on:click={() => {
        IsOkayToMoveNextPage();
        dialogBootPartition.close();
      }}
      fullWidth>Cancel</Button
    >
    <Button
      on:click={() => {
        const selectedDeviceStorageInfo =
          $partitionStore.systemStorageInfo.find(
            (item) => item.displayName === $partitionStore.selectedDevice,
          );

        const athenaOSPartitionSource =
          selectedDeviceStorageInfo?.partitions.find(
            (partition) => partition.name === "boot",
          );

        const athenaOSPartitionTarget =
          selectedDeviceStorageInfo?.partitions.find(
            (partition) => partition.name === "boot",
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
                $partitionStore.bootPartition.partitionName
          )[0].mountPoint =$partitionStore.replacedPartition.mountPoint;
          $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
                $partitionStore.bootPartition.partitionName
          )[0].fileSystem =$partitionStore.replacedPartition.fileSystem;
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
                $partitionStore.bootPartition.partitionName
          )[0].name = "boot";

        IsOkayToMoveNextPage();
        dialogBootPartition.close();
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
      <div style="color:red">You must select boot and athenaOS root partitions. If you cant find the partitions you would like to use, please consider using gparted or select manual mode.</div>
      <h3 class="font-semibold mb-2 text-[#B0B0B0]">Partition Table</h3>
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
      <Button variant="bordered" on:click={async ()=>{await refreshPartitions()}}>
        <img class="h-6" src={replaceIcon} alt="" />
        <span>Refresh</span></Button
      >
      <Button variant="bordered" on:click={dialogReplacePartition.open}>
        <img class="h-6" src={replaceIcon} alt="" />
        <span>Select root partition</span></Button
      >
      <Button variant="bordered" on:click={dialogBootPartition.open}>
        <img class="h-6" src={replaceIcon} alt="" />
        <span>Select boot partition</span></Button
      >
    </div>
  </div>
</StepWrapper>
