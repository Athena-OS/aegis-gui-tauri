<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import editGrayIcon from "../../assets/icons/edit-gray.svg";
  import binGrayIcon from "../../assets/icons/bin-gray.svg";
  import plusWhiteIcon from "../../assets/icons/plus-white.svg";
  import warningIcon from "../../assets/icons/warning.svg";

  import diskIcon from "../../assets/icons/disk.svg";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import Button from "../../lib/components/Button.svelte";
  import { createDialog } from "svelte-headlessui";
  import Dialog from "../../lib/components/Dialog.svelte";
  import Slider from "../../lib/components/Slider.svelte";
  import InputBox from "../../lib/components/InputBox.svelte";

  let partitionList = [{ name: "Samsung NVME SSD 500G" }];

  let partitionData = [
    {
      device: "/dev/nvme/0n1p1",
      name: "EFI System Partition",
      fileSystem: "FAT32",
      mountPoint: "/boot/efi",
      size: "273 MB",
      sizeInMB: 273,
    },
    {
      device: "/dev/nvme/0n1p2",
      name: "Microsoft Reserved Partition",
      fileSystem: "Unknown",
      mountPoint: "",
      size: "17 MB",
      sizeInMB: 17,
    },
    {
      device: "/dev/nvme/0n1p3",
      name: "Basic Data Partition",
      fileSystem: "NTFS",
      mountPoint: "",
      size: "168 GB",
      sizeInMB: 168000,
    },
    {
      device: "/dev/nvme/0n1p4",
      name: "Grubby",
      fileSystem: "exFat",
      mountPoint: "/boot/grub",
      size: "1.1 GB",
      sizeInMB: 1100,
    },
    {
      device: "/dev/nvme/0n1p5",
      name: "Athena OS",
      fileSystem: "Btrfs",
      mountPoint: "/",
      size: "85 GB",
      sizeInMB: 85000,
    },
  ];

  const colorList = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];

  partitionData.forEach((partition, index) => {
    partition.color = colorList[index % colorList.length];
  });

  const totalSizeInMB = 500 * 1024;
  const usedSizeInMB = partitionData.reduce(
    (total, partition) => total + partition.sizeInMB,
    0
  );
  const unusedSizeInMB = totalSizeInMB - usedSizeInMB;

  function handleSelect(event: CustomEvent<any>) {
    console.log("Selected item:", event.detail);
  }

  let selectedOption: null | string = null;

  function handleOptionSelect(option: string) {
    selectedOption = option;
  }

  // create new partition
  let dialogNewPartition = createDialog({ label: "create-partition" });

  // edit existing partition
  let dialogEditPartition = createDialog({ label: "edit-partition" });

  // delete existing partition
  let dialogDeletePartition = createDialog({ label: "delete-partition" });

  const handleResizePartition = () => {
    dialogEditPartition.open();
  };

  const handleDeletePartition = () => {
    dialogDeletePartition.open();
  };
</script>

<!-- create new partition -->

<Dialog dialog={dialogNewPartition} dialogTitle="Create a new partition table">
  <fieldset class="flex space-x-4 pt-6">
    <div class="w-full">
      <label
        class="relative px-3 py-2 w-full flex items-center ring ring-primary-500 rounded-xl"
        for="mbr"
        ><div>
          <div class="text-4xl font-medium text-center p-6">MBR</div>
          <div class="text-xs">
            For legacy operating systems. Ex: Windows XP
          </div>
        </div>
        <input
          class="absolute top-2 right-2"
          type="radio"
          name="format"
          id="mbr"
          value="mbr"
          checked
        /></label
      >
    </div>
    <div class="w-full">
      <label class="relative p-2 w-full flex items-center" for="gpt"
        ><div>
          <div class="text-4xl font-medium text-center p-6">GPT</div>
          <div class="text-xs">Recommended in most cases.</div>
        </div>
        <input
          class="absolute top-2 right-2"
          type="radio"
          name="format"
          id="gpt"
          value="gpt"
          checked
        /></label
      >
    </div>
  </fieldset>
  <div class="flex justify-between pt-8 space-x-20">
    <Button variant="bordered" fullWidth>Cancel</Button>
    <Button fullWidth>Confirm</Button>
  </div>
</Dialog>

<Dialog dialog={dialogEditPartition} dialogTitle="Resize Partition">
  <div class="p-8 space-y-6">
    <div class="text-center">
      Resizing /dev/nvme0n1p7 (85 GB Btrfs: AthenaOS)
    </div>
    <Slider />
    <InputBox placeholderText="Enter Value" label="Size" rightLabel="MB" />
  </div>
  <div class="flex justify-between pt-8">
    <div class="w-40">
      <Button variant="bordered" fullWidth>Cancel</Button>
    </div>
    <div class="w-40">
      <Button fullWidth>Confirm</Button>
    </div>
  </div>
</Dialog>

<Dialog dialog={dialogDeletePartition} dialogTitle="Delete Partition">
  <div class="p-8 space-y-6 flex items-center flex-col">
    <img src={warningIcon} alt="" />
    <div class="text-center text-2xl font-medium">
      Confirm Delete of "EFI System Partition" of size "273 MB"
    </div>
    <div class="text-red-500">This action is irreversable</div>
  </div>
  <div class="flex justify-between pt-8">
    <div class="w-40">
      <Button variant="bordered" fullWidth>Cancel</Button>
    </div>
    <div class="w-40">
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
      <Dropdown
        bind:items={partitionList}
        icon={diskIcon}
        width="30em"
        label="Select Drive"
        on:select={handleSelect}
        additionalIcons={[refreshIcon]}
        defaultItem={{ name: "Select Drive" }}
        fullWidth={false}
      />
      <div class="w-full">
        <p class="text-[#B0B0B0] text-left font-semibold mb-2">Partition</p>
        <div
          class="flex flex-row items-center justify-center rounded-full bg-[#1A1A1A] border-2 border-[#2F2F2F] w-full h-[50px] overflow-hidden"
        >
          <div class="flex-grow h-full bg-red-500" />
          <div class="w-[2px] h-full bg-[#2F2F2F]" />
          <div class="flex-grow h-full bg-green-500" />
          <div class="w-[2px] h-full bg-[#2F2F2F]" />
          <div class="flex-grow h-full bg-blue-500" />
        </div>
      </div>
    </div>
    <div class="w-full">
      <h3 class="font-semibold mb-2 text-[#B0B0B0]">New Partition Table</h3>
      <div
        class="rounded-2xl overflow-hidden bg-[#1A1A1A] border-2 border-[#2F2F2F]"
      >
        <div class="max-h-[18.3em] overflow-auto">
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
                    <div class="rounded-full bg-[#FF5353] w-3 h-3" />
                    {row.device}
                  </td>
                  <td class="text-[#B0B0B0] p-3">{row.name}</td>
                  <td class="text-[#B0B0B0] p-3">{row.fileSystem}</td>
                  <td class="text-[#B0B0B0] p-3">{row.mountPoint}</td>
                  <td class="text-[#B0B0B0] font-semibold p-3">{row.size}</td>
                  <td class="py-2 text-right p-3 pr-9">
                    <button class="mr-2" on:click={handleResizePartition}>
                      <img src={editGrayIcon} alt="edit" />
                    </button>
                    <button on:click={handleDeletePartition}>
                      <img src={binGrayIcon} alt="delete" />
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
    <div class="flex w-full justify-end">
      <Button variant="secondary" on:click={dialogNewPartition.open}
        ><img src={plusWhiteIcon} alt="" />
        <span>Create Partition</span></Button
      >
    </div>
  </div>
</StepWrapper>
