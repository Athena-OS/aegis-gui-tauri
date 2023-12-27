<script lang="ts">
  import uuid from "../utils/uuid";

  export let label = "Test Keyboard"; // default label text
  export let placeholderText = "Type to test.."; // default input placeholder text
  export let inputType: string = "text"; // default
  export let value: string = ""; // default input placeholder text
  export let isDisabled: boolean = false; // default input placeholder text

  export let rightLabel: string = "";

  let inputID = uuid(); // just to generate a unique id for label

  function onChangeValue(e: any) {
    value = e.target.value;
  }
</script>

<div class="w-full space-y-3">
  <label class="font-medium text-neutral-400" for={inputID}>
    {label}
  </label>
  <div
    class="w-full h-[50px] relative border-gray-700 focus-within:border-primary-500 rounded-full overflow-hidden border-2"
  >
    <input
      id={inputID}
      class="w-full h-full placeholder:text-neutral-400 outline-none bg-gray-800 px-4"
      type={inputType}
      placeholder={placeholderText}
      value={value}
      on:change={onChangeValue}
      disabled={isDisabled}
    />
    {#if rightLabel !== ""}
      <div
        class="h-full w-fit px-6 bg-primary-500 absolute top-0 bottom-0 my-auto right-0 text-black flex justify-center items-center"
      >
        {rightLabel}
      </div>
    {/if}
  </div>
</div>
