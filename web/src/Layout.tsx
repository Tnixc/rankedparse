import { NavLink, Outlet } from "react-router-dom";

const NAV_ITEMS = [
  { to: "/completion-times", label: "Completion Times" },
  { to: "/forfeits", label: "Forfeits" },
  { to: "/splits", label: "Splits" },
  { to: "/timeline", label: "Timeline" },
];

export function Layout() {
  return (
    <div className="flex min-h-screen bg-white text-black dark:bg-black dark:text-white">
      <nav className="w-64 shrink-0 border-r border-border bg-surface sticky top-0 h-screen">
        <ul className="flex flex-col h-full">
          {NAV_ITEMS.map(({ to, label }) => (
            <li key={to}>
              <NavLink
                to={to}
                className={({ isActive }) =>
                  `block p-2 text-sm hover:underline ${
                    isActive ? "text-black font-bold" : "text-muted"
                  }`
                }
              >
                {label}
              </NavLink>
            </li>
          ))}
          <li className="text-sm mt-auto">
            <p>
              made by{" "}
              <a href="https://github.com/Tnixc" className="text-blue-700 hover:underline">
                Tnixc
              </a>
            </p>
            <p>
              <a
                href="https://github.com/Tnixc/rankedparse"
                className="text-blue-700 hover:underline"
              >
                Source on github
              </a>
            </p>
          </li>
        </ul>
      </nav>
      <main className="flex-1 p-4 min-w-0">
        <Outlet />
      </main>
    </div>
  );
}
