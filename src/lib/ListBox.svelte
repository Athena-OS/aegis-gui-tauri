<script lang="ts">
    import { createListbox } from 'svelte-headlessui';
	import Transition from 'svelte-transition';
    import arrowDown from "../assets/icons/arrow-down-white.svg";
    
    // External props
    export let icon: string;
    export let width: string = '28em';
    export let additionalIcons: string[] = [];
    export let fullWidth: boolean = true;

    type ListItem = { name: string };
    export let label: string | null = null;
    export let defaultItem: ListItem = { name: 'Default Item' };
    export let items: ListItem[] = [defaultItem];

    const listbox = createListbox({ label: 'Dropdown', selected: defaultItem });

    function onSelect(e: Event) {
		dispatch('select', (e as CustomEvent).detail);
	}

    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
</script>

<div class={fullWidth ? "flex w-full flex-col items-center justify-center" : "flex flex-col items-center justify-center"}>
    <div style="width: {width};">
        {#if label}
            <p class="text-[#B0B0B0] text-left font-semibold mb-2">{label}</p>
        {/if}
        <div class="relative mt-1">
            <button
            use:listbox.button
            on:select={onSelect}
            class="relative flex flex-row items-center justify-between cursor-pointer text-left w-full rounded-full bg-[#1A1A1A] py-4 pl-4 border-2 border-[#2F2F2F] hover:border-[#FFB800]"
            >
                <div class="flex items-center gap-4">
                    <img src={icon} alt="dropdown icon">
                    <span class="block truncate text-[0.9em] font-medium">{$listbox.selected.name}</span>
                </div>
            
                <div class="flex items-center gap-x-2 pr-2">
                    {#each additionalIcons as iconUrl}
                        <img src={iconUrl} alt="icon" class="w-5">
                    {/each}
                    <img src={arrowDown} alt="arrow down" class="w-5">
                </div>
            </button>
        
            
            <Transition show={$listbox.expanded}>
                <ul
                    use:listbox.items
                    class="absolute mt-2 w-full overflow-auto rounded-2xl max-h-[15em] bg-[#1A1A1A] py-2 px-2 text-[0.9em] border-2 border-[#2F2F2F] focus:outline-none focus:ring-0 focus:shadow-none z-10"
                >
                    {#each items as value}
                        {@const active = $listbox.active === value}
                        {@const selected = $listbox.selected === value}
                        <li
                            class="relative cursor-default select-none py-2 pl-2 rounded-lg {active ? 'bg-[#FFB800] text-black' : 'text-white'}"
                            use:listbox.item={{ value }}
                        >
                            <span class="block truncate {selected ? 'font-medium' : 'font-normal'}">{value.name}</span>
                        </li>
                    {/each}
                </ul>
            </Transition>
        </div>
    </div>
</div>

<style lang="scss">
    ::-webkit-scrollbar {
        width: 5px;
    }

    ::-webkit-scrollbar-thumb {
        background-color: #2F2F2F;
        border-radius: 5px;
    }

    ::-webkit-scrollbar-button {
        width: 0;
        height: 0;
    }

    ::-webkit-scrollbar-thumb:active {
        background-color: #FFA020;
    }

    ::-webkit-scrollbar-corner {
        background-color: #2F2F2F;
    }
</style>
