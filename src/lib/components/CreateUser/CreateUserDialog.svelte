<script lang="ts">
  import Transition from "svelte-transition";
  import Button from "../Button.svelte";
  import Intro from "./Intro.svelte";
  import Password from "./Password.svelte";
  import Misc from "./Misc.svelte";

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
      currentStep++;
    } else {
      dialog.close();
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
            <h3 class="text-xl leading-6 text-neutral-400">
              Create a new user
            </h3>
            <div class="mt-2 max-h-[400px] transition-height ease-out">
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
