<script>
  import { bytesToMB } from "../../utils/functions";
  import partitionStore from "../../stores/partitionStore";

  import Dropdown from "../Dropdown.svelte";
  import InputBox from "../InputBox.svelte";
  import Slider from "../Slider.svelte";

  let fileSystemList = [{ name: "Fat" }, { name: "exFat" }, { name: "NIFT" }];
  let mountPointList = [
    { name: "/boot/start" },
    { name: "/etc/start" },
    { name: "/groot/start" },
  ];

  let newPartitionSize = 1024;
</script>

<div class="p-8 space-y-6">
  <Slider
    max={parseInt(
      bytesToMB(
        $partitionStore.systemStorageInfo.filter(
          (item) => item.displayName === $partitionStore.selectedDevice,
        )[0].availableStorage,
      ),
    )}
    min={1024}
    bind:value={$partitionStore.newPartition.size}
  />
  <InputBox
    placeholderText="Enter Partition Size"
    label="Partition Size"
    rightLabel="MB"
    inputType="number"
    isDisabled={true}
    value={$partitionStore.newPartition.size.toString()}
  />
  <InputBox
    bind:value={$partitionStore.newPartition.name}
    placeholderText="Enter Partition Label"
    label="Partition Label"
  />
  <div class="flex space-x-4">
    <Dropdown
      on:select={(e) => {
        $partitionStore.newPartition.fileSystem = e.detail.selected.name;
      }}
      bind:items={fileSystemList}
      label="File System"
      defaultItem={{ name: "Select Filesysteam" }}
    />
    <Dropdown
      on:select={(e) => {
        $partitionStore.newPartition.mountPoint = e.detail.selected.name;
      }}
      bind:items={mountPointList}
      label="Mount Point"
      defaultItem={{ name: "Select Mount Point" }}
    />
  </div>
</div>
