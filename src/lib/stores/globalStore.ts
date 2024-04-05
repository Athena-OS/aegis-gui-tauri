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
      base:$extraStore.base,
      partition: {
        device:$partitionStore.selectedDevice,
        mode:$partitionStore.mode,
        efi: $partitionStore.efi,
        swap: $partitionStore.swap,
        encrypt_check: $partitionStore.encrypt_check,
        swap_size:$partitionStore.newPartition.swapPartitionSize,
        partitions:[],
        system_storage_info:$partitionStore.systemStorageInfo.filter((s) => {
          let partitionDisNAme = s.partitions.map((p) => p.partitionName);
          // check if any of the partionNames has the selected device
          if (
            partitionDisNAme[0]?.indexOf($partitionStore.selectedDevice) != 1
          ) {
            return s;
          }
        }),
        system_storage_info_current:$partitionStore.systemStorageInfoCurrent,
        installAlongPartitions:$partitionStore.installAlongPartitions
      },
      bootloader:{
        type:$partitionStore.grubType,
        location:$partitionStore.grubLocation,
      },
      locale: {
        locale:[$keyboardStore.locale],
        timezone:$keyboardStore.timezone,
        virtkeymap: $keyboardStore.keymaps,
        x11keymap: $keyboardStore.x11keymap,
      },
      networking: {
        hostname:$extraStore.hostname,
        ipv6:$extraStore.ipv6,
      },
      users:$accountStore.users,
      rootpass:$accountStore.users.filter(i => i.hasRoot)[0]?.password,
      desktop:$desktopStore.environment,
      theme: $desktopStore.theme,
      displayManager: $desktopStore.displayManager,
      browser:$extraStore.browser,
      
      packagesStore: $packagesStore,
      terminal: $extraStore.terminal,
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
