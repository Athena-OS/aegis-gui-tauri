import { writable, type Writable } from "svelte/store";
import { type StorageDevice } from "../utils/types"

const partitionStore: Writable<{
  selectedDevice: string,
  mode: string,
  efi:boolean,
  swap:boolean,
  grubLocation:string,
  grubType:string,
  systemStorageInfo: StorageDevice[],
  systemStorageInfoCurrent: StorageDevice[],
  newPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string,
    isEncrypted: boolean,
    swapPartitionSize: string
  },
  replacedPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string
  }
}> = writable({
  selectedDevice: "default",
  mode: "auto",
  efi:true,
  swap: true,
  grubType:"",
  grubLocation:"",
  systemStorageInfo: [],
  systemStorageInfoCurrent: [],
  newPartition: {
    partitionName: "",
    size: 1024,
    fileSystem: "",
    mountPoint: "",
    name: "Athena OS",
    isEncrypted: false,
    swapPartitionSize: "1 Gib"
  },
  replacedPartition: {
    partitionName: "",
    size: 1024,
    fileSystem: "",
    mountPoint: "",
    name: "Athena OS",
    isEncrypted: false
  },
});

export default partitionStore;

export function resetPartitionStore() {
  partitionStore.set({
    selectedDevice: "default",
    mode: "auto",
    efi:true,
    swap: true,
    grubType:"",
    grubLocation:"",
    systemStorageInfo: [],
    systemStorageInfoCurrent: [],
    newPartition: {
      partitionName: "",
      size: 1024,
      fileSystem: "",
      mountPoint: "",
      name: "Athena OS",
      isEncrypted: false,
      swapPartitionSize: "1 Gib"
    },
    replacedPartition: {
      partitionName: "",
      size: 1024,
      fileSystem: "",
      mountPoint: "",
      name: "Athena OS",
      //isEncrypted: false
    },
  })
}
