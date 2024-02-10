import { writable, type Writable } from "svelte/store";

const packagesStore: Writable<{
    packages: {}
}> = writable({
    packages: {}
});

export default packagesStore;
