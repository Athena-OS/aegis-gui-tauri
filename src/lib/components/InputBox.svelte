<script>
  import uuid from "../utils/uuid";

  export let label = "Test Keyboard"; // default label text
  export let placeholderText = "Type to test.."; // default input placeholder text

  let inputID = uuid(); // just to generate a unique id for label
</script>

<div class="w-full space-y-3">
  <label class="font-medium text-zinc-400" for={inputID}>
    {label}
  </label>
  <div class="w-full h-[50px]">
    <input
      id={inputID}
      class="w-full h-full rounded-full overflow-hidden ring ring-zinc-800 px-4 text-lg focus:ring-yellow-500 outline-none bg-zinc-900"
      type="text"
      placeholder={placeholderText}
    />
  </div>
</div>
