import { useMemo } from "react";

const BAR_COUNT = 14;
const MIN_HEIGHT = 4;
const MAX_HEIGHT = 28;

/** Deterministic per-bar sensitivity multipliers for visual variety */
const BAR_WEIGHTS = Array.from({ length: BAR_COUNT }, (_, i) => {
  // Bell-curve: center bars respond more than edges
  const center = (BAR_COUNT - 1) / 2;
  const dist = Math.abs(i - center) / center;
  return 0.5 + 0.5 * (1 - dist * dist);
});

interface PillWaveformProps {
  /** Audio RMS level 0.0–1.0 */
  level: number;
  /** Whether the waveform is in "no speech" flat state */
  isFlat?: boolean;
}

export default function PillWaveform({ level, isFlat }: PillWaveformProps) {
  const bars = useMemo(() => {
    return BAR_WEIGHTS.map((weight) => {
      if (isFlat) return MIN_HEIGHT;
      const h = MIN_HEIGHT + (MAX_HEIGHT - MIN_HEIGHT) * level * weight;
      return Math.min(MAX_HEIGHT, Math.max(MIN_HEIGHT, h));
    });
  }, [level, isFlat]);

  return (
    <div className="flex items-center gap-[3px]" aria-hidden="true">
      {bars.map((height, i) => (
        <div
          key={i}
          className="w-[3px] rounded-full bg-accent-primary motion-safe:transition-[height] motion-safe:duration-[100ms] motion-safe:ease-out"
          style={{ height: `${height}px` }}
        />
      ))}
    </div>
  );
}
