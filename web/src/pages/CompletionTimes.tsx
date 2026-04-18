import { useState, useMemo } from "react";
import { VisXYContainer, VisStackedBar, VisAxis, VisCrosshair, VisTooltip } from "@unovis/react";
import { VisBulletLegend } from "@unovis/react";
import { seasons, DIVISIONS, divisionColors, type Division } from "../data";

interface ChartDatum {
  minute: number;
  coal: number;
  iron: number;
  gold: number;
  emerald: number;
  diamond: number;
  netherite: number;
}

export function CompletionTimes() {
  const [seasonIndex, setSeasonIndex] = useState(seasons.length - 1);
  const [metric, setMetric] = useState<"completions" | "players">("completions");
  const [focusDivision, setFocusDivision] = useState<Division | "all">("all");

  const season = seasons[seasonIndex];
  const activeDivisions = focusDivision === "all" ? DIVISIONS : [focusDivision];

  const data = useMemo(() => {
    const result: ChartDatum[] = [];
    for (let m = 0; m <= 120; m++) {
      const datum: ChartDatum = {
        minute: m,
        coal: 0,
        iron: 0,
        gold: 0,
        emerald: 0,
        diamond: 0,
        netherite: 0,
      };
      for (const div of DIVISIONS) {
        const bucket = season.completion_times[div].buckets.find((b) => b.minute === m);
        if (bucket) datum[div] = bucket[metric];
      }
      result.push(datum);
    }
    return result;
  }, [season, metric]);

  const yAccessors = activeDivisions.map((div) => (d: ChartDatum) => d[div]);

  const yMax = useMemo(() => {
    let max = 0;
    for (const d of data) {
      let sum = 0;
      for (const div of activeDivisions) sum += d[div];
      if (sum > max) max = sum;
    }
    return max;
  }, [data, activeDivisions]);
  const yDomain: [number, number] = [0, Math.ceil(yMax * 1.15)];

  const legendItems = activeDivisions.map((div) => ({
    name: div.charAt(0).toUpperCase() + div.slice(1),
    color: divisionColors()[div],
  }));

  const tooltipTemplate = (d: ChartDatum) => {
    const lines = activeDivisions
      .filter((div) => d[div] > 0)
      .map(
        (div) => `<span style="color:${divisionColors()[div]}">&#9632;</span> ${div}: ${d[div]}`,
      );
    if (lines.length === 0) return "";
    return `<div style="font-size:12px"><strong>${d.minute} min</strong><br/>${lines.join("<br/>")}</div>`;
  };

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-1">Completion Times</h2>
      <p className="text-muted mb-4 text-sm">
        Distribution of completion times (in minutes) per division.
      </p>

      <div className="flex gap-3 mb-4 items-center flex-wrap">
        <select
          value={seasonIndex}
          onChange={(e) => setSeasonIndex(Number(e.target.value))}
          className="border border-border bg-surface px-2 py-1 text-sm"
        >
          {seasons.map((s, i) => (
            <option key={i} value={i}>
              {s.season === "all" ? "All Seasons" : `Season ${s.season}`}
            </option>
          ))}
        </select>

        <select
          value={metric}
          onChange={(e) => setMetric(e.target.value as "completions" | "players")}
          className="border border-border bg-surface px-2 py-1 text-sm"
        >
          <option value="completions">Completions</option>
          <option value="players">Players</option>
        </select>

        <select
          value={focusDivision}
          onChange={(e) => setFocusDivision(e.target.value as Division | "all")}
          className="border border-border bg-surface px-2 py-1 text-sm"
        >
          <option value="all">All Divisions</option>
          {DIVISIONS.map((div) => (
            <option key={div} value={div}>
              {div.charAt(0).toUpperCase() + div.slice(1)}
            </option>
          ))}
        </select>
      </div>

      <VisBulletLegend items={legendItems} />

      <VisXYContainer data={data} height={500} yDomain={yDomain} className="mt-2">
        <VisStackedBar<ChartDatum>
          x={(d) => d.minute}
          y={yAccessors}
          dataStep={1}
          barPadding={0.1}
          roundedCorners={0}
        />
        <VisAxis type="x" label="Minutes" numTicks={24} />
        <VisAxis type="y" label={metric === "completions" ? "Completions" : "Players"} />
        <VisCrosshair<ChartDatum> template={tooltipTemplate} />
        <VisTooltip />
      </VisXYContainer>
    </div>
  );
}
