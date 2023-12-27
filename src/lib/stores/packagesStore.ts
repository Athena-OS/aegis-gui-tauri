import { writable } from "svelte/store";

const packagesStore = writable({
    packages:{}
});

export default packagesStore;
