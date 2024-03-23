<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import diskIcon from "../../assets/icons/disk.svg";
  import eraseDiskIcon from "../../assets/icons/erase-disk.svg";
  import manualDiskIcon from "../../assets/icons/manual-disk.svg";
  import installAlongIcon from "../../assets/icons/wrench-yellow.svg"
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown2 from "../../lib/components/Dropdown2.svelte";
  import CardGroup from "../../lib/components/CardGroup.svelte";
  import Button from "../../lib/components/Button.svelte";
  import partitionStore from "../../lib/stores/partitionStore";
  import replacePartitionIcon from "../../assets/icons/replace-yellow.svg"
  import { disks } from "tauri-plugin-system-info-api";
  import { invoke } from "@tauri-apps/api";

  import {
    type StorageDevice,
    type InstallAlongPartition,
  } from "../../lib/utils/types";
  import { bytesToGB } from "../../lib/utils/functions";
  import { resetPartitionStore } from "../../lib/stores/partitionStore";
  interface Card {
    title: string;
    desc: string;
    value: string;
    icon?: string;
    checked?: boolean;
  }

  let install_along_card: Card = {
    title: "install along",
    desc: "Install athena OS alongside ",
    value: "iA",

    checked: false,
  };
  $partitionStore.mode = "auto";
  let storageDevicesList: any[] = [];
  let hasOs = false;
  async function fetchAndParseStorageInfo() {
    resetPartitionStore();
    let sysInfo_Disks = await disks();
    let gs: string = await invoke("get_gs");
    let devices = JSON.parse(gs).devices;
    console.log(devices)
    devices.map((item: any) => {
      if (item.install_candidate) {
        storageDevicesList.push({
          display_name: item.id,
          size: item.size,
          percentage_used: item.use_percentage,
          name: item.kname,
        });
      }
      item.partitions.map((i: any) => {
        /*storageDevicesList.push({
          display_name: i.id,
          size: i.size,
          percentage_used: i.use_percentage,
          name: i.kname,
        });*/
        if (i.can_install_along) {
          $partitionStore.partitionsWithOS.push(i)
        }
      });
    });
    let os = JSON.parse(gs).operating_systems;
    if (os.length > 0) {
      hasOs = true;
      const bValues = os.map((i: any) => i.raw.split(":")[1]);
      if (bValues.length > 1) {
        bValues[bValues.length - 1] = "and " + bValues[bValues.length - 1];
      }

      const result = bValues.join(", ");
      install_along_card.desc += result;
    }
    invoke("is_uefi").then((p: any) => {
      if (p.trim() === "true") {
        $partitionStore.efi = true;
        $partitionStore.grubLocation = "/boot";
        $partitionStore.grubType = "grub-efi";
      } else {
        $partitionStore.efi = false;
        $partitionStore.grubType = "grub-efi";
      }
    });
    invoke("get_partitions").then((partitions) => {
      let p = JSON.parse(partitions as string)?.blockdevices;
      for (let i = 0; i < p.length; i++) {
        let disk: StorageDevice = {
          diskModel: p[i].model,
          logicalName: p[i].model,
          displayName: p[i].kname,
          totalStorage: p[i].size,
          availableStorage: 0,
          disklabelType: p[i].pptype,
          kind: "",
          isRemovable: p[i].rm,
          partitions: [],
        };
        let children = [];
        if (p[i].children != undefined) {
          for (let j = 0; j < p[i].children.length; j++) {
            let part = p[i].children[j];
            children.push({
              partitionName: part.name,
              size: part.size,
              fileSystem: part.parttypename,
              mountPoint: part.mountpoint,
              availableStorage: part.fsavail,
              name: part.kname,
            });
          }
        }
        disk.partitions = children;
        let temp_disk_data = JSON.parse(JSON.stringify(disk));
        $partitionStore.systemStorageInfoCurrent.push({ ...temp_disk_data });
        $partitionStore.systemStorageInfo.push(disk);
      }
    });
  }

  fetchAndParseStorageInfo();

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    if ($partitionStore.selectedDevice !== "default") {
      if ($partitionStore.mode === "auto") {
        nextPage = "/summary";
      }else if ($partitionStore.mode === "replace-partition"){
        nextPage = "/replace-partition";
      } else {
        nextPage = "/configure-partition";
      }
    } else if ($partitionStore.mode === "install-along") {
      nextPage = "/configure-install-along";
    }
  }
  function handleChange(event: Event) {
    $partitionStore.mode = "install-along";
    $partitionStore.selectedDeviceForInstallAlong = $partitionStore.partitionsWithOS[0].kname;
  }

  $: $partitionStore, IsOkayToMoveNextPage();
</script>

<StepWrapper
  title="Partition"
  dialogTitle="Partitions page"
  dialogContent="This page allows you to select where to install your operating system. There are 4 modes
    There is Auto mode where the whole device selected is used for installation, there is manual mode where you are allowed to manually partition a disk,
    install along mode that allows you to install Athena OS alongside a partition that has another OS, and lastly the replace partion mode where you replace 
    the content of an existing partition with Athena OS."
  prev="/extras"
  next={nextPage}
>
  <div class="flex flex-col items-center gap-10 w-full max-w-md mx-auto">
    <div class="flex space-x-2 items-end w-full">
      <Dropdown2
        bind:items={storageDevicesList}
        icon={diskIcon}
        label="Select Drive"
        on:select={(event) =>
          ($partitionStore.selectedDevice = event.detail.selected.name)}
        defaultItem={{
          name: "Select Drive or partition",
          size: "",
          percentage_used: "",
          display_name: "",
        }}
      />
      <!-- <Button on:click={fetchAndParseStorageInfo}
        ><img src={refreshIcon} alt="" srcset="" /></Button
      > -->
    </div>
    <CardGroup
      title="How do you want to partition ?"
      on:change={(event) => {$partitionStore.mode = event.detail.target.value; if ($partitionStore.mode == "install-along"){$partitionStore.selectedDeviceForInstallAlong = $partitionStore.partitionsWithOS[0].kname;}}}
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
          desc: "Divide the drive matually",
          value: "manual",
          icon: manualDiskIcon,
        },
        {
          title: "Replace Partition",
          desc: "Replace the content of an existing partition with athena OS",
          value: "replace-partition",
          icon: replacePartitionIcon,
        },
        {
          title: "Install Along",
          desc: install_along_card.desc,
          value: "install-along",
          icon: installAlongIcon,
          disabled: !hasOs,
        },
      ]}
    />
    <!--{#if hasOs}
      <div class="relative w-full h-150" style="height:150px">
        <input
          class="absolute top-2 right-2 radio-btn"
          id={install_along_card.value}
          type="radio"
          name="radio-group"
          value={install_along_card.value}
          checked={install_along_card.checked || false}
          on:change={handleChange}
        />
        <label
          class:aspect-square={install_along_card.icon}
          class="h-full w-full p-6 flex flex-col items-center ring ring-gray-700 radio-btn-label rounded-3xl relative"
          for={install_along_card.value}
        >
          <div>Install along</div>
          <div>{install_along_card.desc}</div>
        </label>
      </div>
    {/if}-->
    <!--div class="flex align-center">
    <div class="relative w-full h-150" style="height:150px">
      <label
        class:aspect-square={install_along_card.icon}
        class="h-full w-full p-6 flex flex-col items-center ring ring-gray-700 radio-btn-label rounded-3xl relative"
        for={install_along_card.value}
      >
        <div>Replace</div>
        <div>Select a partition to replace with AthenaOS</div>
      </label>
    </div>
    <div class="flex space-x-2 items-end w-full align-center" style="align-items:center; margin-left: 5px">
      <Dropdown2
        bind:items={storageDevicesList}
        icon={diskIcon}
        label="Select Partition"
        on:select={(event) =>
          ($partitionStore.selectedDevice = event.detail.selected.name)}
        defaultItem={{
          name: "Select Drive or partition",
          size: "",
          percentage_used: "",
          display_name: "",
        }}
      />
    </div>
    </div-->
  </div>
</StepWrapper>

<style>
  .selected {
    border-color: #ffb800;
  }

  .radio-btn {
    @apply appearance-none h-7 w-7 bg-neutral-500 rounded-full;
  }
  .radio-btn:checked {
    @apply bg-primary-500;
  }
  .radio-btn:checked::before {
    content: url(../../assets/icons/check-bg-yellow.svg);
  }
  .radio-btn:checked + .radio-btn-label {
    @apply ring-yellow-500;
  }
</style>
