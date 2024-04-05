<script lang="ts">
    import nix from "../assets/nix.svg"
    import arch from "../assets/arch.svg"
    import StepWrapper from "../lib/components/StepWrapper.svelte";
    import CardGroup from "../lib/components/CardGroup.svelte";
    import extrasStore from "../lib/stores/extrasStore";  
    $extrasStore.base = "arch";
  </script>  
  <StepWrapper
    title="Base system"
    dialogTitle="About base system"
    dialogContent="This is the base system. You can either select NixOS Distro or Arch Linux Distro"
    prev="/"
    next="/keyboard"  >
    <div class="flex flex-col items-center gap-10 w-full max-w-md mx-auto">     
      <CardGroup
        title="Which base system would you like to install ?"
        on:change={(event) => ($extrasStore.base = event.detail.target.value)}
        cards={[
          {
            title: "Arch Linux",
            desc: "Install Arch Linux Base.",
            value: "arch",
            icon: arch,
            checked: true,
          },
          {
            title: "NixOS base",
            desc: "Install NixOS Base",
            value: "nixos",
            icon: nix,
          },
        ]}
      />  
    </div>
  </StepWrapper>
  
 
  