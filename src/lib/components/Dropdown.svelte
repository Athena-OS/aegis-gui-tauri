<script lang="ts">
  import { createListbox } from "svelte-headlessui";
  import Transition from "svelte-transition";
  import downArrow from "../../assets/icons/arrow-down-white.svg";

  // External props
  export let icon: string = "";

  type ListItem = { name: string };
  export let label: string | null = null;
  export let defaultItem: ListItem = { name: "Default Item" };
  export let items: ListItem[] = [defaultItem];

  const listbox = createListbox({ label: "Dropdown", selected: defaultItem });

  function onSelect(e: Event) {
    dispatch("select", (e as CustomEvent).detail);
  }

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
</script>

<div class="flex w-full flex-col items-center justify-center">
  <div class="w-full">
    {#if label}
      <p class="text-left font-medium text-neutral-400 mb-2">{label}</p>
    {/if}
    <div class="relative mt-1">
      <button
        class="w-full h-[50px] flex relative items-center space-x-4 text-left rounded-full overflow-hidden border-2 border-inset border-neutral-800 px-3.5 text-lg hover:border-primary-500 outline-none bg-neutral-900"
        use:listbox.button
        on:select={onSelect}
      >
        {#if icon}
          <img src={icon} alt="dropdown icon" />
        {/if}
        <span class="block truncate text-[0.9em] font-medium"
          >{$listbox.selected.name}</span
        >
        <img
          class="absolute right-3 top-0 bottom-0 my-auto"
          src={downArrow}
          alt=""
        />
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
              class="relative cursor-default select-none py-2 pl-2 rounded-lg {active
                ? 'bg-[#FFB800] text-black'
                : 'text-white'}"
              use:listbox.item={{ value }}
            >
              <span
                class="block truncate {selected
                  ? 'font-medium'
                  : 'font-normal'}">{value.name}</span
              >
            </li>
          {/each}
        </ul>
      </Transition>
    </div>
  </div>
</div>
