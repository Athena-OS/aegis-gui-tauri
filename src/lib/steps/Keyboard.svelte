<script lang="ts">
    import { createListbox } from 'svelte-headlessui'
	import Transition from 'svelte-transition'

    import DialogComponent from '../InfoDialog.svelte';
    import Footer from "../ProgressStepper.svelte";

    import keyboardIcon from "../../assets/keyboard-icon.svg";
    import langIcon from "../../assets/lang-icon.svg";
    import arrowDown from "../../assets/arrow-down-icon.svg";
    import keyboard from "../../assets/keyboard.svg";

    type Person = { name: string }
	type People = Person[]

	// prettier-ignore
	const people = [
        { name: 'Select Language' },
		{ name: 'Wade Cooper' },
		{ name: 'Arlene Mccoy' },
		{ name: 'Devon Webb' },
		{ name: 'Tom Cook' },
		{ name: 'Tanya Fox' },
		{ name: 'Hellen Schmidt' },
	]

	// TODO: type list so 'selected' isn't 'any'
	const listbox = createListbox<People>({ label: 'Actions', selected: people[0] })

	function onSelect(e: Event) {
		console.log('select', (e as CustomEvent).detail)
	}

    export let switchView: (viewName: string) => void;
</script>

<DialogComponent stepNumber="1" title="Select Keyboard" modalHeader="Header Here" modalText="Your text here" />
<div class="flex flex-col items-center gap-10 mt-10">
    <img src={keyboard} alt="athena-logo" class="w-[30em]"/>
    
    <div class="flex w-full flex-col items-center justify-center">
        <div class="w-[28em]">
            <div class="relative mt-1">
                <button
                    use:listbox.button
                    on:select={onSelect}
                    class="flex flex-row items-center cursor-pointer text-left gap-4 relative w-full rounded-full bg-[#1A1A1A] py-4 pl-4 border-2 border-[#2F2F2F] hover:border-[#FFB800]"
                >
                    <img src={langIcon} alt="language icon">
                    <span class="block truncate text-[0.9em] font-medium">{$listbox.selected.name}</span>
                    <span class="pointer-events-none absolute inset-y-0 right-1 flex items-center pr-2">
                        <img src={arrowDown} alt="arrow down" class="w-5">
                    </span>
                </button>
    
                <Transition show={$listbox.expanded} leave="transition ease-in duration-100" leaveFrom="opacity-100" leaveTo="opacity-0">
                    <ul
                        use:listbox.items
                        class="absolute mt-2 w-full overflow-auto rounded-2xl max-h-[15em] bg-[#1A1A1A] py-2 px-2 text-[0.9em] border-2 border-[#2F2F2F]"
                    >
                        {#each people as value, i}
                            {@const active = $listbox.active === value}
                            {@const selected = $listbox.selected === value}
                            <li
                                class="relative cursor-default select-none py-2 pl-2 pr-4 rounded-lg {active ? 'bg-[#FFB800] text-black' : 'text-white'}"
                                use:listbox.item={{ value }}
                            >
                                <span class="block truncate {selected ? 'font-medium' : 'font-normal'}">{value.name}</span>
                                {#if selected}
                                    <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-amber-600">
                                        
                                    </span>
                                {/if}
                            </li>
                        {/each}
                    </ul>
                </Transition>
            </div>
        </div>
    </div>
</div>
<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button class="primary-btn" on:click={() => switchView("initial")}>Back</button>
        <button class="primary-btn" on:click={() => switchView("keyboard")}>Continue</button>
    </div>
</Footer>
