import { derived } from "svelte/store";
import keyboardStore from "./keyboardStore";
import desktopStore from "./desktopStore";

const globalStore = derived(
  [keyboardStore, desktopStore],
  ([$keyboardStore, $desktopStore]) => {
    return {
      keyboard: {
        region: $keyboardStore.region,
        language: $keyboardStore.language,
        layout: $keyboardStore.layout,
      },
      environment: {
        environment: $desktopStore.environment,
        theme: $desktopStore.theme,
        displayManager: $desktopStore.displayManager,
      },
    };
  }
);

export default globalStore;
