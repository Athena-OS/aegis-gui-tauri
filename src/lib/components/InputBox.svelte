<script>
  import uuid from "../utils/uuid";

  export let label = "Test Keyboard"; // default label text
  export let placeholderText = "Type to test.."; // default input placeholder text

  let inputID = uuid(); // just to generate a unique id for label
</script>

<div class="w-full space-y-3">
  <label class="font-medium text-neutral-400" for={inputID}>
    {label}
  </label>
  <div class="w-full h-[50px]">
    <input
      id={inputID}
      class="w-full h-full placeholder:text-neutral-400 rounded-full overflow-hidden ring ring-inset ring-neutral-900 px-4 focus:ring-primary-500 outline-none bg-neutral-900"
      type="text"
      placeholder={placeholderText}
    />
  </div>
</div>
