<script>
  import { createDialog } from "svelte-headlessui";
  import Transition from "svelte-transition";
  import currentActive from "../stepsStore";

  export let title = "";
  export let dialogTitle = "";
  export let dialogContent = "";

  const dialog = createDialog({ label: title });
</script>

<!-- dialog start -->
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
        class="fixed inset-0 bg-black bg-opacity-25 border-none p-0 m-0 focus:ring-0 focus:outline-none"
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
            class="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all"
            use:dialog.modal
          >
            <h3 class="text-lg font-medium leading-6 text-gray-900">
              {dialogTitle}
            </h3>
            <div class="mt-2">
              <p class="text-sm text-gray-500">
                {dialogContent}
              </p>
            </div>

            <div class="mt-4">
              <button
                type="button"
                class="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                on:click={dialog.close}
              >
                Got it, thanks!
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</div>
<!-- dialog end -->

<main
  class="h-full p-4 space-y-4 absolute top-0 left-0 right-0 overflow-scroll"
>
  <div class="flex flex-row items-center mx-auto w-fit space-x-4">
    <h2 class="text-center py-2">{$currentActive}. {title}</h2>
    <button
      on:click={dialog.open}
      aria-label="Open Dialog"
      class="bg-transparent border-none focus:outline-none"
    >
      <i class="ti text-accent-500 !text-3xl ti-info-hexagon-filled" />
    </button>
  </div>
  <div class="h-[calc(100%-140px)]">
    <slot />
  </div>
</main>
