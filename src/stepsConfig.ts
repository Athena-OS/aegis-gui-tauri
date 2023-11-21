import WelcomePage from "./pages/WelcomePage.svelte";
import KeyboardPage from "./pages/KeyboardPage.svelte";
import DesktopPage from "./pages/DesktopPage.svelte";
import PackagesPage from "./pages/PackagesPage.svelte";
import PartitionsPage from "./pages/PartitionPages/1.svelte";
import AccountsPage from "./pages/AccountsPage.svelte";
import ExtrasPage from "./pages/ExtrasPage.svelte";
import SummaryPage from "./pages/SummaryPage.svelte";
import InstallPage from "./pages/InstallPage.svelte";


export interface StepConfig {
  route: string;
  component: any;
}

const stepsConfig: StepConfig[] = [
  { route: "/", component: WelcomePage },
  { route: "/keyboard", component: KeyboardPage },
  { route: "/desktop", component: DesktopPage },
  { route: "/packages", component: PackagesPage },
  { route: "/partition", component: PartitionsPage },
  { route: "/accounts", component: AccountsPage },
  { route: "/extras", component: ExtrasPage },
  { route: "/summary", component: SummaryPage },
  { route: "/install", component: InstallPage },
];

export default stepsConfig;
