import { writable } from "svelte/store";

const extrasStore = writable({
    kernel: "default",
    terminal: "default",
    shell: "default",
    snapper: false,
    zram: false,
    hardening: false,
    keepgoing: false,
    maxjobs: 1,
    cores: 0,
});

export default extrasStore;