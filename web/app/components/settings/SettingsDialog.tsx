import { useState, useEffect } from "react";
import { useLocation, useNavigate } from "@remix-run/react";
import { Button } from "~/components/ui/button";
import {
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";

import SettingsAccountContent from "./SettingsAccountContent";
import SettingsApiContent from "./SettingsApiContent";
import { KeyRoundIcon, User, User2 } from "lucide-react";

type SettingsView = "account" | "api"; // Add more keys for future sections

export default function SettingsDialog() {
  const location = useLocation();
  const navigate = useNavigate();
  const [activeView, setActiveView] = useState<SettingsView>("account");

  // Function to parse the view name from the hash
  const getViewFromHash = (hash: string): SettingsView => {
    if (hash.startsWith("#settings/api")) {
      return "api";
    }
    if (hash.startsWith("#settings")) {
      return "account";
    }
    return "account";
  };

  useEffect(() => {
    const currentView = getViewFromHash(location.hash);
    if (currentView !== activeView) {
      setActiveView(currentView);
    }
  }, [location.hash, activeView]);

  // Handler for clicking sidebar navigation buttons
  const handleNavClick = (view: SettingsView) => {
    const newHash = `#settings/${view}`;
    // Only navigate if the hash is actually different
    if (location.hash !== newHash) {
      navigate(`${location.pathname}${location.search}${newHash}`, {
        preventScrollReset: true, // Keep scroll position
        replace: true, // Replace history for intra-modal navigation
      });
    }
  };

  // This component renders the DialogContent
  return (
    <DialogContent className="gap-y-0 border-gray-800 border rounded-xl bg-[#030105] text-white p-0 max-w-[680px]">
      <DialogHeader className="p-6 border-b border-gray-800">
        <DialogTitle className="text font-semibold">Settings</DialogTitle>
      </DialogHeader>
      <div className="flex h-[500px] max-w-[680px]">
        
        {/* Sidebar */}
        <nav className="w-[180px] border-r border-gray-800 p-4 flex flex-col h-full">
          {/* Sidebar buttons remain the same, controlling the activeView state via hash */}
          <Button
            variant={activeView === "account" ? "secondary" : "ghost"}
            className="w-full justify-start"
            onClick={() => handleNavClick("account")}
          >
            <User2/>
            Account
          </Button>
          <Button
            variant={activeView === "api" ? "secondary" : "ghost"}
            className="w-full justify-start"
            onClick={() => handleNavClick("api")}
          >
            <KeyRoundIcon/>
            API
          </Button>
          {/* Add more settings links here */}
        </nav>

        {/* Content Area - Render ALL views, control visibility with CSS */}
        <div className="flex-1 p-8 w-[500px]">
          {" "}
          {/* `overflow-auto` handles scrolling within the content area */}
          {/* Account View Wrapper - Use Tailwind's hidden/block or similar */}
          {/* The `key` prop might not be strictly necessary here but can help React */}
          {/* differentiate if structure changes significantly based on view later. */}
          <div
            key="account-view"
            className={activeView === "account" ? "block" : "hidden"}
          >
            <SettingsAccountContent />
          </div>
          {/* API View Wrapper */}
          <div
            key="api-view"
            className={activeView === "api" ? "block" : "hidden"}
          >
            <SettingsApiContent />
          </div>
          {/* Add wrappers for other future views here, e.g.: */}
          {/* <div key="preferences-view" className={activeView === 'preferences' ? 'block' : 'hidden'}> */}
          {/* <SettingsPreferencesContent /> */}
          {/* </div> */}
        </div>
      </div>
    </DialogContent>
  );
}
