
import {invoke} from "@tauri-apps/api"
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


export function hashPassword(password:string) {
    let hash: any[] = []
    invoke("hash_password", {password: password}).then((hashS: any)=>{
        console.log(hashS)
        hash.push(hashS.trim())
    })
    return hash
}
