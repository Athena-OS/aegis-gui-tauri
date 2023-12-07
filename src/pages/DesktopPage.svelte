<script lang="ts">
  import gnomeImage from "../assets/desktop/gnome.webp";
  import kdeImage from "../assets/desktop/kde.png";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  let listItems = [
    {
      name: "GNOME",
      desc: "An elegant, GTK-powered desktop environment designed to help you get things done with ease, comfort and control.",
      img: gnomeImage,
    },
    {
      name: "KDE Plasma",
      desc: "",
      img: kdeImage,
    },
    {
      name: "XFCE",
      desc: "",
      img: "",
    },
    {
      name: "Hyprland",
      desc: "",
      img: "",
    },
  ];

  let currentSelected = 0;

  function selectName(index: number) {
    currentSelected = index;
  }
</script>

<StepWrapper
  title="Select Environment"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/keyboard"
  next="/packages"
>
  <div class="flex h-full flex-col items-left">
    <div class="flex flex-row h-full w-full gap-4">
      <div
        class="h-full bg-neutral-900 w-56 rounded-3xl border border-neutral-800 p-2"
      >
        <ul class="space-y-2">
          {#each listItems as { name, desc }, index (index)}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <li
              class="p-2 cursor-pointer rounded-xl"
              class:active={currentSelected === index}
              on:click={() => selectName(index)}
            >
              {name}
            </li>
          {/each}
        </ul>
      </div>
      <div class="flex flex-col h-full flex-1 gap-4">
        <div
          class="bg-neutral-900 w-full h-32 rounded-3xl border border-neutral-800 px-4 py-2 text-xl"
        >
          {#if listItems[currentSelected]}
            <div class="description">
              <div class="font-semibold">{listItems[currentSelected].name}</div>
              <p class="text-neutral-400">{listItems[currentSelected].desc}</p>
            </div>
          {/if}
        </div>
        <div class="w-full flex-1 relative">
          <div class="absolute inset-0">
            {#if listItems[currentSelected]}
              <img
                src={listItems[currentSelected].img}
                alt="gnome"
                class="w-full h-full object-cover rounded-3xl"
              />
            {/if}
          </div>
        </div>
      </div>
    </div>
  </div>
</StepWrapper>

<style>
  .active {
    @apply bg-yellow-500 text-black;
  }
</style>
