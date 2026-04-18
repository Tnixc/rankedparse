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
      <nav className="w-64 shrink-0 border-r border-border bg-surface sticky top-0 h-screen p-0">
        <ul className="flex flex-col gap-0.5">
          {NAV_ITEMS.map(({ to, label }) => (
            <li key={to}>
              <NavLink
                to={to}
                className={({ isActive }) =>
                  `block px-3 py-2 text-sm hover:underline ${
                    isActive ? "text-black font-bold" : "text-muted"
                  }`
                }
              >
                {label}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
      <main className="flex-1 p-4 min-w-0">
        <Outlet />
      </main>
    </div>
  );
}
