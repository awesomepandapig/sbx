interface Section {
  id: string;
  title: string;
}

interface RightSidebarProps {
  sections: Section[];
  activeSection: string;
}

export default function RightSidebar({
  sections,
  activeSection,
}: RightSidebarProps) {
  return (
    <div className="w-64 shrink-0 fixed right-0 top-14 bottom-0 overflow-hidden">
      <div className="h-full overflow-y-auto">
        <ul className="mt-12 relative">
          {sections.map(({ id, title }) => (
            <li key={id} className="relative">
              <a
                href={`#${id}`}
                className={`block pl-4 py-2 border-l text-[13px] ${
                  activeSection === id
                    ? "border-blue-500 text-blue-500"
                    : "border-gray-800 text-gray-500 hover:text-blue-500"
                }`}
              >
                {activeSection === id && (
                  <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-blue-500"></div>
                )}
                {title}
              </a>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
