import { writable } from "svelte/store";

const keyboardStore = writable({
  region: "default",
  language: "default",
  layout: "default",
});

export default keyboardStore;
