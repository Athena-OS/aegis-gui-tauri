<script lang="ts">
    import Footer from "../ProgressStepper.svelte";
    import DialogComponent from '../InfoDialog.svelte';
    import ListBox from "../ListBox.svelte";

    import checkIcon from "../../assets/icons/check-bg-yellow.svg";
    import refreshIcon from "../../assets/icons/refresh.svg";
    import diskIcon from "../../assets/icons/disk.svg";
    import eraseDiskIcon from "../../assets/icons/erase-disk.svg";
    import manualDiskIcon from "../../assets/icons/manual-disk.svg";

    let partitionList = [
        { name: 'Samsung NVME SSD 500G' },
        // ... other names
    ];

    let selectedDrive = null;
    let selectedOption: null | string = null;

    $: canContinue = selectedDrive !== null && selectedDrive.name !== 'Select Drive' && selectedOption !== null;

    function handleSelect(event: CustomEvent<any>) {
        console.log("Selected item:", event.detail);
        selectedDrive = event.detail;
    }

    function handleOptionSelect(option: string) {
        selectedOption = option;
    }

    export let switchView: (viewName: string) => void;
</script>

<style>
    .selected {
        border-color: #FFB800;
    }
</style>

<DialogComponent 
    stepNumber="3" 
    title="Partitioning" 
    modalHeader="Header Here" 
    modalText="Your text here" 
/>

<div class="flex flex-col items-center gap-20">
    <ListBox 
        bind:items={partitionList} 
        icon={diskIcon} 
        label="Select Drive"
        width="30em"
        on:select={handleSelect} 
        additionalIcons={[refreshIcon]}
        defaultItem={{ name: 'Select Drive' }}
    />
    <div class="flex flex-row gap-8">
        <!-- Erase Disk Option -->
        <div 
            class={`flex flex-col bg-[#1A1A1A] w-56 h-64 rounded-3xl border-2 ${selectedOption === 'erase' ? 'selected' : 'border-[#2F2F2F]'} items-center p-4 gap-4 relative`} 
            on:click={() => handleOptionSelect('erase')}
            on:keydown={(e) => e.key === 'Enter' && handleOptionSelect('erase')}
            role="button"
            tabindex="0">

            <!-- Checkmark for selected option -->
            {#if selectedOption === 'erase'}
                <img src={checkIcon} alt="checked" class="absolute top-0 right-0 m-2 rounded-full w-6 h-6">
            {:else}
                <div class="absolute top-0 right-0 m-2 w-6 h-6 bg-[#2F2F2F] rounded-full"></div>
            {/if}

            <!-- Option Details -->
            <h3 class="font-medium text-2xl">Erase disk</h3>
            <img src={eraseDiskIcon} alt="erase disk" class="w-24">
            <p class="text-center">Wipes <b class="text-[#FFB800]">ALL</b> data on disk, please be certain.</p>
        </div>

        <!-- Manual Partition Option -->
        <div 
            class={`flex flex-col bg-[#1A1A1A] w-56 h-64 rounded-3xl border-2 ${selectedOption === 'manual' ? 'selected' : 'border-[#2F2F2F]'} items-center p-4 gap-4 relative`} 
            on:click={() => handleOptionSelect('manual')}
            on:keydown={(e) => e.key === 'Enter' && handleOptionSelect('manual')}
            role="button"
            tabindex="0">

            <!-- Checkmark for selected option -->
            {#if selectedOption === 'manual'}
                <img src={checkIcon} alt="checked" class="absolute top-0 right-0 m-2 rounded-full w-6 h-6">
            {:else}
                <div class="absolute top-0 right-0 m-2 w-6 h-6 bg-[#2F2F2F] rounded-full"></div>
            {/if}

            <!-- Option Details -->
            <h3 class="font-medium text-2xl">Manual</h3>
            <img src={manualDiskIcon} alt="manual partition disk" class="w-24">
            <p class="text-center">Adjust partition sizes manually.</p>
        </div>
    </div>
</div>

<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("desktop")}>Back</button>
        <button 
            class="primary-btn" 
            on:click={() => {
                selectedOption === 'manual' ? switchView("manualPartition") : switchView("erasePartition")
            }}
            disabled={!canContinue}
        >
            Continue
        </button>
    </div>
</Footer>
