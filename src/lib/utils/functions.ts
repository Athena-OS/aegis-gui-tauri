export function bytesToGB(bytes: number) {
    if (typeof bytes !== "number" || isNaN(bytes) || bytes < 0) {
        throw new Error(
            "Invalid input. Please provide a non-negative number representing bytes.",
        );
    }

    const GB = Math.pow(1024, 3);
    const gigabytes = bytes / GB;

    return gigabytes.toFixed(2); // Round to two decimal places
}

export function bytesToMB(bytes: number) {
    if (typeof bytes !== 'number' || isNaN(bytes) || bytes < 0) {
        throw new Error(
            "Invalid input. Please provide a non-negative number representing bytes.",
        );
    }

    const MB = Math.pow(1024, 2);
    const megabytes = bytes / MB;

    return megabytes.toFixed(2); // Round to two decimal places
}