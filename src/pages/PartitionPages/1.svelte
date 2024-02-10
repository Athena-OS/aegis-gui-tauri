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

  $partitionStore.mode = "auto";
  let storageDevicesList: any[] = [];

  async function fetchAndParseStorageInfo() {
    let sysInfo_Disks = await disks();

    invoke("get_partitions_file_systems").then((file_systems_response: any) => {
      invoke("get_storage_devices").then((response: any) => {
        let commandOutput = response
          .split("\n\n\n")
          .filter((item: string) => item.includes("Disk model"));

        commandOutput.map((disksData: any) => {
          let DisksDataLines = disksData.split("\n");

          let disk_data: StorageDevice = {
            diskModel: DisksDataLines[1].split(":")[1].trim(),
            logicalName: DisksDataLines[0].split(":")[0].split(" ")[1].trim(),
            displayName: "",
            totalStorage: parseInt(
              DisksDataLines[0]
                .split(":")[1]
                .split(",")[1]
                .trim()
                .replace("bytes", ""),
            ),
            availableStorage: 0,
            disklabelType: DisksDataLines[5].split(":")[1].trim(),
            kind: "",
            isRemovable: false,
            partitions: [],
          };

          let currentDiskPartitionsSysInfo = sysInfo_Disks.filter((item) =>
            item.name.includes(disk_data.logicalName),
          );

          for (
            let index =
              DisksDataLines.indexOf(
                DisksDataLines.find((line: string) => line.includes("Device")),
              ) + 1;
            index < DisksDataLines.length;
            index++
          ) {
            let elements = DisksDataLines[index]
              .split(" ")
              .filter((item: any) => item.length > 0);

            let elementPartitionInfo = currentDiskPartitionsSysInfo.find(
              (item) => item.name === elements[0].trim(),
            );

            elements.splice(0, 1);
            elements.splice(0, 1);
            elements.splice(0, 1);
            elements.splice(0, 1);
            elements.splice(0, 1);

            if (elementPartitionInfo !== undefined) {
              disk_data.partitions.push({
                partitionName: elementPartitionInfo.name,
                size: elementPartitionInfo.total_space,
                availableStorage: elementPartitionInfo.available_space,
                name: elements.join(" "),
                fileSystem: file_systems_response
                  .split("\n")
                  .find((line: any) => line.includes(elementPartitionInfo.name))
                  .split(" ")
                  .filter((item: any) => item.length > 0)[1],
                mountPoint: elementPartitionInfo.mount_point,
              });
            }
          }

          disk_data.kind = currentDiskPartitionsSysInfo[0].kind;
          disk_data.isRemovable = currentDiskPartitionsSysInfo[0].is_removable;
          disk_data.displayName =
            disk_data.diskModel +
            " " +
            disk_data.kind +
            " ( " +
            bytesToGB(disk_data.totalStorage) +
            "GB )";

          disk_data.availableStorage =
            disk_data.totalStorage -
            disk_data.partitions.reduce((accumulator, partition) => {
              return accumulator + partition.size;
            }, 0);

          disk_data.partitions.map((partition: any) => {
            partition.size = partition.size - partition.availableStorage;
            partition.availableStorage = 0;
          });

          let temp_disk_data = JSON.parse(JSON.stringify(disk_data));
          $partitionStore.systemStorageInfoCurrent.push({...temp_disk_data});
          $partitionStore.systemStorageInfo.push(disk_data);

          storageDevicesList.push({
            name: disk_data.displayName,
          });
        });
      });
    });
  }

  fetchAndParseStorageInfo();

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    if ($partitionStore.selectedDevice !== "default") {
      if ($partitionStore.mode === "auto") {
        nextPage = "/accounts";
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
  prev="/packages"
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
