import { writable, type Writable } from "svelte/store";
import { type StorageDevice, type InstallAlongPartition } from "../utils/types"

const partitionStore: Writable<{
  selectedDevice: string,
  selectedDeviceForInstallAlong: string,
  mode: string,
  efi:boolean,
  swap:boolean,
  grubLocation:string,
  grubType:string,
  r:boolean,
  ind: number,
  swap_size:string,
  encrypt_check:boolean,
  luksPassword:string,
  systemStorageInfo: StorageDevice[],
  systemStorageInfoCurrent: StorageDevice[],
  partitionsWithOS: InstallAlongPartition[],
  installAlongPartitions:any[],
  newPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string,
    isEncrypted: boolean,
    swapPartitionSize: string,
    start: number
  },
  replacedPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string
  },
  bootPartition: {
    partitionName: string,
    size: number,
    fileSystem: string,
    mountPoint: string,
    name: string
  },
  installAlongPartition: {
    partitionName:string
    size:number,
    filesystem:string,
  }
}> = writable({
  selectedDevice: "default",
  selectedDeviceForInstallAlong: "default",
  installAlongPartitions:[],
  mode: "auto",
  r:false,
  ind:0,
  efi:true,
  swap: false,
  grubType:"",
  encrypt_check:false,
  luksPassword:"",
  grubLocation:"",
  swap_size:"1GiB",
  systemStorageInfo: [],
  systemStorageInfoCurrent: [],
  partitionsWithOS: [],
  newPartition: {
    partitionName: "",
    size: 1024,
    fileSystem: "",
    mountPoint: "",
    name: "Athena OS",
    isEncrypted: false,
    swapPartitionSize: "1Gib",
    start: 1024,
  },
  replacedPartition: {
    partitionName: "",
    size: 1024,
    fileSystem: "",
    mountPoint: "",
    name: "Athena OS",
    isEncrypted: false
  },
  bootPartition: {
    partitionName: "",
    size: 1024,
    fileSystem: "",
    mountPoint: "/boot",
    name: "Athena OS",
    isEncrypted: false
  },
  installAlongPartition:{
    partitionName:"",
    size:1024,
    filesystem:""

  }
});

export default partitionStore;

export function resetPartitionStore() {
  partitionStore.set({
    selectedDevice: "default",
    selectedDeviceForInstallAlong: "default",
    installAlongPartitions:[],
    r:false,
    ind: 0,
    mode: "auto",
    efi:true,
    swap: false,
    swap_size:"1GiB",
    grubType:"",
    grubLocation:"",
    encrypt_check:false,
    luksPassword:"",
    systemStorageInfo: [],
    systemStorageInfoCurrent: [],
    partitionsWithOS:[],
    newPartition: {
      partitionName: "",
      size: 1024,
      fileSystem: "",
      mountPoint: "",
      name: "Athena OS",
      isEncrypted: false,
      swapPartitionSize: "1GiB",
      start: 1024
    },
    replacedPartition: {
      partitionName: "",
      size: 1024,
      fileSystem: "",
      mountPoint: "",
      name: "Athena OS",
      //isEncrypted: false
    },
    bootPartition: {
      partitionName: "",
      size: 1024,
      fileSystem: "don't format",
      mountPoint: "/boot",
      name: "Athena OS",
      //isEncrypted: false
    },
    installAlongPartition:{
      partitionName:"",
      size:0,
      filesystem:""
    }
  })
}
