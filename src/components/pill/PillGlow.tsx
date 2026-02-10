interface PillGlowProps {
  /** Whether the glow is active (recording state) */
  active: boolean;
}

export default function PillGlow({ active }: PillGlowProps) {
  if (!active) return null;

  return (
    <div
      className="pointer-events-none absolute inset-0 rounded-full motion-safe:animate-[glow-pulse_2s_ease-in-out_infinite]"
      style={{
        boxShadow: "0 0 12px 4px rgba(99, 102, 241, 0.35)",
      }}
      aria-hidden="true"
    />
  );
}
