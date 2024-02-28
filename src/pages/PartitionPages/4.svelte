<script lang="ts">
  import diskIcon from "../../assets/icons/disk.svg";
  import { human2bytes } from "../../lib/utils/functions";
  import partitionStore from "../../lib/stores/partitionStore";
  import globalStore from "../../lib/stores/globalStore";
  import SegementedBar from "../../lib/components/SegementedBar2.svelte";
  import StepWrapper from "../../lib/components/StepWrapper.svelte";
  import Dropdown from "../../lib/components/Dropdown.svelte";
  console.log($partitionStore);
  let storageDevicesList: any[] = [];
  let partitionData: any[] = [];
  const colorList = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];

  function hasChange(e: Event) {
    $partitionStore.selectedDeviceForInstallAlong = e.detail.selected.name;
    partitionData = [];
    $partitionStore.partitionsWithOS
      .filter(
        (i: any) => i.kname == $partitionStore.selectedDeviceForInstallAlong,
      )
      .suggested_partitions.map((s: any) => {
        partitionData.push({
          label: s.label,
          suggested_size: s.suggested_size,
          minimum_size: s.minimum_size,
          maximum_size: s.maximum_size,
          kname:s.kname
        });
      });
  }
  async function gatherInfo() {
    storageDevicesList = [];
    partitionData = [];
    $partitionStore.partitionsWithOS.map((i: any) => {
      storageDevicesList.push({
        name: i.kname,
        selected:
          i.kname === $partitionStore.selectedDeviceForInstallAlong
            ? true
            : false,
      });
    });
    $partitionStore.partitionsWithOS[0]?.suggested_partitions.map((s: any) => {
      partitionData.push({
        label: s.label,
        suggested_size: s.suggested_size,
        minimum_size: s.minimum_size,
        maximum_size: s.maximum_size,
        kname:s.kname
      });
    });
    if ($partitionStore.r) {
      partitionData = $globalStore.partition.installAlongPartitions;
    }
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
  next="summary"
>
  <div class="flex flex-col items-center mx-5 h-full space-y-6">
    <div class="flex flex-row items-center gap-4 w-full">
      <div class="w-full">
        <Dropdown
          bind:items={storageDevicesList}
          label="Select Drive"
          icon={diskIcon}
          on:select={hasChange}
          defaultItem={{ name: "Select Drive" }}
        />
      </div>
    </div>
    <div class="w-full h-350">
      <div class="w-full h-300">
        <SegementedBar
          totalValue={human2bytes(
            $partitionStore.partitionsWithOS.filter(
              (i) => i.kname == $partitionStore.selectedDeviceForInstallAlong,
            )[0].size,
          )}
          items={partitionData}
        />
      </div>
    </div>
  </div></StepWrapper
>
