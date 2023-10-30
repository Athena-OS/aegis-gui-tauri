<script lang="ts">
    import DialogComponent from '../components/InfoDialog.svelte';
    import PackageButton from "../components/PackageButton.svelte";

    import packagesIcon from "../../assets/icons/packages-icon.svg";

    let gpuOptions = [
        { label: 'Option 1', value: 'option1' },
        { label: 'Option 2', value: 'option2' },
        { label: 'Option 3', value: 'option3' }
    ];

    let otherOptions = [
        { label: 'Option 1', value: 'option1' },
        { label: 'Option 2', value: 'option2' },
        { label: 'Option 3', value: 'option3' }
    ];

    let refundExpanded = false;
    let supportExpanded = false;

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
                <PackageButton label="GPU utilities" options={gpuOptions} bind:expanded={refundExpanded} />
                <PackageButton label="Technical Support" options={otherOptions} bind:expanded={supportExpanded} />
            </div>
        </div>
        <div class="w-0.5 h-full bg-[#2F2F2F]"></div>
        <div class="flex flex-col px-4 py-5 gap-2">
            {#if refundExpanded}
                <h3 class="text-3xl font-medium text-[#FFB800]">GPU Utilities</h3>
                <p class="font-medium max-w-xl">
                    GPU utilities refer to software and tools designed to monitor, manage, and optimize your Graphics Processing Unit (GPU). These utilities help users make the most of their GPU's capabilities, whether it's for gaming, content creation, machine learning, or other GPU-intensive tasks. Here are some key points and functionalities that come under GPU utilities:
                </p>
            {/if}
        </div>

    </div>
</div>