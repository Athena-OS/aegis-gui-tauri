<script lang="ts">
    import { createDisclosure} from 'svelte-headlessui';
	import Transition from 'svelte-transition';

    import Footer from "../ProgressStepper.svelte";
    import DialogComponent from '../InfoDialog.svelte';

    import checkIconBlackBg from "../../assets/icons/check-bg-black.svg";
    import checkIconYellowBg from "../../assets/icons/check-bg-yellow.svg";
    import dashIconBlackBg from "../../assets/icons/dash-bg-black.svg";
    import dashIconYellowBg from "../../assets/icons/dash-bg-yellow.svg";

    import packagesIcon from "../../assets/icons/packages-icon.svg";
    import arrowDownIcon from "../../assets/icons/arrow-down-black.svg";

    const refund = createDisclosure({ label: 'Refund Policy', expanded: false })
	const support = createDisclosure({ label: 'Technical Support', expanded: false })

    let isHovered = false;
    let isFocused = false;

    let options = [
        { label: 'Option 1', value: 'option1' },
        { label: 'Option 2', value: 'option2' },
        { label: 'Option 3', value: 'option3' }
    ];

    let selectedOptions = new Set();

    function toggleSelection(value) {
        if (selectedOptions.has(value)) {
            selectedOptions.delete(value);
        } else {
            selectedOptions.add(value);
        }

        selectedOptions = new Set([...selectedOptions]);
    }


    let allSelected = false; // New reactive variable to observe if all options are selected

    $: {
        allSelected = options.every(option => selectedOptions.has(option.value));
    }

    export let switchView: (viewName: string) => void;

</script>


<DialogComponent stepNumber="3" title="Select Packages" modalHeader="Header Here" modalText="Your text here" />
<div class="flex flex-col items-center justify-center mt-12">
    <div class="flex flex-row bg-[#1A1A1A] w-[80em] h-[36em] rounded-3xl border-2 border-[#2F2F2F]">
        <div class="flex flex-col w-72 p-4">
            <div class="flex flex-row items-center gap-2">
                <img src={packagesIcon} alt="packages" class="w-5">
                <h3 class="font-medium text-md">Packages</h3>
            </div>
            <div class="h-0.5 bg-[#2F2F2F] w-auto mt-3 mb-3"></div>
            <div class="w-full">
                <button
                use:refund.button
                class="flex flex-row items-center gap-2 w-full rounded-full px-2 py-2 text-left text-sm font-medium 
                    {($refund.expanded || isHovered || isFocused) ? 'bg-secondary-container text-black' : 'text-white'}"
                on:mouseover={() => isHovered = true}
                on:mouseout={() => isHovered = false}
                on:focus={() => isFocused = true}
                on:blur={() => isFocused = false}
                >
                    <img 
                        src={
                            allSelected 
                                ? ($refund.expanded || isHovered || isFocused ? checkIconBlackBg : checkIconYellowBg )
                                : ($refund.expanded || isHovered || isFocused ? dashIconBlackBg : dashIconYellowBg )
                        } 
                        alt="status" class="w-7" 
                    >
                    <span class="font-medium text-sm">GPU utilities</span>
                    {#if $refund.expanded || isHovered || isFocused}
                        <img src={arrowDownIcon} alt="arrow down" class="w-7 ml-auto {$refund.expanded ? 'rotate-180 transform' : ''}">
                    {/if}
                </button>

                {#if $refund.expanded}
                    <div class="flex px-4 py-5 gap-2">
                        <div class="border-l-2 border-[#2F2F2F] min-h-[fit-content]"></div>
                        <div class="flex flex-col gap-4 ml-2">
                            {#each options as option (option.value)}
                                <label class="flex items-center cursor-pointer">
                                    <input 
                                        type="checkbox" 
                                        value={option.value}
                                        class="hidden"
                                        on:change={() => toggleSelection(option.value)}
                                    />
                                    <span 
                                        class={`w-6 h-6 rounded-full inline-block 
                                            mr-2
                                            ${selectedOptions.has(option.value) ? '' : 'bg-[#2F2F2F]'}`}
                                    >
                                        {#if selectedOptions.has(option.value)}
                                            <img src={checkIconYellowBg} alt="Checked" class="w-full h-full" />
                                        {/if}
                                    </span>
                                    {option.label}
                                </label>
                            {/each}
                        </div>
                    </div>
                {/if}    
                  
                <button
                use:support.button
                class="flex flex-row items-center gap-2 w-full rounded-full px-2 py-2 text-left text-sm font-medium 
                    {($support.expanded || isHovered || isFocused) ? 'bg-secondary-container text-black' : 'text-white'}"
                on:mouseover={() => isHovered = true}
                on:mouseout={() => isHovered = false}
                on:focus={() => isFocused = true}
                on:blur={() => isFocused = false}
                >
                    <img 
                        src={
                            allSelected 
                                ? ($support.expanded || isHovered || isFocused ? checkIconBlackBg : checkIconYellowBg )
                                : ($support.expanded || isHovered || isFocused ? dashIconBlackBg : dashIconYellowBg )
                        } 
                        alt="status" class="w-7" 
                    >
                    <span class="font-medium text-sm">GPU utilities</span>
                    {#if $support.expanded || isHovered || isFocused}
                        <img src={arrowDownIcon} alt="arrow down" class="w-7 ml-auto {$support.expanded ? 'rotate-180 transform' : ''}">
                    {/if}
                </button>

                {#if $support.expanded}
                    <div class="flex px-4 py-5 gap-2">
                        <div class="border-l-2 border-[#2F2F2F] min-h-[fit-content]"></div>
                        <div class="flex flex-col gap-4 ml-2">
                            {#each options as option (option.value)}
                                <label class="flex items-center cursor-pointer">
                                    <input 
                                        type="checkbox" 
                                        value={option.value}
                                        class="hidden"
                                        on:change={() => toggleSelection(option.value)}
                                    />
                                    <span 
                                        class={`w-6 h-6 rounded-full inline-block 
                                            mr-2
                                            ${selectedOptions.has(option.value) ? '' : 'bg-[#2F2F2F]'}`}
                                    >
                                        {#if selectedOptions.has(option.value)}
                                            <img src={checkIconYellowBg} alt="Checked" class="w-full h-full" />
                                        {/if}
                                    </span>
                                    {option.label}
                                </label>
                            {/each}
                        </div>
                    </div>
                {/if}    
            </div>
        </div>
        <div class="w-0.5 h-full bg-[#2F2F2F]"></div>
    </div>
</div>

<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("initial")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("packages")}>Continue</button>
    </div>
</Footer>
