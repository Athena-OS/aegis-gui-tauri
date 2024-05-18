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
      label: "Method of Partition",
      component: Method,
    },
  ];

  let currentStep = 0;

  const handlePrevious = () => {
    if (currentStep > 0) currentStep--;
  };

  const handleNext = () => {
    $partitionStore.new_ptable = true;
    /*$partitionStore.systemStorageInfo.filter(
      (item) => item.displayName === $partitionStore.selectedDevice,
    )[0].partitions = [];*/
    $partitionStore.systemStorageInfo.filter(
        (item) => item.displayName === $partitionStore.selectedDevice,
      )[0].partitions = [
        $partitionStore.systemStorageInfo
          .filter(
            (item) => item.displayName === $partitionStore.selectedDevice,
          )[0]
          ?.partitions.reduce(
            (acc, curr) => {
              if (acc.start === undefined || curr.start < acc.start) {
                acc.start = curr.start;
              }
              if (acc.end === undefined || curr.end > acc.end) {
                acc.end = curr.end;
              }
              acc.size += curr.size;
              return acc;
            },
            {
              partitionName: "free",
              size: 0,
              availableStorage: 0,
              name: "free",
              fileSystem: "",
              mountPoint: "",
              start: 0,
              end: 0,
              resized: false,
              action: "",
            },
          ),
      ];
    dialog.close();
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
            <h3 class="text-2xl my-2 font-meidum">
              Create a new partition table
            </h3>
            <div class="transition-height ease-out p-2">
              {#each steps as step, index}
                {#if index === currentStep}
                  <svelte:component this={step.component} />
                {/if}
              {/each}
              <div class="flex justify-between items-center pt-8 space-x-4">
                <!--Button
                  disabled={currentStep === 0}
                  variant="bordered"
                  fullWidth
                  on:click={handlePrevious}>Previous</Button
                -->
                <Button fullWidth on:click={handleNext}>Create</Button>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</div>
