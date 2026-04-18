import { seasons } from "../data";

export function Timeline() {
  return (
    <div>
      <h2 className="text-2xl font-semibold mb-1">Timeline Events</h2>
      <p className="text-muted mb-8 text-sm">Event frequency across {seasons.length} seasons.</p>
      <div className="grid grid-cols-1 gap-6">
        <div className="border border-border p-5">
          <div className="h-50 flex items-center justify-center border-2 border-dashed border-border text-muted text-sm">
            Chart placeholder
          </div>
        </div>
      </div>
    </div>
  );
}
