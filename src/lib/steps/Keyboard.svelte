<script lang="ts">
  import ListBox from "../components/ListBox.svelte";
  import InputBox from "../components/InputBox.svelte";

  import globeIcon from "../../assets/icons/globe-icon.svg";
  import keyboardIcon from "../../assets/icons/keyboard-icon.svg";
  import langIcon from "../../assets/icons/lang-icon.svg";
  import StepWrapper from "../components/StepWrapper.svelte";
  import keyboardImage from "../../assets/keyboard.svg";

  let languageList = [
    { name: "English (US)" },
    { name: "German" },
    { name: "Spanish" },
    // ... other names
  ];

  function handleSelect(event: CustomEvent<any>) {
    console.log("Selected item:", event.detail);
  }
</script>

<StepWrapper
  title="Select Keyboard"
  dialogTitle="Header Here"
  dialogContent="Your text here"
>
  <div class="flex flex-col items-center space-y-4">
    <img class="w-[28em] opacity-40" src={keyboardImage} alt="" />
    <div class="flex flex-col items-center space-y-8">
      <ListBox
        bind:items={languageList}
        icon={globeIcon}
        label="Select Region"
        on:select={handleSelect}
        defaultItem={{ name: "Select Region" }}
      />
      <ListBox
        bind:items={languageList}
        icon={langIcon}
        label="Select Language"
        on:select={handleSelect}
        defaultItem={{ name: "Select Language" }}
      />
      <ListBox
        bind:items={languageList}
        icon={keyboardIcon}
        label="Select Layout"
        on:select={handleSelect}
        defaultItem={{ name: "Select Layout" }}
      />
      <InputBox label="Test Keyboard" placeholderText="Type here.." />
    </div>
  </div></StepWrapper
>
