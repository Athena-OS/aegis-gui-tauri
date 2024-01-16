export type StorageDevice = {
    displayName: String;
    diskModel: String;
    logicalName: String;
    totalStorage: number;
    availableStorage: number;
    disklabelType: String;
    kind: String;
    partitions: PartitionInfo[];
    isRemovable: boolean;
};
export type PartitionInfo = {
    partitionName: String;
    size: number;
    availableStorage: number;
    name: String;
    fileSystem: String;
    mountPoint: String;
};