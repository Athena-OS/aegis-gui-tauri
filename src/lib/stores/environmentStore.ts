import { writable } from "svelte/store";

const environmentStore = writable({
  desktop: "default",
});

export default environmentStore;
