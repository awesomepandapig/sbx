import { useState, useEffect } from "react";
import { useLocation, useNavigate } from "@remix-run/react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";
import { Dialog } from "~/components/ui/dialog";
import { LogOut, Settings } from "lucide-react";
import { signOut } from "~/lib/auth";
import SettingsDialog from "~/components/settings/SettingsDialog";

interface AvatarProps {
  userImg: string;
}

export default function AvatarMenu({ userImg }: AvatarProps) {
  const [menuOpen, setMenuOpen] = useState(false);
  const [dialogOpen, setDialogOpen] = useState(false);

  const location = useLocation();
  const navigate = useNavigate();

  // Effect to check hash on mount and potentially open dialog
  useEffect(() => {
    if (location.hash.startsWith("#settings/") && !dialogOpen) {
      setDialogOpen(true);
    }
    if (!location.hash.startsWith("#settings/") && dialogOpen) {
      setDialogOpen(false);
    }
  }, [location.hash, dialogOpen]);

  const handleSettingsClick = () => {
    // Set hash to default view and ensure dialog is open
    navigate(`${location.pathname}${location.search}#settings/account`, {
      preventScrollReset: true,
      replace: true,
    });
    setDialogOpen(true); // Ensure dialog opens
    setMenuOpen(false); // Close the dropdown menu
  };

  const handleOpenChange = (open: boolean) => {
    setDialogOpen(open);
    if (!open && location.hash.startsWith("#settings")) {
      navigate(`${location.pathname}${location.search}`, {
        preventScrollReset: true,
        replace: true,
      });
    }
  };

  return (
    <Dialog open={dialogOpen} onOpenChange={handleOpenChange}>
      <DropdownMenu open={menuOpen} onOpenChange={setMenuOpen}>
        <DropdownMenuTrigger asChild>
          <button className="h-10 w-10 rounded-full overflow-hidden bg-[#141414] flex items-center justify-center hover:opacity-90 transition-opacity">
            <img
              src={userImg}
              alt="User avatar"
              className="rounded-full w-8 h-8"
            />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-64 text-[13px] rounded-xl bg-[#141414] border border-gray-800 shadow-lg p-2">
          <DropdownMenuItem
            className="flex items-center h-11 text-white hover:bg-gray-800/80 transition-colors rounded-md cursor-pointer"
            onSelect={(e) => {
              e.preventDefault();
              handleSettingsClick();
            }}
          >
            <Settings className="h-4 w-4 mr-3 text-white" />
            Settings
          </DropdownMenuItem>

          <div className="h-px bg-gray-700 my-1" />

          <DropdownMenuItem
            className="flex items-center h-11 text-white hover:bg-gray-800/80 transition-colors rounded-md cursor-pointer"
            onSelect={(e) => {
              e.preventDefault();
              signOut();
            }}
          >
            <LogOut className="h-4 w-4 mr-3 text-white" />
            Log out
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {dialogOpen && <SettingsDialog />}
    </Dialog>
  );
}
