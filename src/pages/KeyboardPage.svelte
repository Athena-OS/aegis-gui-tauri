<script lang="ts">
  import Dropdown from "../lib/components/Dropdown.svelte";
  import InputBox from "../lib/components/InputBox.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  import keyboardStore from "../lib/stores/keyboardStore";

  import globeIcon from "../assets/icons/globe-icon.svg";
  import langIcon from "../assets/icons/lang-icon.svg";
  import keyboard from "../assets/keyboard.svg";
  import keyboardIcon from "../assets/icons/keyboard-icon.svg";
  import { invoke } from "@tauri-apps/api";
  let regionList: any[] = []
  let keymapList: any[] = []
  let timezoneList: any[] = []
  let localeList: any[] = []
  invoke("get_timezones").then((timezones: any) => {
    timezoneList = timezones.split("\n").map((i: string) => {
      return {name: i, selected: $keyboardStore.keymaps === i}
    })
  })
  invoke("get_keymaps").then((keymaps:any) => {
    keymapList = keymaps.split("\n").map((i: string) => {
      return {name: i, selected: $keyboardStore.region === i}
    })
  })
  invoke("get_locale").then((keymaps:any) => {
    localeList = keymaps.split("\n").map((i: string) => {
      return {name: i, selected: $keyboardStore.locale === i}
    })
  })
  let regionLis = [
    { name: "English (US)", selected: $keyboardStore.region === "English (US)" },
    { name: "German", selected: $keyboardStore.region === "German" },
    { name: "Spanish", selected: $keyboardStore.region === "Spanish" },
  ];
  let languageList = [
    { name: "English (US)", selected: $keyboardStore.language === "English (US)" },
    { name: "German", selected: $keyboardStore.language === "German" },
    { name: "Spanish", selected: $keyboardStore.language === "Spanish" },
  ];
  let layoutList = [
    { name: "AZERTY", selected: $keyboardStore.layout === "AZERTY" },
    { name: "QWERTY", selected: $keyboardStore.layout === "QWERTY" },
    { name: "QWERTZ", selected: $keyboardStore.layout === "QWERTZ" },
    { name: "QZERTY", selected: $keyboardStore.layout === "QZERTY" },
  ];

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    if (
      $keyboardStore.timezone !== "default" &&
      $keyboardStore.locale !== "default" &&
      $keyboardStore.keymaps !== "default"
    ) {
      nextPage = "/desktop";
    }
  }
  $: $keyboardStore, IsOkayToMoveNextPage();
</script>

<StepWrapper
  title="Select Keyboard"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/"
  next={nextPage}
>
  <div class="flex flex-col items-center space-y-4 w-full">
    <div class="flex flex-col items-center space-y-6 w-full max-w-md">
      <img src={keyboard} class="h-28" alt="" />
      <Dropdown
        icon={globeIcon}
        bind:items={timezoneList}
        label="Select timezone"
        on:select={(event) =>
          ($keyboardStore.timezone = event.detail.selected.name)}
        defaultItem={{ name: "Select Timezone" }}
      />
      <Dropdown
        icon={langIcon}
        bind:items={keymapList}
        label="Select Keymap"
        on:select={(event) =>
          ($keyboardStore.keymaps = event.detail.selected.name)}
        defaultItem={{ name: "Select Keymap" }}
      />
      <Dropdown
        icon={keyboardIcon}
        bind:items={localeList}
        label="Select Locale"
        on:select={(event) =>
          ($keyboardStore.locale = event.detail.selected.name)}
        defaultItem={{ name: "Select Locale" }}
      />
      <InputBox label="Test Keyboard" placeholderText="Type here.." />
    </div>
  </div>
</StepWrapper>
