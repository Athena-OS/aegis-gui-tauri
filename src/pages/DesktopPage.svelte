<script lang="ts">
  import akameThemeImage from "../assets/desktop/akame-theme.png";
  import graphiteThemeImage from "../assets/desktop/graphite-theme.png";
  import gruvboxThemeImage from "../assets/desktop/gruvbox-theme.png";
  import samuraiGirlThemeImage from "../assets/desktop/samurai-girl-theme.png";
  import SweetDarkThemeImage from "../assets/desktop/sweet-dark-theme.png";
  import xxeThemeImage from "../assets/desktop/xxe-theme.png";

  import environmentIcon from "../assets/icons/environment.svg";
  import themeIcon from "../assets/icons/theme.svg";
  import displayManagerIcon from "../assets/icons/desktop-manager.svg";

  import Dropdown from "../lib/components/Dropdown.svelte";
  import desktopStore from "../lib/stores/desktopStore";
  import StepWrapper from "../lib/components/StepWrapper.svelte";

  let environmentList = [
    {
      name: "GNOME",
      desc: "An elegant, GTK-powered desktop environment designed to help you get things done with ease, comfort and control.",
      selected: $desktopStore.environment === "GNOME",
    },
    {
      name: "KDE",
      desc: "",
      selected: $desktopStore.environment === "KDE",
    },
    {
      name: "MATE",
      desc: "",
      selected: $desktopStore.environment === "MATE",
    },
    {
      name: "XFCE",
      desc: "",
      selected: $desktopStore.environment === "XFCE",
    },
    {
      name: "Bspwm",
      desc: "",
      selected: $desktopStore.environment === "Bspwm",
    },
    {
      name: "Cinnamon",
      desc: "",
      selected: $desktopStore.environment === "Cinnamon",
    },
    {
      name: "Hyprland",
      desc: "",
      selected: $desktopStore.environment === "Hyprland",
    },
  ];

  let themeList = [
    {
      name: "Akame",
      img: akameThemeImage,
      selected: $desktopStore.theme === "Akame",
    },
    {
      name: "Graphite",
      img: graphiteThemeImage,
      selected: $desktopStore.theme === "Graphite",
    },
    {
      name: "Gruvbox",
      img: gruvboxThemeImage,
      selected: $desktopStore.theme === "Gruvbox",
    },
    // {
    //   name: "Hack The Box",
    //   img: "",
    // },
    {
      name: "Samurai Girl",
      img: samuraiGirlThemeImage,
      selected: $desktopStore.theme === "Samurai Girl",
    },
    {
      name: "Sweet Dark",
      img: SweetDarkThemeImage,
      selected: $desktopStore.theme === "Sweet Dark",
    },
    {
      name: "XXE",
      img: xxeThemeImage,
      selected: $desktopStore.theme === "XXE",
    },
  ];

  let displayManagerList = [
    {
      name: "GDM ( GNOME display manager )",
      selected:
        $desktopStore.displayManager === "GDM ( GNOME display manager )",
    },
    {
      name: "LightDM Neon",
      selected: $desktopStore.displayManager === "LightDM Neon",
    },
    {
      name: "LightDM Everblush",
      selected: $desktopStore.displayManager === "LightDM Everblush",
    },
    {
      name: "SDDM",
      selected: $desktopStore.displayManager === "SDDM",
    },
  ];

  let nextPage = "";
  function IsOkayToMoveNextPage() {
    if (
      $desktopStore.environment !== "default" &&
      $desktopStore.theme !== "default" &&
      $desktopStore.displayManager !== "default"
    ) {
      nextPage = "/packages";
    }
  }
  $: $desktopStore, IsOkayToMoveNextPage();
</script>

<StepWrapper
  title="Select Environment"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/keyboard"
  next={nextPage}
>
  <div class="flex h-3/4 w-full">
    <div class="w-1/2 h-full p-6">
      {#if $desktopStore.theme !== "default"}
        <img
          src={themeList[
            themeList.findIndex((theme) => theme.name === $desktopStore.theme)
          ].img}
          alt="Selected Theme"
          class="w-full h-full object-cover rounded-3xl"
        />
      {:else}
        <div
          class="w-full h-full flex justify-center items-center bg-neutral-900 rounded-3xl text-2xl text-neutral-500"
        ></div>
      {/if}
    </div>
    <div class="w-1/2 space-y-16">
      <Dropdown
        icon={environmentIcon}
        bind:items={environmentList}
        label="Select Environment"
        on:select={(event) => {
          $desktopStore.environment = event.detail.selected.name;
        }}
        defaultItem={{ name: "Select Environment" }}
      />
      {#if $desktopStore.environment !== "Hyprland" && $desktopStore.environment !== "Bspwm"}
        <Dropdown
          icon={themeIcon}
          bind:items={themeList}
          label="Select Theme"
          on:select={(event) => {
            $desktopStore.theme = event.detail.selected.name;
            $desktopStore.themeImage =
              themeList[
                themeList.findIndex(
                  (theme) => theme.name === event.detail.selected.name,
                )
              ].img;
          }}
          defaultItem={{ name: "Select Theme" }}
        />
      {/if}
      <Dropdown
        icon={displayManagerIcon}
        bind:items={displayManagerList}
        label="Select Display Manager"
        on:select={(event) =>
          ($desktopStore.displayManager = event.detail.selected.name)}
        defaultItem={{ name: "Select Display Manager" }}
      />
    </div>
  </div>
  <div class="w-full h-1/4 text-xl justify-center items-center flex space-x-4">
    <button class="bg-neutral-800 p-4 rounded-xl"
      >{$desktopStore.environment !== "default"
        ? $desktopStore.environment
        : "Environment"}</button
    >
    <h4 class="font-black text-4xl text-primary-400">+</h4>
    <button class="bg-neutral-800 p-4 rounded-xl"
      >{$desktopStore.theme !== "default"
        ? $desktopStore.theme
        : "Theme"}</button
    >
    <h4 class="font-black text-4xl text-primary-400">+</h4>
    <button class="bg-neutral-800 p-4 rounded-xl"
      >{$desktopStore.displayManager !== "default"
        ? $desktopStore.displayManager
        : "Display Manager"}</button
    >
    <h4 class="font-black text-4xl text-primary-400">=</h4>
    <button
      class="p-4 rounded-xl bg-neutral-800 border-2 border-primary-500 font-bold"
      >Your Desktop 🤩</button
    >
  </div>
</StepWrapper>

<style>
  .active {
    @apply bg-yellow-500 text-black;
  }
</style>
