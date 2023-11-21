<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import editGrayIcon from "../../assets/icons/edit-gray.svg";
  import binGrayIcon from "../../assets/icons/bin-gray.svg";

  import diskIcon from "../../assets/icons/disk.svg";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";

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
</script>

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
        on:select={handleSelect}
        additionalIcons={[refreshIcon]}
        defaultItem={{ name: "Select Drive" }}
        fullWidth={false}
      />
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

    <div class="w-full">
      <h3 class="font-semibold mb-4 text-[#B0B0B0]">New Partition Table</h3>

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
                    <button class="mr-2">
                      <img src={editGrayIcon} alt="edit" />
                    </button>
                    <button class="">
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
  </div>
</StepWrapper>
