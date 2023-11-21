<script lang="ts">
  import userIcon from "../assets/icons/user-yellow.svg";
  import plusIcon from "../assets/icons/plus-yellow-icon.svg";
  import { createDialog } from "svelte-headlessui";
  import StepWrapper from "../lib/components/StepWrapper.svelte";
  import CreateUserDialog from "../lib/components/CreateUser/CreateUserDialog.svelte";

  let dialog = createDialog({ label: "dialogTitle" });
</script>

<CreateUserDialog {dialog} />

<StepWrapper
  title="Accounts"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/packages"
  next="/extras"
>
  <div class="flex flex-col items-center">
    <div
      class="flex flex-col item-center bg-[#1A1A1A] w-[32em] h-[22em] rounded-2xl p-4"
    >
      <div class="flex flex-row items-center justify-between">
        <div class="flex flex-row items-center gap-2">
          <img src={userIcon} alt="user" />
          <h3 class="font-semibold">Accounts</h3>
        </div>
        <button on:click={dialog.open}>
          <img src={plusIcon} alt="plus" />
        </button>
      </div>
      <h4
        class="text-center mx-auto my-auto text-4xl max-w-md text-[#B0B0B0] font-medium"
      >
        Create Users to Continue...
      </h4>
    </div>
  </div>
</StepWrapper>
