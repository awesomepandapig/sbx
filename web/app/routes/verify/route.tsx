"use client";

import { useState, useEffect } from "react";
import { Loader2, Shield } from "lucide-react";
import { API_URL } from "~/lib/config";
import { useNavigate } from "@remix-run/react";

// TODO: PROTECT PAGE DISALLOW USE FOR USERS WITHOUT VALID SESSIONS

export default function VerifyIgnPage() {
  const navigate = useNavigate();
  const [ign, setIgn] = useState("");
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState<"idle" | "error">("idle");
  const [submitted, setSubmitted] = useState(false);
  const [isComplete, setIsComplete] = useState(false);
  //   const router = useRouter()

  useEffect(() => {
    // Trigger the completion animation after a short delay
    const timer = setTimeout(() => setIsComplete(true), 300);
    return () => clearTimeout(timer);
  }, []);

  const handleVerify = async (event: React.FormEvent) => {
    event.preventDefault();

    if (!ign.trim()) return;

    setLoading(true);
    setStatus("idle");
    setSubmitted(true);

    try {
      const response = await fetch(`${API_URL}/verify-ign/`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ign }),
        credentials: "include",
      });

      if (!response.ok) {
        throw new Error("IGN verification failed");
      }

      const result = await response.json();
      navigate("/dashboard");
    } catch {
      setStatus("error");
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setIgn(e.target.value);
    setSubmitted(false);
    setStatus("idle");
  };

  const renderButtonContent = () => {
    if (loading) {
      return (
        <>
          <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          Verifying...
        </>
      );
    }
    return status === "error" ? "Invalid IGN" : "Verify";
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-4 bg-[#030105]">
      <div className="max-w-md p-6 border border-gray-700 rounded-md bg-gray-900">
        <div className="space-y-1 text-center mb-6">
          <div className="flex justify-center mb-2">
            <Shield className="h-8 w-8 text-blue-500" />
          </div>
          <h2 className="text-3xl font-bold tracking-tight text-white text-center">
            Verify IGN
          </h2>
        </div>
        <p className="text-gray-300 text-sm leading-relaxed mb-4">
          Before you proceed: we must verify that you have connected your
          Minecraft account to your Discord account using the command{" "}
          <code className="bg-gray-800 px-2 py-0.5 rounded text-white font-mono">
            /discord
          </code>{" "}
          on the Hypixel server.
        </p>

        <form onSubmit={handleVerify}>
          <input
            id="ign"
            type="text"
            value={ign}
            onChange={handleInputChange}
            className="w-full px-2 py-2 mb-4 border border-gray-700 text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-gray-800 text-foreground placeholder:text-gray-500"
            placeholder="IGN"
          />
          <button
            type="submit"
            disabled={loading || !ign.trim() || submitted}
            className={`w-full py-2 rounded-md flex items-center justify-center transition-colors ${
              status === "error"
                ? "bg-red-600 text-white"
                : !ign.trim() || submitted
                  ? "bg-gray-600/20 text-gray-500 cursor-not-allowed"
                  : "bg-blue-500 text-white hover:bg-blue-500/80"
            }`}
          >
            {renderButtonContent()}
          </button>
        </form>
      </div>
    </main>
  );
}
