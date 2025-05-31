import { useState, useEffect } from "react";
import { signIn } from "~/lib/auth";
import { AUTH_URL, DOMAIN } from "~/lib/config";
import { useNavigate } from "@remix-run/react";

export default function GetStartedButton() {
  const navigate = useNavigate();
  const [session, setSession] = useState<null | any>(null);

  useEffect(() => {
    async function checkSession() {
      try {
        const response = await fetch(`${AUTH_URL}/auth/get-session`, {
          credentials: "include",
        });
        if (!response.ok) throw new Error("Request failed");
        const data = await response.json();
        setSession(data);
      } catch {
        setSession(null);
      }
    }

    checkSession();
  }, []);

  return (
    <button className="bg-[#3CFFFF] text-black px-6 py-2 rounded-full text-sm font-medium"
      onClick={() => {
        if (session) {
          navigate(`/trade/JSP`);
        } else {
          signIn(`${DOMAIN}/trade/JSP`);
        }
      }}
    >
          GET STARTED
    </button>
  );
}
