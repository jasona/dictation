import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Loader2, MessageSquare, Sparkles } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { WhisperModelInfo } from "@/hooks/useSettings";

interface FirstDictationStepProps {
  onNext: () => void;
}

type StepPhase =
  | "checkModel"
  | "downloading"
  | "ready"
  | "listening"
  | "processing"
  | "done";

export default function FirstDictationStep({
  onNext,
}: FirstDictationStepProps) {
  const [phase, setPhase] = useState<StepPhase>("checkModel");
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [transcription, setTranscription] = useState("");

  // Check if base.en model is available
  useEffect(() => {
    const check = async () => {
      try {
        const models = await invoke<WhisperModelInfo[]>("list_whisper_models");
        const baseEn = models.find((m) => m.id === "base.en");
        if (baseEn?.downloaded) {
          setPhase("ready");
        } else {
          // Need to download
          setPhase("downloading");
          await invoke("download_whisper_model", { modelId: "base.en" });
          await invoke("load_whisper_model", { modelId: "base.en" });
          setPhase("ready");
        }
      } catch (e) {
        console.error("Model check failed:", e);
        setPhase("ready"); // Proceed anyway
      }
    };
    check();
  }, []);

  // Listen for download progress
  useEffect(() => {
    let unlisten: (() => void) | null = null;
    listen<{ progress: number }>("stt://download-progress", (event) => {
      setDownloadProgress(event.payload.progress);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, []);

  // Listen for dictation events
  useEffect(() => {
    const unlisteners: Array<() => void> = [];

    const setup = async () => {
      unlisteners.push(
        await listen("vozr://start", () => {
          setPhase("listening");
        }),
      );

      unlisteners.push(
        await listen("vozr://stop", () => {
          setPhase("processing");
        }),
      );
    };

    setup();
    return () => {
      unlisteners.forEach((fn) => fn());
    };
  }, []);

  // Simulate getting the transcription result after processing
  // In the real pipeline, this would come from a pill://success event with payload
  useEffect(() => {
    if (phase !== "processing") return;

    // Listen for the result from the pipeline
    let unlisten: (() => void) | null = null;
    listen<string>("pill://success", () => {
      setTranscription("Your transcription will appear here in the real app!");
      setPhase("done");
    }).then((fn) => {
      unlisten = fn;
    });

    // Timeout fallback - show done after 5s if no event
    const timer = setTimeout(() => {
      if (phase === "processing") {
        setTranscription(
          "Great! Vozr is set up and ready to go.",
        );
        setPhase("done");
      }
    }, 5000);

    return () => {
      clearTimeout(timer);
      unlisten?.();
    };
  }, [phase]);

  return (
    <div className="flex flex-1 flex-col items-center justify-center px-8 text-center">
      <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-2xl bg-bg-elevated">
        {phase === "done" ? (
          <Sparkles size={32} className="text-accent-warning" />
        ) : (
          <MessageSquare size={32} className="text-text-secondary" />
        )}
      </div>

      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        {phase === "done" ? "Nice!" : "Try your first recording"}
      </h2>

      {/* Downloading model */}
      {phase === "downloading" && (
        <>
          <p className="mt-3 text-[length:var(--font-size-body)] text-text-secondary">
            Downloading speech recognition model...
          </p>
          <div className="mt-4 h-2 w-64 overflow-hidden rounded-full bg-bg-active">
            <div
              className="h-full rounded-full bg-accent-primary transition-[width] duration-300 ease-out"
              style={{ width: `${downloadProgress}%` }}
            />
          </div>
          <p className="mt-2 text-[length:var(--font-size-caption)] text-text-tertiary">
            {Math.round(downloadProgress)}%
          </p>
        </>
      )}

      {/* Checking model */}
      {phase === "checkModel" && (
        <div className="mt-4 flex items-center gap-2 text-text-secondary">
          <Loader2 size={16} className="animate-spin" />
          <span>Checking model...</span>
        </div>
      )}

      {/* Ready to dictate */}
      {phase === "ready" && (
        <>
          <p className="mt-3 max-w-[360px] text-[length:var(--font-size-body)] text-text-secondary">
            Press your hotkey and say something!
          </p>
          <p className="mt-2 text-[length:var(--font-size-body)] text-accent-primary">
            The floating pill will appear when you start recording.
          </p>
        </>
      )}

      {/* Listening */}
      {phase === "listening" && (
        <p className="mt-3 text-[length:var(--font-size-body)] text-accent-primary">
          Listening... speak now!
        </p>
      )}

      {/* Processing */}
      {phase === "processing" && (
        <div className="mt-4 flex items-center gap-2 text-text-secondary">
          <Loader2 size={16} className="animate-spin" />
          <span>Processing your speech...</span>
        </div>
      )}

      {/* Done - show transcription */}
      {phase === "done" && (
        <>
          <div className="mt-4 w-full max-w-[380px] rounded-lg border border-accent-primary/30 bg-accent-primary/5 px-4 py-3">
            <p className="text-[length:var(--font-size-body)] text-text-primary">
              {transcription}
            </p>
          </div>
          <Button className="mt-6" onClick={onNext}>
            Next
          </Button>
        </>
      )}

      {/* Skip option for non-done states */}
      {phase !== "done" && phase !== "downloading" && phase !== "checkModel" && (
        <Button className="mt-8" variant="ghost" size="sm" onClick={onNext}>
          Skip
        </Button>
      )}
    </div>
  );
}
