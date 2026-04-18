import { useState, useMemo } from "react";
import {
  VisXYContainer,
  VisStackedBar,
  VisAxis,
  VisCrosshair,
  VisTooltip,
  VisPlotline,
} from "@unovis/react";
import { VisBulletLegend } from "@unovis/react";
import { seasons, DIVISIONS, divisionColors, type CompletionBucket, type Division } from "../data";

interface ChartDatum {
  minute: number;
  coal: number;
  iron: number;
  gold: number;
  emerald: number;
  diamond: number;
  netherite: number;
}

type Metric = "completions" | "players";

interface WeightedPoint {
  minute: number;
  weight: number;
}

interface StatsSummary {
  mean: number | null;
  median: number | null;
  totalWeight: number;
}

interface StatsRow extends StatsSummary {
  color: string;
  key: string;
  label: string;
}

function formatDivisionName(division: Division): string {
  return division.charAt(0).toUpperCase() + division.slice(1);
}

function computeStats(points: WeightedPoint[]): StatsSummary {
  const sortedPoints = points
    .filter((point) => point.weight > 0)
    .sort((a, b) => a.minute - b.minute);

  if (sortedPoints.length === 0) {
    return { mean: null, median: null, totalWeight: 0 };
  }

  let totalWeight = 0;
  let weightedMinuteSum = 0;
  for (const point of sortedPoints) {
    totalWeight += point.weight;
    weightedMinuteSum += point.minute * point.weight;
  }

  if (totalWeight === 0) {
    return { mean: null, median: null, totalWeight: 0 };
  }

  const midpoint = totalWeight / 2;
  let cumulativeWeight = 0;
  let median = sortedPoints[sortedPoints.length - 1].minute;

  for (const point of sortedPoints) {
    cumulativeWeight += point.weight;
    if (cumulativeWeight >= midpoint) {
      median = point.minute;
      break;
    }
  }

  return {
    mean: weightedMinuteSum / totalWeight,
    median,
    totalWeight,
  };
}

function computeDivisionStats(buckets: CompletionBucket[], metric: Metric): StatsSummary {
  return computeStats(
    buckets.map((bucket) => ({
      minute: bucket.minute,
      weight: bucket[metric],
    })),
  );
}

function formatMinutes(value: number | null): string {
  return value === null ? "-" : value.toFixed(2);
}

export function CompletionTimes() {
  const [seasonIndex, setSeasonIndex] = useState(seasons.length - 1);
  const [metric, setMetric] = useState<Metric>("completions");
  const [focusDivision, setFocusDivision] = useState<Division | "all">("all");
  const [showStatsLines, setShowStatsLines] = useState(true);

  const season = seasons[seasonIndex];
  const activeDivisions = focusDivision === "all" ? DIVISIONS : [focusDivision];
  const colorByDivision = divisionColors();

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

  const statsRows = useMemo<StatsRow[]>(() => {
    const totalByMinute = new Map<number, number>();

    const rows: StatsRow[] = activeDivisions.map((div) => {
      for (const bucket of season.completion_times[div].buckets) {
        if (bucket[metric] <= 0) continue;
        totalByMinute.set(bucket.minute, (totalByMinute.get(bucket.minute) ?? 0) + bucket[metric]);
      }

      return {
        key: div,
        label: formatDivisionName(div),
        color: colorByDivision[div],
        ...computeDivisionStats(season.completion_times[div].buckets, metric),
      };
    });

    const totalStats = computeStats(
      Array.from(totalByMinute, ([minute, weight]) => ({
        minute,
        weight,
      })),
    );

    rows.push({
      key: "total",
      label: "Total",
      color: "transparent",
      ...totalStats,
    });

    return rows;
  }, [activeDivisions, colorByDivision, metric, season]);

  const yAccessors = activeDivisions.map((div) => (d: ChartDatum) => d[div]);
  const colors = activeDivisions.map((div) => colorByDivision[div]);

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
    name: formatDivisionName(div),
    color: colorByDivision[div],
  }));

  const tooltipTemplate = (d: ChartDatum) => {
    const lines = activeDivisions
      .filter((div) => d[div] > 0)
      .map((div) => `<span style="color:${colorByDivision[div]}">&#9632;</span> ${div}: ${d[div]}`);
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
            <option key={s.season} value={i}>
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
          <option value="players">Player Averages</option>
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

        <label className="inline-flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={showStatsLines}
            onChange={(e) => setShowStatsLines(e.target.checked)}
            className="h-4 w-4"
          />
          Show mean/median lines
        </label>
      </div>

      <VisBulletLegend items={legendItems} />
      {showStatsLines ? (
        <p className="text-muted mt-2 text-xs">Dashed lines are means. Solid lines are medians.</p>
      ) : null}

      <VisXYContainer data={data} height={500} yDomain={yDomain} className="mt-2">
        <VisStackedBar<ChartDatum>
          x={(d) => d.minute}
          y={yAccessors}
          color={colors}
          dataStep={1}
          barPadding={0.1}
          roundedCorners={0}
        />
        {showStatsLines
          ? statsRows.map((row) =>
              row.mean === null ? null : (
                <VisPlotline<ChartDatum>
                  key={`${row.key}-mean`}
                  axis="x"
                  value={row.mean}
                  color={row.color}
                  lineWidth={2}
                  lineStyle={[6, 4]}
                />
              ),
            )
          : null}
        {showStatsLines
          ? statsRows.map((row) =>
              row.median === null ? null : (
                <VisPlotline<ChartDatum>
                  key={`${row.key}-median`}
                  axis="x"
                  value={row.median}
                  color={row.color}
                  lineWidth={2}
                />
              ),
            )
          : null}
        <VisAxis type="x" label="Minutes" numTicks={24} />
        <VisAxis type="y" label={metric === "completions" ? "Completions" : "Players"} />
        <VisCrosshair<ChartDatum> template={tooltipTemplate} />
        <VisTooltip />
      </VisXYContainer>

      <div className="mt-5 overflow-x-auto max-w-fit">
        <table className="min-w-full border border-border text-sm">
          <thead className="bg-surface">
            <tr>
              <th className="px-3 py-2 text-left font-medium">Division</th>
              <th className="px-3 py-2 text-right font-medium">Mean (min)</th>
              <th className="px-3 py-2 text-right font-medium">Median (min)</th>
            </tr>
          </thead>
          <tbody>
            {statsRows.map((row) => (
              <tr key={`stats-${row.key}`} className={row.key === "total" ? "font-semibold" : ""}>
                <td className="border-t border-border px-3 py-2">
                  <span className="inline-flex items-center gap-2">
                    <span
                      className="inline-block h-2.5 w-2.5 rounded-full"
                      style={{ backgroundColor: row.color }}
                      aria-hidden
                    />
                    {row.label}
                  </span>
                </td>
                <td className="border-t border-border px-3 py-2 text-right tabular-nums">
                  {formatMinutes(row.mean)}
                </td>
                <td className="border-t border-border px-3 py-2 text-right tabular-nums">
                  {formatMinutes(row.median)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <p className="text-sm max-w-96 p-2">
        The medians are not entirely accurate because during aggregation of the data they're all
        truncated down to the minute.
      </p>
    </div>
  );
}
