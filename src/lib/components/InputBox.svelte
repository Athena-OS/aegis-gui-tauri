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
      class="w-full h-full placeholder:text-neutral-400 rounded-full overflow-hidden border-2 border-inset border-gray-700 px-4 focus:border-primary-500 outline-none bg-gray-800"
      type="text"
      placeholder={placeholderText}
    />
  </div>
</div>
