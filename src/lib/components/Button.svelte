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
          primary: "bg-primary-500 disabled:bg-primary-200",
          secondary: "bg-gray-100 disabled:bg-gray-200",
          bordered:
            "border-2 border-inset border-primary-500 text-primary-500 disabled:border-primary-500/20 disabled:text-primary-500/20",
        },
        size: {
          medium: "px-4 py-2 text-lg h-[50px]",
          large: "px-6 py-4 text-xl",
        },
        fullWidth: {
          true: "w-full",
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
  export let fullWidth: ButtonProps["fullWidth"] = false;
  export let disabled: boolean = false;
</script>

<button class={buttonStyles({ variant, size, fullWidth })} {disabled} on:click>
  <slot />
</button>
