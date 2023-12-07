<script>
  import Dropdown from "../Dropdown.svelte";
  import InputBox from "../InputBox.svelte";
  import Slider from "../Slider.svelte";

  let fileSystemList = [{ name: "Fat" }, { name: "exFat" }, { name: "NIFT" }];
  let mountPointList = [
    { name: "/boot/start" },
    { name: "/etc/start" },
    { name: "/groot/start" },
  ];
</script>

<div class="p-8 space-y-6">
  <Slider />
  <InputBox
    placeholderText="Enter Partition Size"
    label="Partition Size"
    rightLabel="MB"
  />
  <InputBox placeholderText="Enter Partition Label" label="Partition Label" />
  <div class="flex space-x-4">
    <Dropdown
      bind:items={fileSystemList}
      label="File System"
      defaultItem={{ name: "Select Region" }}
    />
    <Dropdown
      bind:items={mountPointList}
      label="Mount Point"
      defaultItem={{ name: "Select Region" }}
    />
  </div>
</div>
