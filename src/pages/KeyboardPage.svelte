<script lang="ts">
  import Dropdown from "../lib/components/Dropdown.svelte";
  import InputBox from "../lib/components/InputBox.svelte";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  import globeIcon from "../assets/icons/globe-icon.svg";
  import langIcon from "../assets/icons/lang-icon.svg";
  import keyboardIcon from "../assets/icons/keyboard-icon.svg";

  let languageList = [
    { name: "English (US)" },
    { name: "German" },
    { name: "Spanish" },
  ];

  function handleSelect(event: CustomEvent<any>) {
    console.log("Selected item:", event.detail);
  }
</script>

<StepWrapper
  title="Select Keyboard"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/"
  next="/desktop"
>
  <div class="flex flex-col items-center space-y-4">
    <!-- <img class="w-[28em] opacity-40" src={keyboardImage} alt="" /> -->
    <div class="flex flex-col items-center space-y-8">
      <Dropdown
        icon={globeIcon}
        bind:items={languageList}
        label="Select Region"
        on:select={handleSelect}
        defaultItem={{ name: "Select Region" }}
      />
      <Dropdown
        icon={langIcon}
        bind:items={languageList}
        label="Select Language"
        on:select={handleSelect}
        defaultItem={{ name: "Select Language" }}
      />
      <Dropdown
        icon={keyboardIcon}
        bind:items={languageList}
        label="Select Layout"
        on:select={handleSelect}
        defaultItem={{ name: "Select Layout" }}
      />
      <InputBox label="Test Keyboard" placeholderText="Type here.." />
    </div>
  </div>
</StepWrapper>
