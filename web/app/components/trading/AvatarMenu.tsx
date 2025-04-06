import { useState, useRef, useEffect } from "react";
import { signOut } from "~/lib/auth";
import { Settings, LogOut } from "lucide-react";

interface AvatarProps {
  userImg: string;
}

export default function AvatarMenu({ userImg }: AvatarProps) {
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Close menu when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <div className="relative" ref={menuRef}>
      {/* Menu trigger */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="h-10 w-10 rounded-full p-0 overflow-hidden bg-[#141414] flex items-center justify-center hover:opacity-90 transition-opacity"
      >
        <img src={userImg} alt="User avatar" className="rounded-full w-8 h-8" />
      </button>

      {/* Menu content */}
      {isOpen && (
        <div className="absolute right-0 mt-2 w-64 text-[13px] rounded-xl bg-[#141414] shadow-lg border border-gray-800 overflow-hidden z-10 p-2">
          {/* Menu items */}

          <button className="w-full flex items-center px-4 py-3 text-white hover:bg-gray-800/80 transition-colors rounded-md">
            <Settings className="h-4 w-4 mr-3 text-white" />
            <span>Settings</span>
          </button>

          <div className="h-px bg-gray-700 my-1"></div>

          <button
            className="w-full flex items-center px-4 py-3 text-white hover:bg-gray-800/80 transition-colors rounded-md"
            onClick={signOut}
          >
            <LogOut className="h-4 w-4 mr-3 text-white" />
            <span>Log out</span>
          </button>
        </div>
      )}
    </div>
  );
}
