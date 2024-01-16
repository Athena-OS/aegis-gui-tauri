<script lang="ts">
  import Dropdown from "../lib/components/Dropdown.svelte";
  import InputBox from "../lib/components/InputBox.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  import keyboardStore from "../lib/stores/keyboardStore";

  import globeIcon from "../assets/icons/globe-icon.svg";
  import langIcon from "../assets/icons/lang-icon.svg";
  import keyboard from "../assets/keyboard.svg";
  import keyboardIcon from "../assets/icons/keyboard-icon.svg";

  let regionList = [
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
    { name: "English (US)", selected: $keyboardStore.layout === "English (US)" },
    { name: "German", selected: $keyboardStore.layout === "German" },
    { name: "Spanish", selected: $keyboardStore.layout === "Spanish" },
  ];

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    if (
      $keyboardStore.region !== "default" &&
      $keyboardStore.language !== "default" &&
      $keyboardStore.layout !== "default"
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
        bind:items={regionList}
        label="Select Region"
        on:select={(event) =>
          ($keyboardStore.region = event.detail.selected.name)}
        defaultItem={{ name: "Select Region" }}
      />
      <Dropdown
        icon={langIcon}
        bind:items={languageList}
        label="Select Language"
        on:select={(event) =>
          ($keyboardStore.language = event.detail.selected.name)}
        defaultItem={{ name: "Select Language" }}
      />
      <Dropdown
        icon={keyboardIcon}
        bind:items={layoutList}
        label="Select Layout"
        on:select={(event) =>
          ($keyboardStore.layout = event.detail.selected.name)}
        defaultItem={{ name: "Select Layout" }}
      />
      <InputBox label="Test Keyboard" placeholderText="Type here.." />
    </div>
  </div>
</StepWrapper>
