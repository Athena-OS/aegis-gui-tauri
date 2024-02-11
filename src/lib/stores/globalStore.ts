import { derived } from "svelte/store";
import keyboardStore from "./keyboardStore";
import desktopStore from "./desktopStore";
import partitionStore from "./partitionStore";
import packagesStore from "./packagesStore";
import accountsStore from "./accountsStore";
const globalStore = derived(
  [keyboardStore, desktopStore, partitionStore, packagesStore, accountsStore],
  ([$keyboardStore, $desktopStore, $partitionStore, $packagesStore, $accountStore]) => {
    return {
      keyboard: {
        region: $keyboardStore.region,
        language: $keyboardStore.language,
        layout: $keyboardStore.layout,
      },
      desktop:$desktopStore.environment,
      theme: $desktopStore.theme,
      displayManager: $desktopStore.displayManager,
      browser:$packagesStore,
      
      partition: {
        device:$partitionStore.selectedDevice,
        mode:$partitionStore.mode,
        efi: $partitionStore.efi,
        swap: $partitionStore.swap,
        swap_size:$partitionStore.newPartition.swapPartitionSize,
        partitions:[]
      },
      bootloader:{
        type:"",
        location:"",
      },
      locale:{
        locale:["it_IT.UTF-8"],
        keymap:"it",
        timezone: ""
      },
      packagesStore: $packagesStore,
      users:$accountStore
    };
  }
);

export default globalStore;
