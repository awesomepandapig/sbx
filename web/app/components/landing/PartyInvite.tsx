import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";

// Array of possible words to generate random Minecraft usernames
const usernamePool = [
  "DragonMaster",
  "PixelWarrior",
  "CraftKing",
  "BlockLord",
  "NetherExplorer",
  "RedstoneGenius",
  "EnderHunter",
  "BuildWizard",
  "SkyBlocker",
  "MiningLegend",
  "CreeperKiller",
  "CraftHero",
  "ShadowKnight",
  "FireDragon",
  "IronFist",
  "LavaWalker",
  "StormBringer",
  "NightProwler",
  "FrostMage",
  "SilverHunter",
  "CrystalCrafter",
  "MysticSorcerer",
  "StoneWarden",
  "StormChaser",
  "SunsetWarrior",
  "BlazeWizard",
  "VoidBender",
  "WindSlinger",
  "FlameStrike",
  "DarkKnight",
  "PixelMage",
  "SoulReaver",
  "LightningBolt",
  "EarthShaper",
  "CursedKnight",
  "RuneMaster",
  "FireMage",
  "IronKnight",
  "ThunderBreaker",
  "GlowingWarrior",
  "AbyssalSeeker",
  "NightShade",
  "WindRider",
  "Shatterfury",
  "ObsidianHunter",
  "SteelKnight",
  "LunarSorcerer",
  "IceFury",
  "StormBreaker",
  "TempestWizard",
  "SilverFury",
  "MoonShifter",
  "ShadowWarlock",
  "ArcaneGuardian",
  "TitanSorcerer",
  "EmberKnight",
  "ChaosMage",
  "MysticKnight",
  "VortexWarrior",
  "DarkSoulHunter",
  "CelestialWanderer",
  "StormWizard",
  "ThunderKnight",
  "AshenSwordsman",
  "CelestialHunter",
  "MoonMage",
  "RunicGuardian",
  "ThunderWarrior",
  "DoomsdayKnight",
  "SunShifter",
  "ShadowReaper",
  "SearingMage",
  "IceMage",
  "AncientWarrior",
  "StoneMage",
  "RagingKnight",
  "SolarMage",
  "FrostKnight",
  "MysticRanger",
  "BlazingWarrior",
  "StoneSorcerer",
  "VoidMage",
  "ArcaneShifter",
  "RuneKnight",
  "DarkShifter",
  "FireReaver",
  "LightMage",
  "MoonWarrior",
  "StarlightSorcerer",
  "ChaosWarrior",
  "WindSwordsman",
  "CrimsonMage",
  "StormWalker",
  "BloodMage",
  "AetherKnight",
  "EmberSorcerer",
  "GlacierMage",
  "LunarWarrior",
  "ShadowSwordsman",
  "PhoenixMage",
  "EarthMage",
  "VoidWarrior",
  "BlazingKnight",
  "RuneWarlock",
  "FlameGuardian",
  "StormShifter",
  "ArcaneWarrior",
  "ThunderSorcerer",
  "ShadowRanger",
  "CrystalKnight",
  "DawnMage",
  "SolarWarrior",
  "SunMage",
  "FireGuardian",
  "SpectralKnight",
  "BlazeWarrior",
];

const generateRandomSuffix = () => {
  const characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";
  return Array.from({ length: 4 }, () =>
    characters.charAt(Math.floor(Math.random() * characters.length)),
  ).join("");
};

const getRandomUsername = () => {
  const baseUsername =
    usernamePool[Math.floor(Math.random() * usernamePool.length)];
  return `${baseUsername}_${generateRandomSuffix()}`;
};

export default function PartyInvite() {
  const [username, setUsername] = useState(getRandomUsername());
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);

    // Set up an interval to change username every 2 seconds
    const intervalId = setInterval(() => {
      setUsername(getRandomUsername());
    }, 4000);

    // Clean up the interval when the component unmounts
    return () => clearInterval(intervalId);
  }, []);

  if (!mounted) return null; // Prevent rendering until mounted

  return (
    <div
      className="border border-green-500/30 text-green-400 px-4 py-2 rounded-lg
      bg-green-500/10
      hover:bg-green-500/20
      transition-all
      shadow-[0_0_15px_rgba(34,197,94,0.4)]
      hover:shadow-[0_0_25px_rgba(34,197,94,0.6)]
      flex items-center justify-center"
    >
      /party invite{""}
      <AnimatePresence mode="wait">
        <motion.span
          key={username}
          initial={{ opacity: 0, rotateX: -50 }}
          animate={{ opacity: 1, rotateX: 0 }}
          exit={{ opacity: 0, rotateX: 50 }}
          transition={{ duration: 0.4 }}
          className="ml-1"
        >
          {username}
        </motion.span>
      </AnimatePresence>
    </div>
  );
}
