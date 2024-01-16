<script>
  import { MBtoBytes, bytesToGB, bytesToMB } from "../../utils/functions";
  import partitionStore from "../../stores/partitionStore";

  import Dropdown from "../Dropdown.svelte";
  import InputBox from "../InputBox.svelte";
  import Slider from "../Slider.svelte";
  import Switch from "../Switch.svelte";

  let fileSystemList = [
    { name: "vfat" },
    { name: "bfs" },
    { name: "cramfs" },
    { name: "ext3" },
    { name: "fat" },
    { name: "msdos" },
    { name: "xfs" },
    { name: "btrfs" },
    { name: "ext2" },
    { name: "ext4" },
    { name: "minix" },
    { name: "f2fs" },
    { name: "don't format" },
    { name: "noformat" },
  ];
  let mountPointList = [
    { name: "/boot/start" },
    { name: "/etc/start" },
    { name: "/groot/start" },
  ];

  let newPartitionSize = 1024;
</script>

<div class="space-y-6">
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
  <div class="flex space-x-2">
    <InputBox
      value={$partitionStore.newPartition.size.toString()}
      label="Partition Size"
      rightLabel="MB"
      inputType="number"
      isDisabled={true}
      styleClass="w-1/2"
    />
    <InputBox
      value={bytesToGB(
        MBtoBytes($partitionStore.newPartition.size),
      ).toString()}
      label="‎"
      rightLabel="GB"
      inputType="number"
      isDisabled={true}
      styleClass="w-1/2"
    />
  </div>
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

  <div class="flex space-x-2">
    <Switch bind:value={$partitionStore.newPartition.isEncrypted}></Switch>
    <h4>Encrypted</h4>
  </div>
</div>
