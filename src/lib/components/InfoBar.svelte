<script lang="ts">
  import { cva } from "class-variance-authority";
  import type { VariantProps } from "class-variance-authority";

  type InfoBarProps = VariantProps<typeof infoBarStyles>;

  const infoBarStyles = cva(["w-3 h-3 rounded-full"], {
    variants: {
      accent: {
        info: "bg-blue-500",
      },
    },
    defaultVariants: {
      accent: "info",
    },
  });

  export let accent: InfoBarProps["accent"];
</script>

<div class="fixed bottom-0 left-0 w-full p-4">
  <div
    class="w-fit px-3 py-2 text-sm bg-white flex items-center space-x-2 text-zinc-800 rounded-full"
  >
    <div class={infoBarStyles({ accent })} />
    <p class="font-medium">
      We'll guide you through a few steps to install AthenaOS on your system.
    </p>
  </div>
</div>
