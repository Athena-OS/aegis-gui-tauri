<script lang="ts">
  interface Card {
    title: string;
    desc: string;
    value: string;
    icon?: string;
    checked?: boolean;
    disabled?: boolean;
  }

  export let title: string;
  export let cards: Card[];
  export let warning: boolean = true;

  function onChange(e: Event) {
    dispatch("change", (e as CustomEvent));
  }

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
</script>

<fieldset class="grid grid-cols-2 gap-8 w-full">
  <legend class="py-4 font-medium text-neutral-300 text-center">{title}</legend>
  {#each cards as card}
    <div class="relative">
      <input
        class="absolute top-2 right-2 radio-btn"
        id={card.value}
        type="radio"
        name="radio-group"
        value={card.value}
        checked={card.checked || false}
        disabled={card.disabled || false }
        on:change={onChange}
      />
      <label
        class:aspect-square={card.icon}
        class="h-full w-full p-6 flex flex-col items-center ring ring-gray-700 radio-btn-label rounded-3xl relative"
        for={card.value}
      >
        <div
          class="text-center {!card.icon ? 'text-4xl mt-10 -mb-8' : 'text-lg'}"
        >
          {card.title}
        </div>
        <div class="my-auto">
          <img class="h-20" src={card.icon} alt="" />
        </div>
        <div
          class:text-red-500={warning}
          class="text-xs absolute left-0 bottom-0 px-4 py-2"
        >
          {card.desc}
        </div></label
      >
    </div>
  {/each}
</fieldset>

<style>
  .radio-btn {
    @apply appearance-none h-7 w-7 bg-neutral-500 rounded-full;
  }
  .radio-btn:checked {
    @apply bg-primary-500;
  }
  .radio-btn:checked::before {
    content: url(../../assets/icons/check-bg-yellow.svg);
  }
  .radio-btn:checked + .radio-btn-label {
    @apply ring-yellow-500;
  }
</style>
