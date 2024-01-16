import { writable } from "svelte/store";

const desktopStore = writable({
  environment: "default",
  theme: "default",
  displayManager: "default",
  themeImage: "default",
});

export default desktopStore;
