import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Mic, Loader2, Check, AlertCircle } from "lucide-react";
import { usePillState } from "./usePillState";
import { usePillPosition } from "./usePillPosition";
import PillWaveform from "./PillWaveform";
import PillGlow from "./PillGlow";

export default function Pill() {
  const { state, audioLevel, errorMessage, isFadingOut } = usePillState();
  usePillPosition();

  // Show/hide the Tauri window based on pill state
  useEffect(() => {
    if (state === "idle") {
      invoke("hide_pill_window").catch(() => {});
    } else {
      invoke("show_pill_window").catch(() => {});
    }
  }, [state]);

  // Don't render content when idle (window is hidden anyway)
  if (state === "idle" && !isFadingOut) return null;

  return (
    <div
      className="flex h-full items-center justify-center bg-transparent"
      data-tauri-drag-region
    >
      <div
        className={`relative flex h-[52px] w-[280px] items-center justify-center rounded-full border border-border-subtle bg-bg-elevated/90 backdrop-blur-md ${
          isFadingOut
            ? "motion-safe:animate-[fade-out_200ms_ease-out_forwards]"
            : "motion-safe:animate-[fade-in_200ms_ease-out]"
        }`}
      >
        <PillGlow active={state === "recording"} />

        {/* Recording state */}
        {state === "recording" && (
          <div className="flex items-center gap-3">
            <Mic size={16} className="shrink-0 text-accent-primary" />
            <PillWaveform level={audioLevel} />
          </div>
        )}

        {/* Processing state */}
        {state === "processing" && (
          <div className="flex items-center gap-2">
            <Loader2
              size={16}
              className="shrink-0 text-text-secondary motion-safe:animate-spin"
            />
            <span className="text-[length:var(--font-size-body-small)] text-text-secondary">
              Processing...
            </span>
          </div>
        )}

        {/* Success state */}
        {state === "success" && (
          <div className="flex items-center gap-2">
            <Check size={16} className="shrink-0 text-accent-success" />
            <span className="text-[length:var(--font-size-body-small)] text-accent-success">
              Done
            </span>
          </div>
        )}

        {/* Error state */}
        {state === "error" && (
          <div className="flex items-center gap-2 px-4">
            <AlertCircle size={16} className="shrink-0 text-accent-error" />
            <span className="truncate text-[length:var(--font-size-body-small)] text-accent-error">
              {errorMessage || "Error"}
            </span>
          </div>
        )}

        {/* No-speech state */}
        {state === "noSpeech" && (
          <div className="flex items-center gap-3">
            <PillWaveform level={0} isFlat />
            <span className="text-[length:var(--font-size-body-small)] text-accent-warning motion-safe:animate-pulse">
              Waiting for you...
            </span>
          </div>
        )}
      </div>
    </div>
  );
}
