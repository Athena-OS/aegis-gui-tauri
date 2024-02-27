<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
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
    MBtoGB,
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
  console.log($partitionStore)
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



  gatherInfo();
  $: $partitionStore.systemStorageInfo, gatherInfo();
  $: $partitionStore.selectedDevice, gatherInfo();
  //$: $partitionStore.systemStorageInfo.filter((item) => item.displayName === $partitionStore.selectedDevice,)[0]?.availableStorage,    changeAllowCreation();

  function ResizingPartitionOnChangeValue(e: any) {
    let value = GBToMB(parseFloat(e.target.value));

    if (!isNaN(parseFloat(value))) {
      if (
        parseFloat(value) >=
          parseFloat(
            bytesToMB(
              $partitionStore.systemStorageInfoCurrent
                .filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0]
                .partitions.filter(
                  (partition) =>
                    partition.partitionName ===
                      selectedPartitionForAction.partitionName &&
                    partition.name === selectedPartitionForAction.name,
                )[0].size,
            ),
          ) &&
        parseFloat(value) <=
          parseFloat(
            bytesToMB(
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].totalStorage,
            ),
          ) -
            parseFloat(
              bytesToMB(
                $partitionStore.systemStorageInfo
                  .filter(
                    (item) =>
                      item.displayName === $partitionStore.selectedDevice,
                  )[0]
                  .partitions.reduce((accumulator, partition) => {
                    if (partition.name !== selectedPartitionForAction.name) {
                      return accumulator + partition.size;
                    } else {
                      return accumulator;
                    }
                  }, 0),
              ),
            )
      ) {
        e.target.parentElement.classList.remove("border-red-500");
        selectedPartitionForAction.size = MBtoBytes(parseFloat(value));
      } else {
        e.target.parentElement.classList.add("border-red-500");
      }
    } else {
      e.target.parentElement.classList.add("border-red-500");
    }
  }

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
</script>
<!--Dialog dialog={dialogEditPartition}>
  <div class="w-full h-fit space-y-4 py-4">
    <h4 class="text-2xl font-meidum">Resize Partition</h4>
    <div class="text-center">
      Resizing {selectedPartitionForAction.partitionName} ({bytesToGB(
        selectedPartitionForAction.size,
      )} GB {selectedPartitionForAction.fileSystem}: {selectedPartitionForAction.name})
    </div>
    <SegementedBar
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
    />
    {#if selectedPartitionForAction.size !== undefined}
      <div class="text-xl justify-between">
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
      </div>
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
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  selectedPartitionForAction.partitionName &&
                partition.name === selectedPartitionForAction.name,
            )[0].size = selectedPartitionForAction.size;

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

          gatherInfo();
          dialogEditPartition.close();
        }}
        fullWidth>Confirm</Button
      >
    </div>
  </div>
</Dialog-->


<StepWrapper
  title="Configure Partition for install along"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="partition"
  next="finalize-partition"
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
      <!--div class="w-full">
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
      </div-->
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
                    <td class="py-2 text-right p-3 pr-9">
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
                    </td>
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
