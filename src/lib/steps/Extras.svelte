<script lang="ts">
    import DialogComponent from '../components/InfoDialog.svelte';

    import kernelIcon from "../../assets/icons/kernel-yellow.svg";
    import terminalIcon from "../../assets/icons/terminal-yellow.svg";
    import wrenchIcon from "../../assets/icons/wrench-yellow.svg";

</script>

<DialogComponent 
    stepNumber="3.1" 
    title="Extras" 
    modalHeader="Header Here" 
    modalText="Your text here" 
/>

<div class="flex flex-col items-center">
    <div class="flex flex-row flex-wrap mt-28 gap-10 max-w-5xl">
        <div class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer">
            <img src={kernelIcon} alt="kernel">
            <h3 class="font-semibold text-4xl">Kernel</h3>
        </div>
        <div class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer">
            <img src={terminalIcon} alt="kernel">
            <div>
                <h3 class="font-semibold text-4xl">Terminal</h3>
                <span class="font-semibold text-xl">& Shell</span>
            </div>
        </div>
        <div class="flex flex-row items-center gap-3 bg-[#1A1A1A] h-40 w-72 justify-center rounded-2xl hover:border-[#FFB800] hover:border-2 cursor-pointer">
            <img src={wrenchIcon} alt="kernel">
            <h3 class="font-semibold text-4xl">System</h3>
        </div>
    </div>
</div>