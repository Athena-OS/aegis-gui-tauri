<script lang="ts">
    import DialogComponent from '../InfoDialog.svelte';
    import Footer from "../ProgressStepper.svelte";

    import gnomeImage from "../../assets/gnome.svg";

    export let switchView: (viewName: string) => void;
</script>
  
<DialogComponent stepNumber="2" title="Select Environment" modalHeader="Header Here" modalText="Your text here" />
<div class="flex flex-col items-left mx-12 my-8">
    <div class="flex flex-row h-[38em] w-full gap-4">
        <div class="h-full bg-[#1A1A1A] w-56 rounded-3xl border-2 border-[#2F2F2F]">
        </div>
        <div class="flex flex-col h-full flex-1 gap-4">
            <div class="bg-[#1A1A1A] w-full h-32 rounded-3xl border-2 border-[#2F2F2F]">
            </div>
            <div class="w-full flex-1 relative">
                <div class="absolute inset-0">
                    <img src={gnomeImage} alt="gnome" class="w-full h-full object-cover rounded-3xl"> 
                </div>
            </div>
        </div>
    </div>
</div>



<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("keyboard")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("packages")}>Continue</button>
    </div>
</Footer>