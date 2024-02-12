import { derived } from "svelte/store";
import keyboardStore from "./keyboardStore";
import desktopStore from "./desktopStore";
import partitionStore from "./partitionStore";
import packagesStore from "./packagesStore";
import accountsStore from "./accountsStore";
import extrasStore from "./extrasStore";
const globalStore = derived(
  [keyboardStore, desktopStore, partitionStore, packagesStore, accountsStore, extrasStore],
  ([$keyboardStore, $desktopStore, $partitionStore, $packagesStore, $accountStore, $extraStore]) => {
    return {
      partition: {
        device:$partitionStore.selectedDevice,
        mode:$partitionStore.mode,
        efi: $partitionStore.efi,
        swap: $partitionStore.swap,
        swap_size:$partitionStore.newPartition.swapPartitionSize,
        partitions:[]
      },
      bootloader:{
        type:$partitionStore.grubType,
        location:$partitionStore.grubLocation,
      },
      locale: {
        locale:[$keyboardStore.locale],
        timezone:$keyboardStore.timezone,
        virtkeymap: $keyboardStore.keymaps,
        x11keymap: "",
      },
      netwroking: {
        hostname:$extraStore.hostname,
        ipv6:$extraStore.ipv6,
      },
      users:$accountStore.users,
      rootpass:"",
      desktop:$desktopStore.environment,
      theme: $desktopStore.theme,
      displayManager: $desktopStore.displayManager,
      browser:$extraStore.browser,
      
      packagesStore: $packagesStore,
      
      extra_packages:[],
      kernel: $extraStore.kernel,
      snapper:$extraStore.snapper,
      zramd:$extraStore.zram,
      hardened:$extraStore.hardening,
      flatpak:$extraStore.flatpak,
      params:{
        cores:$extraStore.cores,
        jobs:$extraStore.maxjobs,
        keep:$extraStore.keepgoing,
      }
    };
  }
);

export default globalStore;
