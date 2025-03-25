export default function Footer() {
  return (
    <footer className="border-t border-gray-800 py-6 md:py-8 text-center">
      <p className="text-sm text-gray-500">
        SBX is not affiliated with or endorsed by Hypixel Inc.
      </p>
      <p className="text-sm text-gray-500">Released under the MIT License.</p>
      <p className="text-sm text-gray-500">
        Copyright © {new Date().getFullYear()} SBX. All rights reserved.
      </p>
    </footer>
  );
}
