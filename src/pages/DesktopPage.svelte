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
    import extrasStore from "../lib/stores/extrasStore";

  let environmentList = [
    {
      name: "GNOME",
      desc: "An elegant, GTK-powered desktop environment designed to help you get things done with ease, comfort and control.",
      selected: $desktopStore.environment === "gnome",
    },
    {
      name: "KDE",
      desc: "",
      selected: $desktopStore.environment === "kde",
    },
    {
      name: "MATE",
      desc: "",
      selected: $desktopStore.environment === "mate",
    },
    {
      name: "XFCE",
      desc: "",
      selected: $desktopStore.environment === "xfce",
    },
    {
      name: "Bspwm",
      desc: "",
      selected: $desktopStore.environment === "bspwm",
    },
    {
      name: "Cinnamon",
      desc: "",
      selected: $desktopStore.environment === "cinnamon",
    },
    {
      name: "Hyprland",
      desc: "",
      selected: $desktopStore.environment === "hyprland",
    },
  ];

  let environmentListNix = [
    {
      name: "GNOME",
      desc: "An elegant, GTK-powered desktop environment designed to help you get things done with ease, comfort and control.",
      selected: $desktopStore.environment === "gnome",
    },
        {
      name: "MATE",
      desc: "",
      selected: $desktopStore.environment === "mate",
    },
    {
      name: "Cinnamon",
      desc: "",
      selected: $desktopStore.environment === "cinnamon",
    },    
  ];
  let themeList = [
    {
      name: "Akame",
      img: akameThemeImage,
      selected: $desktopStore.theme === "akame",
    },
    {
      name: "Graphite",
      img: graphiteThemeImage,
      selected: $desktopStore.theme === "graphite",
    },
    {
      name: "Cyborg",
      img: gruvboxThemeImage,
      selected: $desktopStore.theme === "cyborg",
    },
    {
       name: "HackTheBox",
       img: "",
       selected: $desktopStore.theme === "hackthebox",
     },
    {
      name: "Samurai",
      img: samuraiGirlThemeImage,
      selected: $desktopStore.theme === "samurai",
    },
    {
      name: "Sweet",
      img: SweetDarkThemeImage,
      selected: $desktopStore.theme === "sweet",
    },
    /*{
      name: "XXE",
      img: xxeThemeImage,
      selected: $desktopStore.theme === "xxe",
    },*/
  ];

  let displayManagerList = [
    {
      name: "GDM",
      selected:
        $desktopStore.displayManager === "gdm",
    },
    {
      name: "LightDM",
      selected: $desktopStore.displayManager === "lightdm",
    },
  
    {
      name: "SDDM",
      selected: $desktopStore.displayManager === "ssdm",
    },
  ];
  let displayManagerListNix = [
    {
      name: "GDM",
      selected:
        $desktopStore.displayManager === "gdm",
    },
    {
      name: "LightDM",
      selected: $desktopStore.displayManager === "lightdm",
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
  dialogTitle="About Environment"
  dialogContent="In this page, you select your desktop environment"
  prev="/keyboard"
  next={nextPage}
>
  <div class="flex h-3/4 w-full">
    <div class="w-1/2 h-full p-6">
      {#if $desktopStore.theme !== "default"}
        <img
          src={themeList[
            themeList.findIndex((theme) => theme.name.toLowerCase() === $desktopStore.theme)
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
      {#if $extrasStore.base == "arch"}
        <Dropdown
          icon={environmentIcon}
          bind:items={environmentList}
          label="Select Environment"
          on:select={(event) => {
            $desktopStore.environment =
              event.detail.selected.name.toLowerCase();
          }}
          defaultItem={{ name: "Select Environment" }}
        />
      {:else}
        <Dropdown
          icon={environmentIcon}
          bind:items={environmentListNix}
          label="Select Environment"
          on:select={(event) => {
            $desktopStore.environment =
              event.detail.selected.name.toLowerCase();
          }}
          defaultItem={{ name: "Select Environment" }}
        />
      {/if}
      {#if $desktopStore.environment !== "hyprland" && $desktopStore.environment !== "bspwm"}
        <Dropdown
          icon={themeIcon}
          bind:items={themeList}
          label="Select Theme"
          on:select={(event) => {
            $desktopStore.theme = event.detail.selected.name.toLowerCase();
            $desktopStore.themeImage =
              themeList[
                themeList.findIndex(
                  (theme) => theme.name.toLowerCase() === event.detail.selected.name.toLowerCase(),
                )
              ].img;
          }}
          defaultItem={{ name: "Select Theme" }}
        />
      {/if}
      {#if $extrasStore.base == "arch"}
      <Dropdown
        icon={displayManagerIcon}
        bind:items={displayManagerList}
        label="Select Display Manager"
        on:select={(event) =>
          ($desktopStore.displayManager = event.detail.selected.name.toLowerCase())}
        defaultItem={{ name: "Select Display Manager" }}
      />
      {:else}
      <Dropdown
      icon={displayManagerIcon}
      bind:items={displayManagerListNix}
      label="Select Display Manager"
      on:select={(event) =>
        ($desktopStore.displayManager = event.detail.selected.name.toLowerCase())}
      defaultItem={{ name: "Select Display Manager" }}
    />
    {/if}
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
