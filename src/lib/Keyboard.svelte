<script lang="ts">
    import keyboard from "../assets/keyboard.svg";
    import Footer from "./ProgressStepper.svelte";
    export let switchView: (viewName: string) => void;
</script>

<div class="col-container">
    <h2>Select keyboard layout</h2>
    <img src={keyboard} alt="athena-logo" />
    <div class="layout-choice">
        <div class="layout-lang">
            <ul>
                <li>English (US)</li>
                <li>French</li>
                <li>German</li>
                <li>English (UK)</li>
                <li>Spanish</li>
                <li>Finnish</li>
                <li>Finnish</li>
            </ul>
        </div>
        <div class="layout-model">
            <div class="keyboard-model">

            </div>
            <div class="keyboard-layout">

            </div>
        </div>
    </div>
    
</div>
<Footer steps={5} currentStep={3}>
    <span slot="description">This is step 3 out of 5</span>
    <div slot="controls">
        <button>Back</button>
        <button>Continue</button>
    </div>
</Footer>


<style lang="scss">
    .col-container {
        align-items: center;

        h2 {
            margin-bottom: 0.6rem;
        }

        img {
            width: 30em;
        }

        .layout-choice {
            margin-top: 1rem;
            display: flex;
            flex-direction: row;

            .layout-lang {
                background-color: #232433;
                width: 25em;
                height: 10em;
                border-radius: 1rem;
                display: flex; 
                justify-content: center;
                align-items: center;
                padding: 0.3rem 0rem;

                ul {
                    font-size: 0.95em;
                    font-weight: 500;
                    width: 23em;
                    margin: 0;
                    padding: 0em 1em;
                    list-style-type: none;
                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                    max-height: 8em;
                    overflow-y: scroll;
                    overflow-x: hidden;
                    gap: 0.4rem;

                    li {
                        cursor: pointer;
                        text-align: left;
                        width: 21em;
                        padding: 0.2rem 0.6rem;
                        border-radius: 0.4rem;
                    
                        &:hover {
                            background-color: rgb(108, 107, 112, 0.71);
                        }
                    }
                }
            }
        }



    }
</style>
