<script lang="ts">
    export let steps: number;
    export let currentStep: number = 1;
</script>

<div class="footer">
  <div class="left-description">
      <slot name="description"></slot>
  </div>
  <div class="center-stepper">
      {#each Array(steps) as _, index (index)}
          <div class="nob {index + 1 === currentStep ? 'active' : ''}"></div>
      {/each}
  </div>
  <div class="right-controls">
      <slot name="controls"></slot>
  </div>
</div>


<style lang="scss">
 .footer {
    display: flex;
    position: fixed;
    bottom: 10px;
    width: 100%;
    justify-content: space-between;
    align-items: center;
    padding: 0 2%;
    z-index: 1000;
    box-sizing: border-box;
}

.left-description {
    text-align: left;
    flex: 1;
    white-space: nowrap;
}

.center-stepper {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 5px;
}

.right-controls {
    text-align: right;
    flex: 1;
    white-space: nowrap;

    button {
        margin-left: 5px;
    }
}

.nob {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #ddd;
}

.nob.active {
    background: #007bff;
}


</style>
