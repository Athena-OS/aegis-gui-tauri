<script lang="ts">
  import Transition from "svelte-transition";

  export let dialog: any;
  export let dialogTitle = "";
</script>

<div class="relative z-20">
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
            {#if dialogTitle && dialogTitle !== ""}
              <h3 class="text-lg leading-6 text-neutral-200">
                {dialogTitle}
              </h3>
            {/if}
            <div class={dialogTitle ? "mt-2" : ""}>
              <slot />
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</div>
