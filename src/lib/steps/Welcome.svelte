<script lang="ts">
    import athenaLogo from "../../assets/athena-logo.svg";
    import infoIndicator from "../../assets/icons/info-indicator-blue.svg";
    export let switchView: (viewName: string) => void;
</script>

<div class="flex flex-col items-center gap-10 mt-36">
    <h1>Welcome to AthenaOS</h1>
    <div class="relative circle-surround">
        <img src={athenaLogo} alt="athena-logo" class="w-32 border-8 border-solid rounded-full border-yellow-500" />
    </div>
    
    <button class="primary-btn" on:click={() => switchView("keyboard")}>
        Start installation
    </button>
</div>

<div class="flex flex-row gap-2 items-center absolute bottom-0 left-0 bg-[#1A1A1A] w-full p-1 font-medium text-sm">
    <img src={infoIndicator} alt="info indicator" class="w-3">
    <p>We'll guide you through a few steps to install AthenaOS on your system.</p>
</div>

<style lang="scss">
        .circle-surround {
            &::before {
                content: "";
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                width: 80rem;
                height: 80rem;
                border-radius: 50%;
                z-index: -1;
                background: url("../../assets/welcome-radius.svg") no-repeat center center;
                background-size: cover;
            }
        }
</style>
