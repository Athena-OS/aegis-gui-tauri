import { writable, type Writable } from "svelte/store";
import { type StorageDevice } from "../utils/types"

const partitionStore: Writable<{
  selectedDevice: string,
  mode: string,
  systemStorageInfo: StorageDevice[],
  newPartition: {
    name: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    type: string
  },
  replacedPartition: string
}> = writable({
  selectedDevice: "default",
  mode: "auto",
  systemStorageInfo: [],
  newPartition: {
    name: "Athena OS",
    size: 1024,
    fileSystem: "",
    mountPoint: "",
    type: "",
  },
  replacedPartition: "",
});

export default partitionStore;
