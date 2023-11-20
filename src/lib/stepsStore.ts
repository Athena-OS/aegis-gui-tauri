import { writable } from "svelte/store";

const currentActive = writable(5);

export default currentActive;
