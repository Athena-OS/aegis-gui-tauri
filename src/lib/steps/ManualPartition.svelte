<script lang="ts">
    import Footer from "../ProgressStepper.svelte";
    import DialogComponent from '../InfoDialog.svelte';
    import ListBox from "../ListBox.svelte";


    import refreshIcon from "../../assets/icons/refresh.svg";
    import diskIcon from "../../assets/icons/disk.svg";

    let partitionList = [
        { name: 'Samsung NVME SSD 500G' },
        // ... other names
    ];

    function handleSelect(event: CustomEvent<any>) {
        console.log("Selected item:", event.detail);
    }

    let selectedOption: null | string = null;

    function handleOptionSelect(option: string) {
        selectedOption = option;
    }

    export let switchView: (viewName: string) => void;
</script>

<DialogComponent 
    stepNumber="3.1" 
    title="Manual Partitioning" 
    modalHeader="Header Here" 
    modalText="Your text here" 
/>

<div class="flex flex-col items-center mx-2 h-full">
    <div class="flex flex-row items-center justify-left w-full g-8">
        <ListBox 
        bind:items={partitionList} 
        icon={diskIcon} 
        width="30em"
        on:select={handleSelect} 
        additionalIcons={[refreshIcon]}
        defaultItem={{ name: 'Select Drive' }}
        />
        <div class="flex flex-row items-center justify-center rounded-full bg-[#1A1A1A] py-4 pl-4 border-2 border-[#2F2F2F] w-full h-[60px]"></div>
    </div>
</div>

<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("desktop")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("partition")}>Continue</button>
    </div>
</Footer>