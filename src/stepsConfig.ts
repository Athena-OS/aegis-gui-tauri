import WelcomePage from "./pages/WelcomePage.svelte";
import KeyboardPage from "./pages/KeyboardPage.svelte";
import DesktopPage from "./pages/DesktopPage.svelte";
import PackagesPage from "./pages/PackagesPage.svelte";
import PartitionsPage from "./pages/PartitionPages/1.svelte";
import ConfigurePartitionPage from "./pages/PartitionPages/2.svelte";
import ReplacePartitionPage from "./pages/PartitionPages/5.svelte";
import FinalizePartitionPage from "./pages/PartitionPages/3.svelte";
import ConfigureInstallAlongPage from "./pages/PartitionPages/4.svelte";
import AccountsPage from "./pages/AccountsPage.svelte";
import ExtrasPage from "./pages/ExtrasPage.svelte";
import SummaryPage from "./pages/SummaryPage.svelte";
import InstallPage from "./pages/InstallPage.svelte";
import DonePage from "./pages/DonePage.svelte";
import BaseDitroPage from "./pages/BaseDitroPage.svelte";

export interface StepConfig {
  route: string;
  component: any;
  exclude?: boolean;
}

const stepsConfig: StepConfig[] = [
  { route: "/", component: WelcomePage, exclude: true },
  { route: "/base", component: BaseDitroPage},
  { route: "/keyboard", component: KeyboardPage },
  { route: "/desktop", component: DesktopPage },
  { route: "/packages", component: PackagesPage },
  { route: "/accounts", component: AccountsPage },
  { route: "/extras", component: ExtrasPage },
  { route: "/partition", component: PartitionsPage },
  { route: "/configure-partition", component: ConfigurePartitionPage, exclude: true },
  { route: "/finalize-partition", component: FinalizePartitionPage, exclude: true },
  { route: "/configure-install-along", component: ConfigureInstallAlongPage },
  { route: "/summary", component: SummaryPage },
  { route: "/install", component: InstallPage, exclude: true },
  { route: "/done", component: DonePage, exclude: true },
  {route:"/replace-partition", component: ReplacePartitionPage }
];

export default stepsConfig;
