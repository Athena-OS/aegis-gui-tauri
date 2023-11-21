<script lang="ts">
  import checkIconBlackBg from "../../assets/icons/check-bg-black.svg";
  import checkIconYellowBg from "../../assets/icons/check-bg-yellow.svg";
  import dashIconBlackBg from "../../assets/icons/dash-bg-black.svg";
  import dashIconYellowBg from "../../assets/icons/dash-bg-yellow.svg";
  import arrowDownIcon from "../../assets/icons/arrow-down-black.svg";

  export let label: string;
  export let options: { label: string; value: string; selected?: boolean }[];
  export let expanded: boolean = false;
  let allSelected: boolean = false;
  let isHovered: boolean = false;
  let isFocused: boolean = false;

  $: allSelected = options.every((option) => option.selected);
</script>

<button
  class="flex flex-row items-center gap-2 w-full rounded-full p-1 text-left text-sm font-medium
          {isHovered || isFocused ? 'bg-primary-500 text-black' : 'text-white'}"
  on:mouseenter={() => (isHovered = true)}
  on:mouseleave={() => (isHovered = false)}
  on:focus={() => (isFocused = true)}
  on:blur={() => (isFocused = false)}
  on:click={() => (expanded = !expanded)}
>
  <img
    src={allSelected
      ? isHovered || isFocused
        ? checkIconBlackBg
        : checkIconYellowBg
      : isHovered || isFocused
      ? dashIconBlackBg
      : dashIconYellowBg}
    alt="status"
    class="w-7"
  />
  <span class="font-medium text-sm">{label}</span>
  {#if isHovered || isFocused}
    <img
      src={arrowDownIcon}
      alt="arrow down"
      class="w-6 ml-auto mr-1 {expanded ? 'rotate-180 transform' : ''}"
    />
  {/if}
</button>
{#if expanded}
  <div class="flex p-2 gap-2">
    <div class="border-l-2 border-[#2F2F2F] min-h-[fit-content]" />
    <div class="flex flex-col gap-4 ml-2">
      {#each options as option, index (option.value)}
        <label class="flex items-center cursor-pointer">
          <input
            type="checkbox"
            bind:checked={option.selected}
            class="hidden"
          />
          <span
            class={`w-7 h-7 rounded-full inline-block mr-2 ${
              option.selected ? "" : "bg-[#2F2F2F]"
            }`}
          >
            {#if option.selected}
              <img
                src={checkIconYellowBg}
                alt="Checked"
                class="w-full h-full"
              />
            {/if}
          </span>
          {option.label}
        </label>
      {/each}
    </div>
  </div>
{/if}
