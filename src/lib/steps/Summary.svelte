<script lang="ts">
    import DialogComponent from '../components/InfoDialog.svelte';
</script>

<DialogComponent 
    stepNumber="7" 
    title="Summary" 
    modalHeader="Header Here" 
    modalText="Your text here" 
/>

<div class="flex w-full flex-col items-center justify-center">
    <div class="w-[28em]">
        <p class="text-[#B0B0B0] text-left font-semibold mb-2">Label</p>
        <div class="flex flex-row items-center cursor-pointer text-left gap-4 relative w-full rounded-full bg-[#1A1A1A] py-4 pl-4 text-[#B0B0B0] font-medium border-2 border-[#2F2F2F] focus:outline-none focus:ring-2 focus:ring-[#FFB800] focus:shadow-none">
            <span>test</span>
        </div>
    </div>
</div>