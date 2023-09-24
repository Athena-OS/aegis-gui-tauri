<script lang="ts">
    import ListBox from '../ListBox.svelte';
    import DialogComponent from '../InfoDialog.svelte';
    import Footer from "../ProgressStepper.svelte";

    import globeIcon from "../../assets/icons/globe-icon.svg";
    import keyboardIcon from "../../assets/icons/keyboard-icon.svg";
    import langIcon from "../../assets/icons/lang-icon.svg";
    import arrowDown from "../../assets/icons/arrow-down-icon.svg";
    import keyboard from "../../assets/keyboard.svg";

    let languageList = [
        { name: 'English (US)' },
        { name: 'German' },
        { name: 'Spanish' },
        { name: 'French' },
        { name: 'Africaans' },
        { name: 'Arab' },
        { name: 'English (UK)' },
        // ... other names
    ];

    function handleSelect(event: CustomEvent<any>) {
        console.log("Selected item:", event.detail);
    }


    export let switchView: (viewName: string) => void;
</script>

<DialogComponent stepNumber="1" title="Select Keyboard" modalHeader="Header Here" modalText="Your text here" />
<div class="flex flex-col items-center gap-10 mt-10">
    <img src={keyboard} alt="athena-logo" class="w-[30em]"/>

    <div class="flex flex-col items-center gap-4">
        <ListBox bind:items={languageList} icon={globeIcon} label="Select Region" on:select={handleSelect} defaultItem={{ name: 'Select Region' }}/>
        <ListBox bind:items={languageList} icon={langIcon} label="Select Language" on:select={handleSelect} defaultItem={{ name: 'Select Language' }}/>
        <ListBox bind:items={languageList} icon={langIcon} label="Select Language" on:select={handleSelect} defaultItem={{ name: 'Select Language' }}/>
    </div>
</div>
<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("initial")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("keyboard")}>Continue</button>
    </div>
</Footer>
