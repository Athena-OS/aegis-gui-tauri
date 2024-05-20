
import { invoke } from "@tauri-apps/api/core";
export function bytesToGB(bytes: number) {
    if (isNaN(bytes) || bytes < 0) {
        throw new Error(
            "Invalid input. Please provide a non-negative number representing bytes.",
        );
    }

    const GB = Math.pow(1024, 3);
    const gigabytes = bytes / GB;

    return gigabytes.toFixed(2); // Round to two decimal places
}

export function bytesToMB(bytes: number) {
    if (isNaN(bytes) || bytes < 0) {
        throw new Error(
            "Invalid input. Please provide a non-negative number representing bytes.",
        );
    }

    const MB = Math.pow(1024, 2);
    const megabytes = bytes / MB;

    return megabytes.toFixed(2); // Round to two decimal places
}

export function MBtoBytes(megabytes: number) {
    if (isNaN(megabytes) || megabytes < 0) {
        throw new Error('Invalid input. Please provide a non-negative number representing megabytes.');
    }

    const bytesInMB = Math.pow(1024, 2);
    const bytes = megabytes * bytesInMB;

    return bytes;
}

export function MBtoGB(megabytes: number) {
    if (isNaN(megabytes) || megabytes < 0) {
        throw new Error('Invalid input. Please provide a non-negative number representing megabytes.');
    }

    const bytesInMB = Math.pow(1024, 2);
    const bytesInGB = Math.pow(1024, 3);
    const gigabytes = megabytes / (bytesInGB / bytesInMB);

    return gigabytes.toFixed(2); // Round to two decimal places
}

export function GBToMB(gigabytes: number) {
    if (typeof gigabytes !== 'number' || gigabytes < 0) {
        throw new Error('Invalid input. Please provide a non-negative number representing gigabytes.');
    }

    const megabytes = gigabytes * 1024;

    return megabytes.toFixed(2); // Round to two decimal places
}


export async  function hashPassword(password:string) {
    let hash: string
    hash = await invoke("hash_password", {password: password})
    return hash.trim()
}

type SizeParseError = 'InvalidInput' | 'NegativeSize';

export function human2bytes(size: string) {
    let normalizedSize = size.trim().toUpperCase();
    let multiplier: number = 1;

    const units: Array<[string, number]> = [
        ["B", 1],
        ["K", 1024],
        ["M", 1048576],
        ["G", 1073741824],
        ["T", 1099511627776],
    ];

    for (const [unit, mult] of units) {
        if (normalizedSize.endsWith(unit)) {
            normalizedSize = normalizedSize.slice(0, -unit.length);
            multiplier = mult;
            break;
        }
    }

    const number = parseFloat(normalizedSize);
    if (isNaN(number)) {
        throw new Error(`InvalidInput: ${normalizedSize}`);
    }
    if (number < 0) {
        throw new Error('NegativeSize');
    }

    return number * multiplier;
}
