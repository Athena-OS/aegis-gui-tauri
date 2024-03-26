<script lang="ts">
  import userIcon from "../assets/icons/user-yellow.svg";
  import plusIcon from "../assets/icons/plus-yellow-icon.svg";
  import editGrayIcon from "../assets/icons/edit-gray.svg";
  import binGrayIcon from "../assets/icons/bin-gray.svg";

  import { createDialog } from "svelte-headlessui";
  import { event } from "@tauri-apps/api";

  import accountsStore from "../lib/stores/accountsStore";
  import partitionStore from "../lib/stores/partitionStore";
  import StepWrapper from "../lib/components/StepWrapper.svelte";
  import CreateUserDialog from "../lib/components/CreateUser/CreateUserDialog.svelte";
  import InputBox from "../lib/components/InputBox.svelte";
  import Button from "../lib/components/Button.svelte";


  let dialog = createDialog({ label: "dialogTitle" });
</script>

<CreateUserDialog {dialog} />

<StepWrapper
  title="Accounts"
  dialogTitle="Accounts page"
  dialogContent="In this page, you create accounts for users. One of the accounts must have root access."
  prev="/packages"
  next={$accountsStore.users.filter((item) => item.hasRoot === true).length > 0
    ? "/extras"
    : ""}
>
  <div class="flex w-full h-full justify-center items-center">
    <div
      class="flex flex-col item-center bg-[#1A1A1A] w-1/2 h-full rounded-2xl"
    >
      <div
        class="flex flex-row rounded-t-2xl bg-neutral-800 items-center justify-between p-4"
      >
        <div class="flex flex-row items-center gap-2">
          <img src={userIcon} alt="user" />
          <h3 class="font-semibold text-2xl">Accounts</h3>
        </div>
        <div class="flex justify-center items-center space-x-2">
          {#if $accountsStore.users.filter((item) => item.hasRoot === true).length <= 0}
            <h4 class="text-lg text-neutral-300 font-bold">
              Create atleast one root user
            </h4>
          {/if}
          <button
            class="text-4xl font-medium bg-neutral-800 text-yellow-500 h-10 pb-1 px-2 rounded-lg flex justify-center items-center"
            on:click={dialog.open}><p>+</p></button
          >
        </div>
      </div>
      <h4 class="text-center text-[#B0B0B0] font-medium">
        {#each $accountsStore.users as user, index (index)}
          <div class="px-4 py-4 bg-[#5B5B5B]/40 flex space-x-4 text-left">
            <div
              class="h-12 w-12 aspect-square rounded-full bg-white text-xl text-black flex justify-center items-center"
            >
              {user.name.charAt(0).toUpperCase()}
            </div>
            <div class="flex justify-between items-center w-full text-lg">
              <div>
                <div>
                  {user.userName} ( {user.name} ) {#if user.hasRoot}<span>
                      - <span class="text-primary-500 font-bold">ROOT</span
                      ></span
                    >{/if}
                </div>
              </div>
              <div class="flex items-center">
                <button
                  on:click={async () => {
                    
                    $accountsStore.createNewUserTemp = {
                      name: user.name,
                      userName: user.userName,
                      password: user.password,
                      confirmPassword: user.password,
                      passwordSameAsRoot: false,
                      hasRoot: user.hasRoot,
                      isEditing: true
                    };
                    dialog.open();
                  }}
                  class="mr-2"
                >
                  <img src={editGrayIcon} alt="edit" />
                </button>
                <button
                  on:click={() => {
                    $accountsStore.users = $accountsStore.users.filter(
                      (item) => item !== user,
                    );
                  }}
                >
                  <img src={binGrayIcon} alt="delete" />
                </button>
              </div>
            </div>
          </div>
        {/each}
      </h4>
    </div>
  </div>
</StepWrapper>

<style>
</style>
