<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import diskIcon from "../../assets/icons/disk.svg";
  import eraseDiskIcon from "../../assets/icons/erase-disk.svg";
  import manualDiskIcon from "../../assets/icons/manual-disk.svg";

  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import CardGroup from "../../lib/components/CardGroup.svelte";
  import Button from "../../lib/components/Button.svelte";
  import partitionStore from "../../lib/stores/partitionStore";

  import { disks } from "tauri-plugin-system-info-api";
  import { invoke } from "@tauri-apps/api";

  import { type StorageDevice } from "../../lib/utils/types";
  import { bytesToGB } from "../../lib/utils/functions";
  import { resetPartitionStore } from "../../lib/stores/partitionStore";
  
  $partitionStore.mode = "auto";
  let storageDevicesList: any[] = [];

  async function fetchAndParseStorageInfo() {
    resetPartitionStore()
    let sysInfo_Disks = await disks();
    console.log(sysInfo_Disks)
    /*sysInfo_Disks.forEach(item => {
      console.log(item.total_space)
      storageDevicesList.push({"name":item.kind})
    })*/
    console.log(storageDevicesList)
    invoke("is_uefi").then((p: any) => {
      if (p.trim()==="true"){
        $partitionStore.efi = true
        $partitionStore.grubLocation = "/boot"
        $partitionStore.grubType ="grub-efi"
      }else{
        $partitionStore.efi = false
        $partitionStore.grubType ="grub-efi"
      }
    })
    invoke("get_partitions").then((partitions) => {
      console.log(JSON.parse(partitions as string))
      let p = JSON.parse(partitions as string)?.blockdevices
      for (let i=0; i<p.length;i++){
        let disk: StorageDevice = {
        diskModel:p[i].model,
        logicalName: p[i].model,
        displayName:p[i].kname,
        totalStorage:p[i].size,
        availableStorage:0,
        disklabelType:p[i].pptype,
        kind:"",
        isRemovable:p[i].rm,
        partitions:[]
      }
      let children = []
      if (p[i].children != undefined) {
        for (let j=0; j<p[i].children.length; j++) {
          let part = p[i].children[j]
          children.push({
            partitionName: part.name,
            size: part.size,
            fileSystem: part.parttypename,
            mountPoint: part.mountpoint,
            availableStorage:part.fsavail,
            name:part.kname,
          })
          storageDevicesList.push({
            name: part.kname,
            size:"20"
          });
        }
      }
      disk.partitions = children
      console.log(disk)
      let temp_disk_data = JSON.parse(JSON.stringify(disk));
          $partitionStore.systemStorageInfoCurrent.push({...temp_disk_data});
          $partitionStore.systemStorageInfo.push(disk);

          storageDevicesList.push({
            name: disk.diskModel,
            size:"20"
          });
      }
      
    })
    
  }

  fetchAndParseStorageInfo();

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    if ($partitionStore.selectedDevice !== "default") {
      if ($partitionStore.mode === "auto") {
        nextPage = "/summary";
      } else {
        nextPage = "/configure-partition";
      }
    }
  }

  $: $partitionStore, IsOkayToMoveNextPage();
</script>

<StepWrapper
  title="Partition"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/extras"
  next={nextPage}
>
  <div class="flex flex-col items-center gap-10 w-full max-w-md mx-auto">
    <div class="flex space-x-2 items-end w-full">
      <Dropdown
        bind:items={storageDevicesList}
        icon={diskIcon}
        label="Select Drive"
        on:select={(event) =>
          ($partitionStore.selectedDevice = event.detail.selected.name)}
        defaultItem={{ name: "Select Drive" }}
      />
      <!-- <Button on:click={fetchAndParseStorageInfo}
        ><img src={refreshIcon} alt="" srcset="" /></Button
      > -->
    </div>
    <CardGroup
      title="How do you want to partition ?"
      on:change={(event) => ($partitionStore.mode = event.detail.target.value)}
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
      ]}
    />
  </div>
</StepWrapper>

<style>
  .selected {
    border-color: #ffb800;
  }
</style>
