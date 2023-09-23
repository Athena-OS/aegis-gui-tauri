<script lang="ts">
    import athenaLogo from "../assets/athena-logo.svg";
    export let switchView: (viewName: string) => void;
</script>

<img src={athenaLogo} alt="athena-logo">
<h1>Welcome to Athena-OS</h1>
<button on:click={() => switchView("desktop")}>Click here to start installation.</button>

<style lang="scss">
    img {
        width: 8rem;
    }
</style>