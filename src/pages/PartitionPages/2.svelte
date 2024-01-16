<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import replaceIcon from "../../assets/icons/replace-yellow.svg";
  import editGrayIcon from "../../assets/icons/edit-gray.svg";
  import binGrayIcon from "../../assets/icons/bin-gray.svg";
  import plusWhiteIcon from "../../assets/icons/plus-white.svg";
  import warningIcon from "../../assets/icons/warning.svg";
  import diskIcon from "../../assets/icons/disk.svg";

  import { createDialog } from "svelte-headlessui";

  import { MBtoBytes, bytesToGB, bytesToMB } from "../../lib/utils/functions";
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

  function gatherInfo() {
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

  // create new partition
  let dialogNewPartition = createDialog({ label: "create-partition" });
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
</script>

<!-- create new partition -->
<CreatePartitionDialog dialog={dialogNewPartition} />

<Dialog dialog={dialogReplacePartition}>
  <div class="w-full h-72 space-y-4">
    <h4 class="text-2xl my-4 font-meidum">Replace Partition</h4>
    <Dropdown
      items={[
        ...partitionData.map((partition) => {
          return { name: partition.partitionName, selected: false };
        }),
      ]}
      icon={diskIcon}
      label="Select Partition"
      on:select={(event) =>
        ($partitionStore.replacedPartition.partitionName =
          event.detail.selected.name)}
      defaultItem={{ name: "Select Partition" }}
    />
  </div>
  <div class="flex justify-between space-x-2 pt-8">
    <Button
      variant="bordered"
      on:click={() => dialogReplacePartition.close()}
      fullWidth>Cancel</Button
    >
    <Button
      on:click={() => {
        console.log($partitionStore.replacedPartition);
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
    <Slider
      max={parseFloat(bytesToMB(selectedPartitionForAction.size))}
      min={parseFloat(bytesToMB(selectedPartitionForAction.size)) -
        parseFloat(bytesToMB(selectedPartitionForAction.availableStorage))}
      bind:value={selectedPartitionForAction.newSize}
    />
    {#if selectedPartitionForAction.newSize !== undefined}
      <div class="flex text-xl justify-between">
        <h4>
          Used: {bytesToGB(
            parseFloat(selectedPartitionForAction.size) -
              parseFloat(selectedPartitionForAction.availableStorage),
          )} GB
        </h4>
        <h4>
          Available: {(
            parseFloat(bytesToGB(selectedPartitionForAction.size)) -
            parseFloat(bytesToGB(MBtoBytes(selectedPartitionForAction.newSize)))
          ).toFixed(2)} GB
        </h4>
      </div>
      <div class="flex space-x-2">
        <InputBox
          value={selectedPartitionForAction.newSize}
          placeholderText="Enter Value"
          label="Resized Storage"
          rightLabel="MB"
          inputType="number"
          isDisabled={true}
          styleClass="w-1/2"
        />
        <InputBox
          value={bytesToGB(
            MBtoBytes(selectedPartitionForAction.newSize),
          ).toString()}
          placeholderText="Enter Value"
          label="‎"
          rightLabel="GB"
          inputType="number"
          isDisabled={true}
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
            )[0].size = MBtoBytes(selectedPartitionForAction.newSize);

          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.filter(
              (partition) =>
                partition.partitionName ===
                  selectedPartitionForAction.partitionName &&
                partition.name === selectedPartitionForAction.name,
            )[0].availableStorage =
            selectedPartitionForAction.size -
            MBtoBytes(selectedPartitionForAction.newSize);

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
        $partitionStore.systemStorageInfo.filter(
          (item) => item.displayName === $partitionStore.selectedDevice,
        )[0].partitions = $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.filter(
            (item) =>
              item.size !== selectedPartitionForAction.size &&
              item.name !== selectedPartitionForAction.name,
          );

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
                            newSize: bytesToMB(row.size - row.availableStorage),
                          };

                          console.log(selectedPartitionForAction);
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
    <div class="flex w-full justify-end space-x-4">
      {#if parseInt(bytesToGB($partitionStore.systemStorageInfo.filter((item) => item.displayName === $partitionStore.selectedDevice)[0].availableStorage)) > 1}
        <Button variant="secondary" on:click={dialogNewPartition.open}
          ><img src={plusWhiteIcon} alt="" />
          <span>Create Partition</span></Button
        >
      {/if}
      <Button variant="bordered" on:click={dialogReplacePartition.open}>
        <img class="h-6" src={replaceIcon} alt="" />
        <span>Replace</span></Button
      >
    </div>
  </div>
</StepWrapper>
