<script>
  import InputBox from "../InputBox.svelte";
  import accountsStore from "../../stores/accountsStore";
</script>

<div class="space-y-4">
  <InputBox
    bind:value={$accountsStore.createNewUserTemp.name}
    label="Full Name"
    placeholderText="Enter your Full Name"
  />
  {#if $accountsStore.createNewUserTemp.isEditing}
    <InputBox
      isDisabled={true}
      value={$accountsStore.createNewUserTemp.username}
      label="Username"
      placeholderText="Create a Username"
    />
  {:else}
    <InputBox
      bind:value={$accountsStore.createNewUserTemp.username}
      label="Username"
      styleClass="lowercase"
      placeholderText="Create a Username"
    />
  {/if}
</div>
