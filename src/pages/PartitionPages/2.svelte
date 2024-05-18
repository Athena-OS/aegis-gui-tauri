<script lang="ts">
  import replaceIcon from "../../assets/icons/replace-yellow.svg";
  import editGrayIcon from "../../assets/icons/edit-gray.svg";
  import binGrayIcon from "../../assets/icons/bin-gray.svg";
  import plusWhiteIcon from "../../assets/icons/plus-white.svg";
  import warningIcon from "../../assets/icons/warning.svg";
  import diskIcon from "../../assets/icons/disk.svg";
  import { createDialog } from "svelte-headlessui";
  import {
    GBToMB,
    MBtoBytes,
    bytesToGB,
    bytesToMB,
  } from "../../lib/utils/functions";
  import partitionStore from "../../lib/stores/partitionStore";
  import SegementedBar from "../../lib/components/SegementedBar.svelte";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import Button from "../../lib/components/Button.svelte";
  import Dialog from "../../lib/components/Dialog.svelte";
  import Slider from "../../lib/components/Slider.svelte";
  import InputBox from "../../lib/components/InputBox.svelte";
  import CardGroup from "../../lib/components/CardGroup.svelte";
  import CreatePartitionDialog from "../../lib/components/CreatePartition/CreatePartitionDialog.svelte";
  import CreatePartitionTableDialog from "../../lib/components/CreatePartition/CreatePartitionTableDialog.svelte";
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
  function npt() {
    if ($partitionStore.new_ptable) {
      /*partitionData = [
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          ?.partitions.reduce(
            (acc, curr) => {
              if (acc.start === undefined || curr.start < acc.start) {
                acc.start = curr.start;
              }
              if (acc.end === undefined || curr.end > acc.end) {
                acc.end = curr.end;
              }
              acc.size += curr.size;
              return acc;
            },
            {
              partitionName: "free",
              size: 0,
              availableStorage: 0,
              name: "free",
              fileSystem: "",
              mountPoint: "",
              start: 0,
              end: 0,
              resized: false,
              action: "",
            },
          ),
      ];*/
      $partitionStore.systemStorageInfo.filter(
        (item) => item.displayName === $partitionStore.selectedDevice,
      )[0].partitions = [
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          ?.partitions.reduce(
            (acc, curr) => {
              if (acc.start === undefined || curr.start < acc.start) {
                acc.start = curr.start;
              }
              if (acc.end === undefined || curr.end > acc.end) {
                acc.end = curr.end;
              }
              acc.size += curr.size;
              return acc;
            },
            {
              partitionName: "free",
              size: 0,
              availableStorage: 0,
              name: "free",
              fileSystem: "",
              mountPoint: "",
              start: 0,
              end: 0,
              resized: false,
              action: "",
            },
          ),
      ];
    }
  }
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
  //$: $partitionStore.new_ptable, npt(), gatherInfo();
  $: $partitionStore.systemStorageInfo.filter(
    (item) => item.displayName === $partitionStore.selectedDevice,
  )[0]?.availableStorage,
    changeAllowCreation();

  function ResizingPartitionOnChangeValue(e: any) {
    let value = GBToMB(parseFloat(e.target.value));

    // We can shrink partitions to create free space or expand to take new space
    if (!isNaN(parseFloat(value))) {
      // If the value is a number
      // We check if the value is less than the current size
      if (
        MBtoBytes(parseFloat(value)) <=
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
              selectedPartitionForAction.partitionName,
          )[0].size
      ) {
        e.target.parentElement.classList.remove("border-red-500");
        selectedPartitionForAction.size = MBtoBytes(parseFloat(value));
      } else {
        // If its extension/ not shrinking, It can only be done if there is a free partition/spcae following it
        // find index of the current partition
        let index = $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.findIndex(
            (partition) =>
              partition.partitionName ===
              selectedPartitionForAction.partitionName,
          );
        // if Item exists(Not out of range/undefined)
        if (
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].partitions[index + 1] != undefined
        ) {
          // Now that the partition is defined, lets check if its a free space
          if (
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions[index + 1].name.includes("free")
          ) {
            // lets make sure our partition does not grow beyond the free space
            // Total space of the partition and the free space
            let ts =
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index + 1].size +
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index].size;
            let v = MBtoBytes(parseFloat(value));
            if (v <= ts) {
              e.target.parentElement.classList.remove("border-red-500");
              selectedPartitionForAction.size = MBtoBytes(parseFloat(value));
              // Stop the function execution at this point
              return;
            }
          }
        }
        e.target.parentElement.classList.add("border-red-500");
      }
    } else {
      e.target.parentElement.classList.add("border-red-500");
    }
  }
  function updateInformation() {
    // Update the current partition
    $partitionStore.systemStorageInfo
      .filter((item) => item.displayName === $partitionStore.selectedDevice)[0]
      .partitions.forEach((partition, index) => {
        // Get our desired partition
        if (
          partition.partitionName ===
            selectedPartitionForAction.partitionName &&
          partition.name === selectedPartitionForAction.name
        ) {
          partition.size = selectedPartitionForAction.size;
          partition.resized = true;
          // This should be the second action after delete. Shrink will also be used for expansion.
          // If the partition action is create, it should remain create
          if (partition.action != "create") {
            partition.action = "shrink";
          }

          // update the end. Here we create a new freespace or resize adjacent partitions
          if (partition.size / 512 + partition.start < partition.end) {
            // Partition was shrinked
            // We create a new free empty partition
            if (
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions.length >
              index + 1
            ) {
              // check if the partition is a free space and remove it
              if (
                $partitionStore.systemStorageInfo
                  .filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0]
                  .partitions[index + 1].name.includes("free")
              ) {
                $partitionStore.systemStorageInfo
                  .filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0]
                  .partitions.splice(index + 1, 1);
                // Now create a free partition
                if (
                  $partitionStore.systemStorageInfo.filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0].partitions.length >
                  index + 1
                ) {
                  // If its between partitions
                  let start = $partitionStore.systemStorageInfo.filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0].partitions[index + 1].start;
                  $partitionStore.systemStorageInfo
                    .filter(
                      (item) =>
                        item.displayName === $partitionStore.selectedDevice,
                    )[0]
                    .partitions.push({
                      partitionName: "free-space" + index,
                      size:
                        (start - (partition.size / 512 + partition.start)) *
                        512,
                      fileSystem: "",
                      mountPoint: "",
                      availableStorage:
                        (start - (partition.size / 512 + partition.start)) *
                        512,
                      name: "free" + index,
                      start: partition.size / 512 + partition.start,
                      end: start,
                      resized: false,
                      action: "none",
                    });
                } else {
                  // This means we have eddited the last partition and it has no trailling free space
                  $partitionStore.systemStorageInfo
                    .filter(
                      (item) =>
                        item.displayName === $partitionStore.selectedDevice,
                    )[0]
                    .partitions.push({
                      partitionName: "free-space-end",
                      size:
                        ($partitionStore.systemStorageInfo.filter(
                          (item) =>
                            item.displayName === $partitionStore.selectedDevice,
                        )[0].totalStorage /
                          512 -
                          (partition.start + partition.size / 512)) *
                        512,
                      fileSystem: "",
                      mountPoint: "",
                      availableStorage:
                        ($partitionStore.systemStorageInfo.filter(
                          (item) =>
                            item.displayName === $partitionStore.selectedDevice,
                        )[0].totalStorage /
                          512 -
                          (partition.start + partition.size / 512)) *
                        512,
                      name: "free",
                      start: partition.size / 512 + partition.start,
                      end:
                        $partitionStore.systemStorageInfo.filter(
                          (item) =>
                            item.displayName === $partitionStore.selectedDevice,
                        )[0].totalStorage / 512,
                      resized: false,
                      action: "none",
                    });
                }
              } else {
                // The next partition is not a free space
                let start = $partitionStore.systemStorageInfo.filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0].partitions[index + 1].start;
                $partitionStore.systemStorageInfo
                  .filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0]
                  .partitions.push({
                    partitionName: "free-space" + index,
                    size:
                      (start - (partition.size / 512 + partition.start)) * 512,
                    fileSystem: "",
                    mountPoint: "",
                    availableStorage:
                      (start - (partition.size / 512 + partition.start)) * 512,
                    name: "free" + index,
                    start: partition.size / 512 + partition.start,
                    end: start,
                    resized: false,
                    action: "none",
                  });
              }
            } else {
              // This means we have eddited the last partition and it has no trailling free space
              $partitionStore.systemStorageInfo
                .filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0]
                .partitions.push({
                  partitionName: "free-space-end",
                  size:
                    ($partitionStore.systemStorageInfo.filter(
                      (item) =>
                        item.displayName === $partitionStore.selectedDevice,
                    )[0].totalStorage /
                      512 -
                      (partition.start + partition.size / 512)) *
                    512,
                  fileSystem: "",
                  mountPoint: "",
                  availableStorage:
                    ($partitionStore.systemStorageInfo.filter(
                      (item) =>
                        item.displayName === $partitionStore.selectedDevice,
                    )[0].totalStorage /
                      512 -
                      (partition.start + partition.size / 512)) *
                    512,
                  name: "free-end",
                  start: partition.size / 512 + partition.start,
                  end:
                    $partitionStore.systemStorageInfo.filter(
                      (item) =>
                        item.displayName === $partitionStore.selectedDevice,
                    )[0].totalStorage / 512,
                  resized: false,
                  action: "none",
                });
            }
          } else {
            // We only get here if the partition following the partition being resized is a free space
            // We update the freespace. If All of it was used, we remove the partition
            if (
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index + 1].end ==
              partition.start + partition.size / 512
            ) {
              $partitionStore.systemStorageInfo
                .filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0]
                .partitions.splice(index + 1, 1);
            } else {
              // Here, there is free space left
              // Update start
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index + 1].start =
                partition.start + partition.size / 512;
              // Update size. The end remains the same
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index + 1].size =
                ($partitionStore.systemStorageInfo.filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0].partitions[index + 1].end -
                  $partitionStore.systemStorageInfo.filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0].partitions[index + 1].start) *
                512;
            }
          }
          partition.end = partition.size / 512 + partition.start;
        }
        //)[0].size = selectedPartitionForAction.size;
      });

    // Sort partitions using partition size
    $partitionStore.systemStorageInfo
      .filter((item) => item.displayName === $partitionStore.selectedDevice)[0]
      .partitions.sort((p1, p2) => p1.start - p2.start);

    $partitionStore.systemStorageInfo.filter(
      (item) => item.displayName === $partitionStore.selectedDevice,
    )[0].availableStorage =
      $partitionStore.systemStorageInfo.filter(
        (item) => item.displayName === $partitionStore.selectedDevice,
      )[0].totalStorage -
      $partitionStore.systemStorageInfo
        .filter(
          (item) => item.displayName === $partitionStore.selectedDevice,
        )[0]
        .partitions.reduce((accumulator, partition) => {
          return accumulator + partition.size;
        }, 0);
  }
  // create new partition
  let dialogNewPartition = createDialog({ label: "create-partition" });
  let dialogNewPartitionTable = createDialog({
    label: "create-partition-table",
  });
  let dialogBootPartition = createDialog({ label: "boot-partition" });
  let allowCreation = true;
  // replace partition
  let dialogReplacePartition = createDialog({ label: "replace-partition" });
  // edit existing partition
  let dialogEditPartition = createDialog({ label: "edit-partition" });
  // delete existing partition
  let dialogDeletePartition = createDialog({ label: "delete-partition" });

  let selectedPartitionForAction: any = {
    partitionName: "",
    size: 0,
    availableStorage: 0,
    name: "",
    fileSystem: "",
    mountPoint: "",
  };

  let nextPage = "";
  // Athena OS's parition for installation must be selected
  function IsOkayToMoveNextPage() {
    /*const selectedDeviceStorageInfo = $partitionStore.systemStorageInfo.find(
      (item) => item.displayName === $partitionStore.selectedDevice
    );
    const athenaOSPartitionSource = selectedDeviceStorageInfo?.partitions.find(
      (partition) => partition.name === "Athena OS"
    );
    if (athenaOSPartitionSource) {
      nextPage = "finalize-partition";
    }*/
    if (
      $partitionStore.replacedPartition.partitionName != "" &&
      $partitionStore.bootPartition.partitionName != ""
    ) {
      nextPage = "/finalize-partition";
    }
  }

  $: $partitionStore, IsOkayToMoveNextPage();
</script>

<!-- create new partition -->
<CreatePartitionDialog dialog={dialogNewPartition} />
<CreatePartitionTableDialog dialog={dialogNewPartitionTable} />
<Dialog dialog={dialogBootPartition}>
  <div class="w-full h-fit my-4 space-y-8 flex flex-col justify-between">
    <h4 class="text-2xl font-meidum">Replace Partition</h4>
    <Dropdown
      items={[
        ...partitionData
          .filter((p) => p.size > 5.12e8) //
          .map((partition) => {
            return { name: partition.partitionName, selected: false };
          }),
      ]}
      icon={diskIcon}
      label="Select Partition"
      on:select={(event) => {
        $partitionStore.bootPartition.partitionName =
          event.detail.selected.name;
      }}
      defaultItem={{ name: "Select Partition" }}
    />
    <Dropdown
      items={[
        { name: "don't format", selected: true },
        { name: "fat" },
        { name: "vfat" },
        { name: "ext4" },
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
              $partitionStore.bootPartition.partitionName,
          )[0].mountPoint = $partitionStore.replacedPartition.mountPoint;
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
              $partitionStore.bootPartition.partitionName,
          )[0].fileSystem = $partitionStore.replacedPartition.fileSystem;
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (partition) =>
              partition.partitionName ===
              $partitionStore.bootPartition.partitionName,
          )[0].name = "boot";

        IsOkayToMoveNextPage();
        dialogBootPartition.close();
      }}
      fullWidth>Confirm</Button
    >
  </div>
</Dialog>
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

        // If its a free partition, the action should be create
        if (
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  $partitionStore.replacedPartition.partitionName &&
                partition.partitionName.includes("free"),
            )[0] != undefined
        ) {
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  $partitionStore.replacedPartition.partitionName &&
                partition.partitionName.includes("free"),
            )[0].action = "create";
          // The name should not longer be "free"
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  $partitionStore.replacedPartition.partitionName &&
                partition.partitionName.includes("free"),
            )[0].partitionName =
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].logicalName +
            `p${
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions.length + 1
            }`;
        }
        gatherInfo();
        changeAllowCreation();
        IsOkayToMoveNextPage();
        console.log(nextPage);
        dialogReplacePartition.close();
      }}
      fullWidth>Confirm</Button
    >
  </div>
</Dialog>

<Dialog dialog={dialogEditPartition}>
  <div class="w-full h-fit space-y-4 py-4">
    <h4 class="text-2xl font-meidum">Resize Partition</h4>
    <div class="text-center">
      Resizing {selectedPartitionForAction.partitionName} ({bytesToGB(
        selectedPartitionForAction.size,
      )} GB {selectedPartitionForAction.fileSystem}: {selectedPartitionForAction.name})
    </div>
    <!--SegementedBar
      totalValue={parseFloat(
        bytesToMB(
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].totalStorage -
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.reduce((accumulator, partition) => {
                if (partition.name !== selectedPartitionForAction.name) {
                  return accumulator + partition.size;
                } else {
                  return accumulator;
                }
              }, 0),
        ),
      )}
      color={"bg-cyan-400"}
      items={[{ ...selectedPartitionForAction }]}
    /-->
    {#if selectedPartitionForAction.size !== undefined}
      <!--div class="text-xl justify-between">
        <h4>
          New Reallocated Size: {bytesToGB(
            parseFloat(selectedPartitionForAction.size),
          )} GB
        </h4>
        {#if $partitionStore.systemStorageInfoCurrent
          .filter((item) => item.displayName === $partitionStore.selectedDevice)[0]
          .partitions.filter((partition) => partition.partitionName === selectedPartitionForAction.partitionName).length > 0}
          <h4>
            Used: {bytesToGB(
              $partitionStore.systemStorageInfoCurrent
                .filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0]
                .partitions.filter(
                  (partition) =>
                    partition.partitionName ===
                    selectedPartitionForAction.partitionName,
                )[0].size,
            )} GB
          </h4>
        {/if}
        <h4>
          Available: {bytesToGB(
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].totalStorage -
              ($partitionStore.systemStorageInfo
                .filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0]
                .partitions.reduce((accumulator, partition) => {
                  if (partition.name !== selectedPartitionForAction.name) {
                    return accumulator + partition.size;
                  } else {
                    return accumulator;
                  }
                }, 0) +
                selectedPartitionForAction.size),
          )} GB
        </h4>
      </div-->
      <!--TODO: Add inputs for start and end to give more control over partions-->
      <div class="flex space-x-2">
        <InputBox
          givenOnChangeValue={ResizingPartitionOnChangeValue}
          value={bytesToGB(selectedPartitionForAction.size)}
          placeholderText="Enter Value"
          label="Resized Storage"
          rightLabel="GB"
          inputType="number"
          styleClass="w-1/2"
        />
      </div>
    {/if}
  </div>
  <div class="flex justify-between mt-4">
    <div class="w-40">
      <Button
        variant="bordered"
        on:click={() => dialogEditPartition.close()}
        fullWidth>Cancel</Button
      >
    </div>
    <div class="w-40">
      <Button
        on:click={() => {
          updateInformation();
          gatherInfo();
          changeAllowCreation();
          dialogEditPartition.close();
        }}
        fullWidth>Confirm</Button
      >
    </div>
  </div>
</Dialog>

<Dialog dialog={dialogDeletePartition} dialogTitle="Delete Partition">
  <div class="p-8 space-y-6 flex items-center flex-col">
    <img src={warningIcon} alt="" />
    <div class="text-center text-2xl font-medium">
      Confirm Delete of "{selectedPartitionForAction.name}" of size "{bytesToMB(
        selectedPartitionForAction.size,
      )} MB"
    </div>
    <div class="text-red-500">This action is irreversable</div>
  </div>
  <div class="flex justify-between pt-8">
    <div class="w-40">
      <Button
        variant="bordered"
        on:click={() => dialogDeletePartition.close()}
        fullWidth>Cancel</Button
      >
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="w-40"
      on:click={() => {
        // Update current information as delete. This will be used for delete action.
        // Delete can only happen on partitions that only existed
        if (
          $partitionStore.systemStorageInfoCurrent
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (item) => item.name == selectedPartitionForAction.name,
            )[0] != undefined
        ) {
          $partitionStore.systemStorageInfoCurrent
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (item) => item.name == selectedPartitionForAction.name,
            )[0].action = "delete";
        }
        // Check if the next partition is a free space
        let index = $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.findIndex(
            (partition) =>
              partition.partitionName ===
              selectedPartitionForAction.partitionName,
          );
        if (
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].partitions[index + 1] != undefined && // Exists (Not the last)
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions[index + 1].name.includes("free") // Is a free space
        ) {
          // Check if the partition before this partition is a free space
          if (
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index - 1] != undefined && // Exists (Not the first)
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions[index - 1].name.includes("free") // Is a free space
          ) {
            // Update the start and the size of the free space
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index + 1].size +=
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index].size +
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index - 1].size;
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index + 1].start =
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index - 1].start;
            // Delete the partition before this one
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.splice(index, 1);
            // Delete the current partition
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.splice(index - 1, 1);
          } else {
            // Update the start and the size of the free space
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index + 1].size +=
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index].size;
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index + 1].start =
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index].start;
            // Delete the current partition
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.splice(index, 1);
          }
        } else {
          // Check if the partition before this partition is a free space
          if (
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index - 1] != undefined && // Exists (Not the first)
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions[index - 1].name.includes("free") // Is a free space
          ) {
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index - 1].size +=
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index].size;
            $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[index - 1].end =
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[index].end;
            // Delete the current partition
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.splice(index, 1);
          } else {
            // Instead of deleting this partition, convert it to a free space
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.filter(
                (item) =>
                  item.size == selectedPartitionForAction.size &&
                  item.name == selectedPartitionForAction.name,
              )[0].name = "free";
            $partitionStore.systemStorageInfo
              .filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0]
              .partitions.filter(
                (item) =>
                  item.size == selectedPartitionForAction.size &&
                  item.name == selectedPartitionForAction.name,
              )[0].partitionName = "free-space";
          }
        }

        // After deleting, if a partition had been created, its part number is now a number less.
        // This is important for config file
        $partitionStore.systemStorageInfo.filter(
          (item) => item.displayName === $partitionStore.selectedDevice,
        )[0].availableStorage =
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].availableStorage + selectedPartitionForAction.size;

        gatherInfo();

        dialogDeletePartition.close();
      }}
    >
      <Button fullWidth>Confirm</Button>
    </div>
  </div>
</Dialog>

<StepWrapper
  title="Configure Partition"
  dialogTitle="Edit or create partition table"
  dialogContent="In this page, you edit an existing partition table or create a new partition table"
  prev="partition"
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
      <div style="color:red">
        You must select boot and athenaOS root partitions. If you cant find the
        partitions you would like to use, please consider using gparted or
        create new partitions or edit.
      </div>
      <h3 class="font-semibold mb-2 text-[#B0B0B0]">New Partition Table</h3>
      <div
        class="rounded-2xl overflow-hidden bg-[#1A1A1A] border-2 border-[#2F2F2F]"
      >
        <div class="max-h-[18.3em] overflow-auto">
          {#if partitionData.length > 0}
            <table class="min-w-full w-full">
              <thead class="bg-[#363636] sticky top-0">
                <tr>
                  <th class="w-1/6 text-left p-3">Block Device</th>
                  <th class="text-left p-3">Name</th>
                  <th class="text-left p-3">Type</th>
                  <th class="text-left p-3">File System</th>
                  <th class="text-left p-3">Mount Point</th>
                  <th class="text-left p-3">Start</th>
                  <th class="text-left p-3">End</th>
                  <th class="text-left p-3">Size</th>
                  <th class="text-right p-3 pr-9">Actions</th>
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
                    <td class="text-[#B0B0B0] p-3">{row.name}</td>
                    <td class="text-[#B0B0B0] p-3">{row.name}</td>
                    <td class="text-[#B0B0B0] p-3">{row.fileSystem}</td>
                    <td class="text-[#B0B0B0] p-3">{row.mountPoint}</td>
                    <td class="text-[#B0B0B0] p-3">{row.start}</td>
                    <td class="text-[#B0B0B0] p-3">{row.end}</td>
                    <td class="text-[#B0B0B0] font-semibold p-3"
                      >{bytesToMB(parseInt(row.size))} MB / {bytesToGB(
                        parseInt(row.size),
                      )} GB</td
                    >
                    <td class="py-2 text-right p-3 pr-9">
                      {#if row.name.includes("free")}
                        <!--For free partitions, we can create new partitions in that position-->

                        <Button
                          variant="secondary"
                          on:click={() => {
                            selectedPartitionForAction = {
                              ...row,
                            };
                            $partitionStore.ind =
                              $partitionStore.systemStorageInfo
                                .filter(
                                  (item) =>
                                    item.displayName ===
                                    $partitionStore.selectedDevice,
                                )[0]
                                .partitions.findIndex(
                                  (partition) =>
                                    partition.partitionName ===
                                    selectedPartitionForAction.partitionName,
                                );
                            console.log($partitionStore);
                            console.log(
                              $partitionStore.systemStorageInfo.filter(
                                (item) =>
                                  item.displayName ===
                                  $partitionStore.selectedDevice,
                              )[0],
                            );
                            dialogNewPartition.open();
                          }}
                          ><img src={plusWhiteIcon} alt="" />
                          <span>Create Partition</span></Button
                        >
                      {:else}
                        <button
                          class="mr-2"
                          on:click={() => {
                            selectedPartitionForAction = {
                              ...row,
                            };
                            dialogEditPartition.open();
                          }}
                        >
                          <img src={editGrayIcon} alt="edit" />
                        </button>
                        <button
                          on:click={() => {
                            selectedPartitionForAction = row;
                            dialogDeletePartition.open();
                          }}
                        >
                          <img src={binGrayIcon} alt="delete" />
                        </button>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>
    </div>
    <div class="flex-colum w-full justify-end space-x-4">
      <div class="flex w-full justify-end space-x-4">
        <Button variant="bordered" on:click={dialogBootPartition.open}>
          <img class="h-6" src={replaceIcon} alt="" />
          <span>Select boot partition</span></Button
        >
        <Button variant="bordered" on:click={dialogReplacePartition.open}>
          <img class="h-6" src={replaceIcon} alt="" />
          <span>Replace with Athena OS</span></Button
        >
      </div>
      <div class="flex w-full justify-center space-x-4">
        <Button variant="secondary" on:click={dialogNewPartitionTable.open}
          ><img src={plusWhiteIcon} alt="" />
          <span>Create New Partion Table</span></Button
        >
      </div>
    </div>
  </div>
</StepWrapper>
