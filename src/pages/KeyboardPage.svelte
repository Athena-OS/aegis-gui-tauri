<script lang="ts">
  // @ts-nocheck
  import InputBox from "../lib/components/InputBox.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";
  import keyboardStore from "../lib/stores/keyboardStore";
  import globeIcon from "../assets/icons/globe-icon.svg";
  import langIcon from "../assets/icons/lang-icon.svg";
  import keyboard from "../assets/keyboard.svg";
  import keyboardIcon from "../assets/icons/keyboard-icon.svg";
  import { invoke } from "@tauri-apps/api";
  import ComboBox from "../lib/components/ComboBox.svelte";
  let keymapList: any[] = [];
  let timezoneList: any[] = [];
  let localeList: any[] = [];
  let x11keymaps: any[] = [];
  invoke("get_timezones").then((timezones: any) => {
    timezoneList = timezones.split("\n").map((i: string) => {
      return { text: i, value: i };
    });
    timezoneList.splice(timezoneList.length - 1, 1);
  });

  invoke("get_x11_keymaps").then((x11: any) => {
    x11keymaps = x11.split("\n").map((i: string) => {
      return { text: i, value: i };
    });
    x11keymaps.splice(x11keymaps.length - 1, 1);
  });
  invoke("get_keymaps").then((keymaps: any) => {
    keymapList = keymaps.split("\n").map((i: string) => {
      return { text: i, value: i };
    });
  });
  invoke("get_locale").then((keymaps: any) => {
    localeList = keymaps.split("\n").map((i: string) => {
      return { text: i, value: i };
    });
  });

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
  dialogTitle="About Keyboad Page"
  dialogContent="In this page, you setup your keyboard, timezone and locale"
  prev="/base"
  next={nextPage}
>
  <div class="flex flex-col items-center space-y-4 w-full">
    <div class="flex flex-col items-center space-y-6 w-full max-w-md">
      <img src={keyboard} class="h-28" alt="" />
      <ComboBox
        icon={globeIcon}
        bind:options={timezoneList}
        label="Select timezone"
        name="Timezone"
        placeholder="Select or search timezone ..."
        on:select={(event) => {
          if (event.detail.key === "Enter" || event.detail.type === "click") {
            $keyboardStore.timezone = event.detail.target.dataset.value;
          } else {
            $keyboardStore.timezone = event.detail.target.value;
          }
        }}
      />
      <ComboBox
        icon={langIcon}
        bind:options={keymapList}
        label="Select Console Keymap"
        name="Console Keymap"
        placeholder="Select or search console keymap ..."
        on:select={(event) => {
          if (event.detail.key === "Enter" || event.detail.type === "click") {
            $keyboardStore.keymaps = event.detail.target.dataset.value;
          } else {
            $keyboardStore.keymaps = event.detail.target.value;
          }
        }}
      />
      <ComboBox
        name="X11"
        icon={langIcon}
        placeholder="Select or search x11keymap ..."
        bind:options={x11keymaps}
        label="Select X11 Keymap"
        on:select={(event) => {
          console.log(event);
          if (event.detail.key === "Enter" || event.detail.type === "click") {
            $keyboardStore.x11keymap = event.detail.target.dataset.value;
          } else {
            $keyboardStore.x11keymap = event.detail.target.value;
          }
        }}
      />
      <ComboBox
        icon={keyboardIcon}
        name="Locale"
        bind:options={localeList}
        placeholder="Select or search locale ..."
        label="Select Locale"
        on:select={(event) => {
          if (event.detail.key === "Enter" || event.detail.type === "click") {
            $keyboardStore.locale = event.detail.target.dataset.value;
          } else {
            $keyboardStore.locale = event.detail.target.value;
          }
        }}
      />
      <InputBox label="Test Keyboard" placeholderText="Type here.." />
    </div>
  </div>
</StepWrapper>
