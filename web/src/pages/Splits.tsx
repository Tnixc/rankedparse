import { seasons, DIVISIONS } from "../data";

export function Splits() {
  return (
    <div>
      <h2 className="text-2xl font-semibold mb-1">Splits</h2>
      <p className="text-muted mb-8 text-sm">
        Average split times per division across {seasons.length} seasons.
      </p>
      <div className="grid grid-cols-[repeat(auto-fill,minmax(400px,1fr))] gap-6">
        {DIVISIONS.map((div) => (
          <div key={div} className="border border-border p-5">
            <h3 className="text-sm font-medium mb-4 capitalize">{div}</h3>
            <div className="h-50 flex items-center justify-center border-2 border-dashed border-border text-muted text-sm">
              Chart placeholder
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
