<script lang="ts">
  import refreshIcon from "../../assets/icons/refresh.svg";
  import replaceIcon from "../../assets/icons/replace-yellow.svg";
  import editGrayIcon from "../../assets/icons/edit-gray.svg";
  import binGrayIcon from "../../assets/icons/bin-gray.svg";
  import plusWhiteIcon from "../../assets/icons/plus-white.svg";
  import warningIcon from "../../assets/icons/warning.svg";
  import diskIcon from "../../assets/icons/disk.svg";

  import { createDialog } from "svelte-headlessui";

  import {
    GBToMB,
    MBtoBytes,
    MBtoGB,
    bytesToGB,
    bytesToMB,
  } from "../../lib/utils/functions";
  import partitionStore from "../../lib/stores/partitionStore";

  import SegementedBar from "../../lib/components/SegementedBar2.svelte";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  import Button from "../../lib/components/Button.svelte";
  import Dialog from "../../lib/components/Dialog.svelte";
  import Slider from "../../lib/components/Slider.svelte";
  import InputBox from "../../lib/components/InputBox.svelte";
  import CardGroup from "../../lib/components/CardGroup.svelte";
  import CreatePartitionDialog from "../../lib/components/CreatePartition/CreatePartitionDialog.svelte";
  console.log($partitionStore);
  let storageDevicesList: any[] = [];
  let partitionData: any[] = [];
  let pl = $partitionStore.partitionsWithOS;
  $partitionStore.partitionsWithOS.map((i: any) => {
    storageDevicesList.push({
      name: i.kname,
      selected: false,
    });
  });
  //storageDevicesList[0].selected = true;
  const colorList = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];

  async function gatherInfo() {
    storageDevicesList = [];
    partitionData = [];
    //partitionData.push({name:"sda2", size:1024000000, available_storage: 5}, {name:"sda2", size:1024000000, available_storage: 5})
    $partitionStore.partitionsWithOS.map((i: any) => {
      storageDevicesList.push({
        name: i.kname,
        selected: i.kname === $partitionStore.selectedDevice ? true : false,
      });
    partitionData.push({name:"sda2", size:MBtoBytes(parseFloat(GBToMB(parseFloat(i.size)))), available_storage: 5})
    });
    console.log(partitionData)
  }

  gatherInfo();
  $: $partitionStore.systemStorageInfo, gatherInfo();
  $: $partitionStore.selectedDevice, gatherInfo();
 



</script>



<StepWrapper
  title="Configure Partition for install along"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="partition"
  next="finalize-partition"
>
  <div class="flex flex-col items-center mx-5 h-full space-y-6">
    <div class="flex flex-row items-center gap-4 w-full">
      <div class="w-full">
        <Dropdown
          bind:items={storageDevicesList}
          label="Select Drive"
          icon={diskIcon}
          on:select={(event) =>
            ($partitionStore.selectedDevice = event.detail.selected.name)}
          defaultItem={{ name: "Select Drive" }}
        />
      </div>
     
    </div>
    <div class="w-full h-350">
       <div class="w-full h-300">        
          <SegementedBar
                    
            items={partitionData}
          />
        
      </div>
  </div>
</StepWrapper>
