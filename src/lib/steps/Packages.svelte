<script lang="ts">
    import { createDisclosure} from 'svelte-headlessui';
	import Transition from 'svelte-transition';

    import Footer from "../ProgressStepper.svelte";
    import DialogComponent from '../InfoDialog.svelte';

    import packagesIcon from "../../assets/icons/packages-icon.svg";

    const refund = createDisclosure({ label: 'Refund Policy', expanded: false })
	const support = createDisclosure({ label: 'Technical Support', expanded: false })

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
                <div class="mx-auto w-full max-w-md rounded-2xl">
                    <div>
                        <button
                            use:refund.button
                            class="flex w-full justify-between rounded-lg bg-secondary-container px-4 py-2 text-left text-sm font-medium"
                        >
                            <span>What is your refund policy?</span>
               
                        </button>
                        {#if $refund.expanded}
                        
                            <div class="space-y-2">
                                {#each options as option (option.value)}
                                    <label class="flex items-center cursor-pointer">
                                        <input 
                                            type="checkbox" 
                                            value={option.value}
                                            class="hidden"
                                            on:change={() => toggleSelection(option.value)}
                                        />
                                        <span 
                                            class={`w-6 h-6 border-2 rounded-full inline-block 
                                                mr-2
                                                ${selectedOptions.has(option.value) ? 'bg-yellow-400' : 'bg-gray-400'}`}
                                        >
                                            {#if selectedOptions.has(option.value)}
                                                <!-- You can use an SVG or Font Awesome icon for the X here -->
                                                <!-- I'm using a simple "X", but you can replace it with a more stylish representation -->
                                                <span class="block text-white text-xs font-bold leading-6 text-center">X</span>
                                            {/if}
                                        </span>
                                        {option.label}
                                    </label>
                                {/each}
                            </div>
                        
                        {/if}
                    </div>
                    <div class="mt-2">
                        <button
                            use:support.button
                            class="flex w-full justify-between rounded-lg bg-purple-100 px-4 py-2 text-left text-sm font-medium text-purple-900 hover:bg-purple-200 focus:outline-none focus-visible:ring focus-visible:ring-purple-500 focus-visible:ring-opacity-75"
                        >
                            <span>Do you offer technical support?</span>
                         
                        </button>
                        {#if $support.expanded}
                            <div use:support.panel class="px-4 pt-4 pb-2 text-sm text-gray-500">No.</div>
                        {/if}
                    </div>
                </div>
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
