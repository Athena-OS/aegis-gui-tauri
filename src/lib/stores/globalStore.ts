import { derived } from "svelte/store";
import keyboardStore from "./keyboardStore";
import environmentStore from "./environmentStore";

const globalStore = derived(
  [keyboardStore, environmentStore],
  ([$keyboardStore, $environmentStore]) => {
    return {
      keyboard: {
        region: $keyboardStore.region,
        language: $keyboardStore.language,
        layout: $keyboardStore.layout,
      },
      environment: {
        desktop: $environmentStore.desktop,
      },
    };
  }
);

export default globalStore;
