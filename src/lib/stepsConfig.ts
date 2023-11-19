import Initial from "./steps/Welcome.svelte";
import Desktop from "./steps/Desktop.svelte";
import Keyboard from "./steps/Keyboard.svelte";
import Packages from "./steps/Packages.svelte";
import Partition from "./steps/Partition.svelte";
import Accounts from "./steps/Accounts.svelte";
import Extras from "./steps/Extras.svelte";
import Summary from "./steps/Summary.svelte";

import type { StepConfig } from "./types";

const stepsConfig: StepConfig[] = [
  { view: "initial", component: Initial },
  { view: "keyboard", component: Keyboard },
  { view: "desktop", component: Desktop },
  { view: "packages", component: Packages },
  { view: "partition", component: Partition },
  { view: "accounts", component: Accounts },
  { view: "extras", component: Extras },
  { view: "summary", component: Summary },
];

export default stepsConfig;
