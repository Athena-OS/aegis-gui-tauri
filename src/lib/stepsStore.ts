import { writable } from "svelte/store";

const currentActive = writable(0);

export default currentActive;
