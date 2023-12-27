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
    name: String;
    size: number;
    availableStorage: number;
    type: String;
    fileSystem: String;
    mountPoint: String;
};