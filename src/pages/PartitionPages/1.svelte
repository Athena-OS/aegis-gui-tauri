<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import diskIcon from "../../assets/icons/disk.svg";
  import eraseDiskIcon from "../../assets/icons/erase-disk.svg";
  import manualDiskIcon from "../../assets/icons/manual-disk.svg";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import CardGroup from "../../lib/components/CardGroup.svelte";

  let partitionList = [
    { name: "Samsung NVME SSD 500G" },
    // ... other names
  ];

  let selectedDrive: { name: string } | null = null;
  let selectedOption: null | string = null;

  $: canContinue =
    selectedDrive !== null &&
    selectedDrive.name !== "Select Drive" &&
    selectedOption !== null;

  function handleSelect(event: CustomEvent<any>) {
    console.log("Selected item:", event.detail);
    selectedDrive = event.detail;
  }

  function handleOptionSelect(option: string) {
    selectedOption = option;
  }
</script>

<StepWrapper
  title="Partition"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="packages"
  next="configure-partition"
>
  <div class="flex flex-col items-center gap-10 w-full max-w-md mx-auto">
    <Dropdown
      bind:items={partitionList}
      icon={diskIcon}
      label="Select Drive"
      on:select={handleSelect}
      defaultItem={{ name: "Select Drive" }}
    />
    <CardGroup
      title="Method of Partition"
      cards={[
        {
          title: "Automatic",
          desc: "Wipe everything on drive.",
          value: "auto",
          icon: eraseDiskIcon,
          checked: true,
        },
        {
          title: "Manual",
          desc: "",
          value: "manual",
          icon: manualDiskIcon,
        },
      ]}
    />
  </div>
</StepWrapper>

<style>
  .selected {
    border-color: #ffb800;
  }
</style>
