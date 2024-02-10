export type StorageDevice = {
    displayName: string;
    diskModel: string;
    logicalName: string;
    totalStorage: number;
    availableStorage: number;
    disklabelType: string;
    kind: string;
    partitions: PartitionInfo[];
    isRemovable: boolean;
};
export type PartitionInfo = {
    partitionName: string;
    size: number;
    availableStorage: number;
    name: string;
    fileSystem: string;
    mountPoint: string;
};