<script lang="ts">
  import type { InstallAlongPartition, PartitionInfo } from "../utils/types";
  import { createDialog } from "svelte-headlessui";
  import Dialog from "../../lib/components/Dialog.svelte";
  import InputBox from "../../lib/components/InputBox.svelte";
  import Button from "../../lib/components/Button.svelte";
  import partitionStore from "../../lib/stores/partitionStore";
  import globalStore from "../stores/globalStore";
  import {
    GBToMB,
    MBtoBytes,
    bytesToGB,
    bytesToMB,
  } from "../../lib/utils/functions";
  export let totalValue = 100;
  export let color: string;
  function ResizingPartitionOnChangeValue(e: any) {
    let value = GBToMB(parseFloat(e.target.value));
    if (!isNaN(parseFloat(value))) {
      if (
        parseFloat(MBtoBytes(parseFloat(value)).toString()) >
          selectedItem.minimum_size &&
        parseFloat(MBtoBytes(parseFloat(value)).toString()) <
          selectedItem.maximum_size
      ) {
        e.target.parentElement.classList.remove("border-red-500");
        items[
          items.map((i: any) => i.label == selectedItem.label).indexOf(false)
        ].suggested_size =
          items[
            items.map((i: any) => i.label == selectedItem.label).indexOf(false)
          ].suggested_size +
          selectedItem.suggested_size -
          MBtoBytes(parseFloat(value));
        items[
          items.map((i: any) => i.label == selectedItem.label).indexOf(true)
        ].suggested_size = MBtoBytes(parseFloat(value));
        selectedItem.suggested_size = MBtoBytes(parseFloat(value));
      } else {
        e.target.parentElement.classList.add("border-red-500");
      }
    } else {
      e.target.parentElement.classList.add("border-red-500");
    }
  }
  export let items: InstallAlongPartition[];
  const backgroundClasses = [
    "bg-red-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-primary-500",
    "bg-purple-500",
    "bg-neutral-500",
  ];
  $partitionStore.installAlongPartitions = items
  let selectedItem: InstallAlongPartition;
  let dialogEditPartition = createDialog({ label: "edit-partition" });
  function partitionClicked(e: any) {
    selectedItem = items.filter((i: any) => i.label == e.target.id)[0];
    dialogEditPartition.open();
  }
</script>

<Dialog dialog={dialogEditPartition}>
  <div class="w-full h-fit space-y-4 py-4">
    <h4 class="text-2xl font-meidum">Resize Partition</h4>
    <div class="text-xl justify-between">
      <h4>
        Maximum Size for {selectedItem?.label} partition: {bytesToGB(
          selectedItem?.maximum_size ?? 0,
        )} GB
      </h4>
      <h4>
        Minimun Size for {selectedItem?.label} partition: {bytesToGB(
          selectedItem?.minimum_size ?? 0,
        )} GB
      </h4>
      <h4>
        Suggested Size for {selectedItem?.label} partition: {bytesToGB(
          selectedItem?.suggested_size ?? 0,
        )} GB
      </h4>
    </div>
    <div class="flex space-x-2">
      <InputBox
        givenOnChangeValue={ResizingPartitionOnChangeValue}
        value={bytesToGB(selectedItem?.suggested_size ?? 0)}
        placeholderText="Enter Value"
        label="Resized Storage"
        rightLabel="GB"
        inputType="number"
        styleClass="w-1/2"
      />
    </div>
  </div>
  <div class="flex justify-between mt-4">
    <div class="w-40">
      <Button
        variant="bordered"
        on:click={() => dialogEditPartition.close()}
        fullWidth>Cancel</Button
      >
    </div>
    <div class="w-40">
      <Button
        on:click={() => {
          $partitionStore.installAlongPartitions = items;
          $partitionStore.r = true;
          dialogEditPartition.close();
        }}
        fullWidth>Confirm</Button
      >
    </div>
  </div>
</Dialog>
<div style="height:300px" class="w-full bg-neutral-800 flex overflow-hidden">
  {#if $partitionStore.r}
    {#each items as item, index}
      <button
        class="group flex justify-center items-center hover:border-2 hover:border-yellow h-full cursor-pointer {color !==
        undefined
          ? color
          : backgroundClasses[index]}"
        style="width: {(item.suggested_size / totalValue) * 100}%"
        id={item.label}
        on:click={partitionClicked}
      >
        <h4
          class="absolute mt-20 group-hover:bg-gray-800 group-hover:border-2 group-hover:border-gray-700 font-bold px-2 py-1 rounded-lg text-transparent group-hover:text-white"
        >
          Edit {item.label} partition size.
        </h4>
        <h4>
          {item.label}
          <br />
          {bytesToMB(item.suggested_size)} MB / {bytesToGB(item.suggested_size)}
          GB
        </h4>
      </button>
    {/each}
  {:else}
    {#each items as item, index}
      <button
        class="group flex justify-center items-center hover:border-2 hover:border-yellow h-full cursor-pointer {color !==
        undefined
          ? color
          : backgroundClasses[index]}"
        style="width: {(item.suggested_size / totalValue) * 100}%"
        id={item.label}
        on:click={partitionClicked}
      >
        <h4
          class="absolute mt-20 group-hover:bg-gray-800 group-hover:border-2 group-hover:border-gray-700 font-bold px-2 py-1 rounded-lg text-transparent group-hover:text-white"
        >
          Edit {item.label} partition size.
        </h4>
        <h4>
          {item.label}
          <br />
          {bytesToMB(item.suggested_size)} MB / {bytesToGB(item.suggested_size)}
          GB
        </h4>
      </button>
    {/each}
  {/if}
</div>
