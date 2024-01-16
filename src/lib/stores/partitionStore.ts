import { writable, type Writable } from "svelte/store";
import { type StorageDevice } from "../utils/types"

const partitionStore: Writable<{
  selectedDevice: string,
  mode: string,
  systemStorageInfo: StorageDevice[],
  systemStorageInfoCurrent: StorageDevice[],
  newPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string,
    isEncrypted: boolean
  },
  replacedPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string,
    isEncrypted: boolean
  }
}> = writable({
  selectedDevice: "default",
  mode: "auto",
  systemStorageInfo: [],
  systemStorageInfoCurrent: [],
  newPartition: {
    partitionName: "",
    size: 1024,
    fileSystem: "",
    mountPoint: "",
    name: "Athena OS",
    isEncrypted: false
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
