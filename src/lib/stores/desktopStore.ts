import { writable } from "svelte/store";

const desktopStore = writable({
  environment: "default",
  theme: "default",
  displayManager: "default",
});

export default desktopStore;
