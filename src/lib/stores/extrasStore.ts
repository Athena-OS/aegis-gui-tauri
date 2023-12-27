import { writable } from "svelte/store";

const extrasStore = writable({
    kernel: "default",
    terminal: "default",
    shell: "default",
});

export default extrasStore;