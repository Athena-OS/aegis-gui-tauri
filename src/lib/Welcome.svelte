<script lang="ts">
    import athenaLogo from "../assets/athena-logo.svg";
    import infoIndicator from "../assets/info-indicator-blue.svg";
    export let switchView: (viewName: string) => void;
</script>

<div class="flex flex-col items-center">
    <h1 class="text-5xl font-medium text-white">Welcome to AthenaOS</h1>
    <div class="circle-surround">
        <img src={athenaLogo} alt="athena-logo" class="w-32" />
    </div>
    
    <button on:click={() => switchView("keyboard")}>
        Start installation
    </button>
</div>

<div class="flex flex-row items-center absolute bottom-0 left-0 bg-white w-full">
    <img src={infoIndicator} alt="info indicator">
    <p>We'll guide you through a few steps to install AthenaOS on your system.</p>
</div>

<!-- 
<style lang="scss">
    .col-container {
        margin-top: 3.5rem;
        align-items: center;

        h1 {
            margin-bottom: 3rem;
        }
        
        .circle-surround {
            position: relative;
            margin-bottom: 2.5rem;
            
            &::before {
                content: "";
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                width: 45rem;
                height: 45rem;
                border-radius: 50%;
                z-index: -1;
                background: url("../assets/welcome-radius.svg") no-repeat center center;
                background-size: cover;
            }

            img {
                width: 8rem; 
                display: block;
                border: 6px solid #FFB800;
                border-radius: 50%;
            }
        }
    }
    .info-footer {
        display: flex;
        flex-direction: row;
        position: absolute;
        align-items: center;
        bottom:0;
        left:0;
        width: 100%;
        background:black;
        height: 2em;
        padding: 0.2rem 0.5rem;
        gap:0.5rem;

        img {
            width: 1em;
        }
    }
</style>
-->