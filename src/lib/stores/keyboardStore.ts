import { writable } from "svelte/store";

const keyboardStore = writable({
  region: "default",
  language: "default",
  layout: "default",
  keymaps: "default",
  timezone: "default",
  locale: "default",
  x11keymap:"default"
});

export default keyboardStore;
