<script lang="ts">
    import refreshIcon from "../assets/icons/refresh.svg";
    import diskIcon from "../assets/icons/disk.svg";
    import eraseDiskIcon from "../assets/icons/erase-disk.svg";
    import manualDiskIcon from "../assets/icons/manual-disk.svg";
    import nix from "../assets/nix.svg"
    import arch from "../assets/arch.svg"
    import StepWrapper from "../lib/components/StepWrapper.svelte";
    import Dropdown2 from "../lib/components/Dropdown2.svelte";
    import CardGroup from "../lib/components/CardGroup.svelte";
    import Button from "../lib/components/Button.svelte";
    import extrasStore from "../lib/stores/extrasStore";
  
    import { bytesToGB } from "../lib/utils/functions";
    import { resetPartitionStore } from "../lib/stores/partitionStore";
    interface Card {
      title: string;
      desc: string;
      value: string;
      icon?: string;
      checked?: boolean;
    }
  
    
    $extrasStore.base = "arch";
    
     
    
  
  
  </script>
  
  <StepWrapper
    title="Base system"
    dialogTitle="About base system"
    dialogContent="This is the base system"
    prev="/"
    next="/keyboard"
  >
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
  
  <style>
    .selected {
      border-color: #ffb800;
    }
  
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
  