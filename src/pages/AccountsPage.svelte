<script lang="ts">
  import userIcon from "../assets/icons/user-yellow.svg";
  import plusIcon from "../assets/icons/plus-yellow-icon.svg";
  import { createDialog } from "svelte-headlessui";
  import StepWrapper from "../lib/components/StepWrapper.svelte";
  import CreateUserDialog from "../lib/components/CreateUser/CreateUserDialog.svelte";

  import editGrayIcon from "../assets/icons/edit-gray.svg";
  import binGrayIcon from "../assets/icons/bin-gray.svg";

  let dialog = createDialog({ label: "dialogTitle" });
</script>

<CreateUserDialog {dialog} />

<StepWrapper
  title="Accounts"
  dialogTitle="Header Here"
  dialogContent="Your text here"
  prev="/finalize-partition"
  next="/extras"
>
  <div class="flex flex-col items-center">
    <div
      class="flex flex-col item-center bg-[#1A1A1A] w-[32em] h-[22em] rounded-2xl"
    >
      <div class="flex flex-row items-center justify-between p-4">
        <div class="flex flex-row items-center gap-2">
          <img src={userIcon} alt="user" />
          <h3 class="font-semibold">Accounts</h3>
        </div>
        <button on:click={dialog.open}>
          <img src={plusIcon} alt="plus" />
        </button>
      </div>
      <h4 class="text-center text-[#B0B0B0] font-medium">
        <div class="px-4 py-2 bg-[#5B5B5B]/40 flex space-x-4 text-left">
          <div
            class="h-12 w-12 aspect-square rounded-full bg-white text-sm text-black flex justify-center items-center"
          >
            Z
          </div>
          <div class="flex justify-between w-full">
            <div>
              <div>zekki</div>
              <div class="text-primary-500">root</div>
            </div>
            <div class="flex items-center">
              <button class="mr-2">
                <img src={editGrayIcon} alt="edit" />
              </button>
              <button class="">
                <img src={binGrayIcon} alt="delete" />
              </button>
            </div>
          </div>
        </div>
      </h4>
    </div>
  </div>
</StepWrapper>
