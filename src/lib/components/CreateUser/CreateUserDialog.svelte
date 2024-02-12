<script lang="ts">
  import Transition from "svelte-transition";
  
  import Button from "../Button.svelte";
  import Intro from "./Intro.svelte";
  import Password from "./Password.svelte";
  import Misc from "./Misc.svelte";
  import crossIcon from "../../../assets/icons/cross.svg";
  import accountsStore from "../../stores/accountsStore";

  export let dialog: any;

  let steps = [
    {
      label: "User Configuration",
      component: Intro,
    },
    {
      label: "Password Configuration",
      component: Password,
    },
    {
      label: "Extras",
      component: Misc,
    },
  ];

  let currentStep = 0;

  const handlePrevious = () => {
    if (currentStep > 0) currentStep--;
  };

  const handleNext = () => {
    if (currentStep < steps.length - 1) {
      if (currentStep === 0) {
        if (
          $accountsStore.createNewUserTemp.name !== "" &&
          $accountsStore.createNewUserTemp.userName !== "" &&
          $accountsStore.users.filter(
            (item) =>
              item.userName === $accountsStore.createNewUserTemp.userName,
          ).length <= 1
        ) {
          currentStep++;
        }
      } else if (currentStep === 1) {
        if ($accountsStore.createNewUserTemp.passwordSameAsRoot === false) {
          if (
            $accountsStore.createNewUserTemp.password !== "" &&
            $accountsStore.createNewUserTemp.confirmPassword !== "" &&
            $accountsStore.createNewUserTemp.password ===
              $accountsStore.createNewUserTemp.confirmPassword
          ) {
            currentStep++;
          }
        } else {
          currentStep++;
        }
      }
    } else {
      if (currentStep === 2) {
        if ($accountsStore.createNewUserTemp.passwordSameAsRoot) {
          if ($accountsStore.createNewUserTemp.isEditing) {
            $accountsStore.users[
              $accountsStore.users.indexOf(
                $accountsStore.users.filter(
                  (item) =>
                    item.userName === $accountsStore.createNewUserTemp.userName,
                )[0],
              )
            ] = {
              name: $accountsStore.createNewUserTemp.name,
              userName: $accountsStore.createNewUserTemp.userName,
              password: $accountsStore.users.filter(
                (item) => item.hasRoot === true,
              )[0].password,
              hasRoot: $accountsStore.createNewUserTemp.hasRoot,
            };
          } else {
            $accountsStore.users.push({
              name: $accountsStore.createNewUserTemp.name,
              userName: $accountsStore.createNewUserTemp.userName,
              password: $accountsStore.users.filter(
                (item) => item.hasRoot === true,
              )[0].password,
              hasRoot: $accountsStore.createNewUserTemp.hasRoot,
            });
            
          }
        } else {
          if ($accountsStore.createNewUserTemp.isEditing) {
            $accountsStore.users[
              $accountsStore.users.indexOf(
                $accountsStore.users.filter(
                  (item) =>
                    item.userName === $accountsStore.createNewUserTemp.userName,
                )[0],
              )
            ] = {
              name: $accountsStore.createNewUserTemp.name,
              userName: $accountsStore.createNewUserTemp.userName,
              password: $accountsStore.createNewUserTemp.password,
              hasRoot: $accountsStore.createNewUserTemp.hasRoot,
            };
          } else {
            $accountsStore.users.push({
              name: $accountsStore.createNewUserTemp.name,
              userName: $accountsStore.createNewUserTemp.userName,
              password: $accountsStore.createNewUserTemp.password,
              hasRoot: $accountsStore.createNewUserTemp.hasRoot,
            });
          }
        }
        currentStep = 0;
        $accountsStore.createNewUserTemp = {
          name: "",
          userName: "",
          password: "",
          confirmPassword: "",
          passwordSameAsRoot: false,
          hasRoot: false,
          isEditing: false,
        };
        dialog.close();
      }
    }
  };
</script>

<div class="relative z-10">
  <Transition show={$dialog.expanded}>
    <Transition
      enter="ease-out duration-300"
      enterFrom="opacity-0"
      enterTo="opacity-100"
      leave="ease-in duration-200"
      leaveFrom="opacity-100"
      leaveTo="opacity-0"
    >
      <button
        on:click={dialog.close}
        aria-label="Close Dialog"
        class="fixed inset-0 bg-black backdrop-blur-xl bg-opacity-10 border-none p-0 m-0 focus:ring-0 focus:outline-none"
      />
    </Transition>
    <div class="fixed inset-0 overflow-y-auto">
      <div class="flex min-h-full items-center justify-center p-4 text-center">
        <Transition
          enter="ease-out duration-300"
          enterFrom="opacity-0 scale-95"
          enterTo="opacity-100 scale-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100 scale-100"
          leaveTo="opacity-0 scale-95"
        >
          <div
            class="w-full max-w-lg border border-neutral-700 transform overflow-hidden rounded-2xl bg-gray-800 px-4 py-3 text-left align-middle shadow-xl transition-all"
            use:dialog.modal
          >
            <div class="w-full justify-between pt-2 flex items-center">
              <h3 class="text-xl leading-6 font-medium text-neutral-400">
                Create a new user
              </h3>
              <button on:click={dialog.close}
                ><img src={crossIcon} alt="" /></button
              >
            </div>

            <div class="mt-2 transition-height ease-out p-6">
              {#each steps as step, index}
                {#if index === currentStep}
                  <svelte:component this={step.component} />
                {/if}
              {/each}
              <div class="flex justify-between items-center pt-8 space-x-4">
                <Button
                  disabled={currentStep === 0}
                  variant="bordered"
                  fullWidth
                  on:click={handlePrevious}>Previous</Button
                ><Button fullWidth on:click={handleNext}
                  >{currentStep < steps.length - 1 ? "Next" : "Done"}</Button
                >
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</div>
