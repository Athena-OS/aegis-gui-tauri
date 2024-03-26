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
    start: number;
    end: number;
    resized:boolean,
    action: string,
};

export type InstallAlongPartition = {
    maximum_size: number,
    minimum_size: number,
    label: string,
    suggested_size: number,
    kname: string,
}