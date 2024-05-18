<script lang="ts">
  import Transition from "svelte-transition";
  import partitionStore from "../../stores/partitionStore";

  import Button from "../Button.svelte";
  import Method from "./Method.svelte";
  import Size from "./Size.svelte";
  import { MBtoBytes } from "../../utils/functions";

  export let dialog: any;

  let steps = [
    {
      label: "Size of Partition",
      component: Size,
    },
  ];

  let currentStep = 0;

  const handlePrevious = () => {
    dialog.close()
  };

  const handleNext = () => {
      if (
        $partitionStore.newPartition.name !== "" &&
        $partitionStore.newPartition.fileSystem !== "" &&
        $partitionStore.newPartition.mountPoint !== ""
      ) {
        $partitionStore.newPartition.size = MBtoBytes(
          $partitionStore.newPartition.size,
        );

        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.push({
            partitionName:
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].logicalName +
              `p${
                $partitionStore.systemStorageInfo.filter(
                  (item) => item.displayName === $partitionStore.selectedDevice,
                )[0].partitions.length + 1
              }`,
            size: $partitionStore.newPartition.size,
            fileSystem: $partitionStore.newPartition.fileSystem,
            mountPoint: $partitionStore.newPartition.mountPoint,
            name: $partitionStore.newPartition.name,
            availableStorage: 0,
            start: $partitionStore.systemStorageInfo.filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0].partitions[$partitionStore.ind].start,
            end:
              $partitionStore.systemStorageInfo.filter(
                (item) => item.displayName === $partitionStore.selectedDevice,
              )[0].partitions[$partitionStore.ind].start +
              $partitionStore.newPartition.size / 512,
            resized: true,
            // The last action after shrink and delete
            action: "create"
          });
        // Update the free space
        if (
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].partitions[$partitionStore.ind].size ==
          $partitionStore.newPartition.size
        ) {
          // Remove the partition since all the free space is used
          $partitionStore.systemStorageInfo
            .filter(
              (item) => item.displayName === $partitionStore.selectedDevice,
            )[0]
            .partitions.splice($partitionStore.ind, 1);
        } else {
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].partitions[$partitionStore.ind].start +=
            $partitionStore.newPartition.size / 512;
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].partitions[$partitionStore.ind].size -=
            $partitionStore.newPartition.size;
        }

        // Sort partitions using partition size
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          .partitions.sort((p1, p2) => p1.start - p2.start);

        $partitionStore.systemStorageInfo.filter(
          (item) => item.displayName === $partitionStore.selectedDevice,
        )[0].availableStorage =
          $partitionStore.systemStorageInfo.filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0].availableStorage - $partitionStore.newPartition.size;

        $partitionStore.newPartition = {
          partitionName: "",
          size: 1024,
          fileSystem: "",
          mountPoint: "",
          name: "Athena OS",
          isEncrypted: false,
          start: 1024,
          swapPartitionSize: "1 Gb",
        };

        dialog.close();
      }
  };
</script>

<div class="relative z-10">
  <Transition show={$dialog.expanded}>
    <Transition
      enter="ease-out duration-300"
      enterFrom="opacity-0"
      enterTo="opacity-100"
      leave="ease-in duration-200"
      leaveFrom="opacity-100"
      leaveTo="opacity-0"
    >
      <button
        on:click={dialog.close}
        aria-label="Close Dialog"
        class="fixed inset-0 bg-black backdrop-blur-xl bg-opacity-10 border-none p-0 m-0 focus:ring-0 focus:outline-none"
      />
    </Transition>
    <div class="fixed inset-0 overflow-y-auto">
      <div class="flex min-h-full items-center justify-center p-4 text-center">
        <Transition
          enter="ease-out duration-300"
          enterFrom="opacity-0 scale-95"
          enterTo="opacity-100 scale-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100 scale-100"
          leaveTo="opacity-0 scale-95"
        >
          <div
            class="w-full max-w-xl border border-neutral-700 transform overflow-hidden rounded-2xl bg-gray-800 px-4 py-3 text-left align-middle shadow-xl transition-all"
            use:dialog.modal
          >
            <h3 class="text-2xl my-2 font-meidum">Create a new partition</h3>
            <div class="transition-height ease-out p-2">
              {#each steps as step, index}
                {#if index === currentStep}
                  <svelte:component this={step.component} />
                {/if}
              {/each}
              <div class="flex justify-between items-center pt-8 space-x-4">
                <Button
                  variant="bordered"
                  fullWidth
                  on:click={handlePrevious}>Cancel</Button
                ><Button fullWidth on:click={handleNext}
                  >Create</Button
                >
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</div>
