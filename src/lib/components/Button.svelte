<script lang="ts">
  import { cva } from "class-variance-authority";
  import type { VariantProps } from "class-variance-authority";

  type ButtonProps = VariantProps<typeof buttonStyles>;

  const buttonStyles = cva(
    [
      "rounded-full text-black font-medium flex items-center justify-center space-x-2",
    ],
    {
      variants: {
        variant: {
          primary: "bg-primary-500",
          bordered: "ring ring-primary-500 ring-inset text-primary-500",
        },
        size: {
          medium: "px-4 py-2 text-lg h-[50px]",
          large: "px-6 py-4 text-xl",
        },
      },
      defaultVariants: {
        variant: "primary",
        size: "medium",
      },
    }
  );

  export let variant: ButtonProps["variant"] = "primary";
  export let size: ButtonProps["size"] = "medium";
</script>

<button class={buttonStyles({ variant, size })} on:click>
  <slot />
</button>
