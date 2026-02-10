import Pill from "@/components/pill/Pill";
import Settings from "@/components/settings/Settings";
import Onboarding from "@/components/onboarding/Onboarding";

function App() {
  const path = window.location.pathname;

  if (path === "/pill") return <Pill />;
  if (path === "/settings") return <Settings />;
  if (path === "/onboarding") return <Onboarding />;

  // Main window (hidden by default, used for routing/background logic)
  return null;
}

export default App;
