export interface CompletionBucket {
  completions: number;
  minute: number;
  players: number;
}

export interface DivisionCompletionTimes {
  buckets: CompletionBucket[];
  total_completions: number;
  total_players: number;
}

export interface ForfeitData {
  forfeited: number;
  total: number;
}

export interface SplitData {
  avg_ms: number;
  count: number;
  name: string;
}

export interface TimelineEvent {
  count: number;
  event: string;
}

export type Division = "coal" | "iron" | "gold" | "emerald" | "diamond" | "netherite";

export interface SeasonData {
  completion_times: Record<Division, DivisionCompletionTimes>;
  errors: number;
  forfeits: Record<Division, ForfeitData>;
  records: number;
  season: string;
  splits: Record<Division, SplitData[]>;
  timeline: {
    events: TimelineEvent[];
    total_matches: number;
  };
}

const raw = import.meta.glob("../../output/analytics.jsonl", {
  query: "?raw",
  import: "default",
  eager: true,
});

const jsonlContent = Object.values(raw)[0] as string;

export const seasons: SeasonData[] = jsonlContent
  .trim()
  .split("\n")
  .map((line) => JSON.parse(line));

export const DIVISIONS: Division[] = ["netherite", "diamond", "emerald", "gold", "iron", "coal"];
