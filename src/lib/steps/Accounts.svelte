<script lang="ts">
    import Footer from "../ProgressStepper.svelte";
    import DialogComponent from '../InfoDialog.svelte';

    export let switchView: (viewName: string) => void;
</script>

<DialogComponent 
    stepNumber="3.1" 
    title="Manual Partitioning" 
    modalHeader="Header Here" 
    modalText="Your text here" 
/>

<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("desktop")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("extras")}>Continue</button>
    </div>
</Footer>