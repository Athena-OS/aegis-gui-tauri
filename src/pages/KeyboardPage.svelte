<script lang="ts">
  import Dropdown from "../lib/components/Dropdown.svelte";
  import InputBox from "../lib/components/InputBox.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  import globeIcon from "../assets/icons/globe-icon.svg";
  import langIcon from "../assets/icons/lang-icon.svg";
  import keyboardIcon from "../assets/icons/keyboard-icon.svg";
  import keyboardStore from "../lib/stores/keyboardStore";

  let languageList = [
    { name: "English (US)" },
    { name: "German" },
    { name: "Spanish" },
  ];
</script>

<StepWrapper
  title="Select Keyboard"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/"
  next="/desktop"
>
  <div class="flex flex-col items-center space-y-4 w-full">
    <div class="flex flex-col items-center space-y-8 w-full max-w-md">
      <Dropdown
        icon={globeIcon}
        bind:items={languageList}
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
        bind:items={languageList}
        label="Select Layout"
        on:select={(event) =>
          ($keyboardStore.layout = event.detail.selected.name)}
        defaultItem={{ name: "Select Layout" }}
      />
      <InputBox label="Test Keyboard" placeholderText="Type here.." />
    </div>
  </div>
</StepWrapper>
