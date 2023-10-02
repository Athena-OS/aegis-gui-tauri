<script lang="ts">
    import Footer from "../ProgressStepper.svelte";
    import DialogComponent from '../InfoDialog.svelte';
    import ListBox from "../ListBox.svelte";

    import diskIcon from "../../assets/icons/disk.svg";
    import eraseDiskIcon from "../../assets/icons/erase-disk.svg";

    let paritionList = [
        { name: 'Samsung NVME SSD 500G' },
        { name: 'Samsung NVME SSD 500G' },
        { name: 'Samsung NVME SSD 500G' },
        { name: 'Samsung NVME SSD 500G' },
        { name: 'Samsung NVME SSD 500G' },
        { name: 'Samsung NVME SSD 500G' },
        { name: 'Samsung NVME SSD 500G' },
        // ... other names
    ];

    function handleSelect(event: CustomEvent<any>) {
        console.log("Selected item:", event.detail);
    }


    export let switchView: (viewName: string) => void;
</script>

<DialogComponent stepNumber="3" title="Partitioning" modalHeader="Header Here" modalText="Your text here" />

<div class="flex flex-col items-center">
    <ListBox bind:items={paritionList} icon={diskIcon} label="Select Drive" on:select={handleSelect} defaultItem={{ name: 'Select Drive' }}/>
</div>

<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("desktop")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("partition")}>Continue</button>
    </div>
</Footer>


