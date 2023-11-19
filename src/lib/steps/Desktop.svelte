<script lang="ts">
  import gnomeImage from "../../assets/gnome.svg";
  import StepWrapper from "../components/StepWrapper.svelte";
</script>

<StepWrapper
  title="Select Environment"
  dialogTitle="Header Here"
  dialogContent="Your text here"
>
  <div class="flex h-full flex-col items-left">
    <div class="flex flex-row h-full w-full gap-4">
      <div class="h-full bg-zinc-900 w-56 rounded-3xl border border-zinc-800" />
      <div class="flex flex-col h-full flex-1 gap-4">
        <div
          class="bg-zinc-900 w-full h-32 rounded-3xl border border-zinc-800"
        />
        <div class="w-full flex-1 relative">
          <div class="absolute inset-0">
            <img
              src={gnomeImage}
              alt="gnome"
              class="w-full h-full object-cover rounded-3xl"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</StepWrapper>
