<script lang="ts">
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import SegementedBar from "../../lib/components/SegementedBar.svelte";

  let partitionDisks = [{ name: "Samsung NVME SSD 500G" }];

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
      sizeInMB: 168,
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
      sizeInMB: 8500,
    },
  ];

  let partitionDataNew = [
    {
      device: "/dev/nvme/0n1p1",
      name: "This is New",
      fileSystem: "FAT32",
      mountPoint: "/boot/efi",
      size: "548 MB",
      sizeInMB: 548,
    },
    {
      device: "/dev/nvme/0n1p2",
      name: "Im new too",
      fileSystem: "Unknown",
      mountPoint: "",
      size: "1200 MB",
      sizeInMB: 1200,
    },
    {
      device: "/dev/nvme/0n1p3",
      name: "Im old",
      fileSystem: "NTFS",
      mountPoint: "",
      size: "168 GB",
      sizeInMB: 168,
    },
    {
      device: "/dev/nvme/0n1p4",
      name: "You Ask",
      fileSystem: "exFat",
      mountPoint: "/boot/grub",
      size: "7.1 GB",
      sizeInMB: 7100,
    },
    {
      device: "/dev/nvme/0n1p5",
      name: "Same Here",
      fileSystem: "Btrfs",
      mountPoint: "/",
      size: "2.5 GB",
      sizeInMB: 2500,
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
</script>

<StepWrapper
  title="Finalize Partition"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="configure-partition"
  next="accounts"
>
  <div class="space-y-10">
    <div class="flex flex-col items-center mx-5 h-fit space-y-6">
      <div class="text-left w-full text-2xl font-semibold">
        1. Current Partition
      </div>
      <SegementedBar totalValue={20000} items={partitionData} />
    </div>
    <div class="flex flex-col items-center mx-5 h-fit space-y-6">
      <div class="text-left w-full text-2xl font-semibold">
        2. New Partition
      </div>
      <SegementedBar totalValue={20000} items={partitionDataNew} />
    </div>
  </div>
</StepWrapper>
