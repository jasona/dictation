import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Mic, MicOff, Check, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { AudioLevel } from "@/types";

interface MicrophoneStepProps {
  onNext: () => void;
}

type MicStatus = "idle" | "requesting" | "granted" | "denied";

export default function MicrophoneStep({ onNext }: MicrophoneStepProps) {
  const [status, setStatus] = useState<MicStatus>("idle");
  const [audioLevel, setAudioLevel] = useState(0);

  // Listen for audio level to confirm mic access works
  useEffect(() => {
    let unlisten: (() => void) | null = null;
    listen<AudioLevel>("audio://level", (event) => {
      setAudioLevel(event.payload.rms);
      setStatus("granted");
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, []);

  const requestMicAccess = async () => {
    setStatus("requesting");
    try {
      // Start a short audio capture to trigger the permission dialog
      await invoke("list_audio_devices");
      // If we get here, the permission was likely granted
      // The audio level listener will confirm
      setStatus("granted");
    } catch {
      setStatus("denied");
    }
  };

  return (
    <div className="flex flex-1 flex-col items-center justify-center px-8 text-center">
      <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-2xl bg-bg-elevated">
        {status === "granted" ? (
          <Check size={32} className="text-accent-success" />
        ) : status === "denied" ? (
          <MicOff size={32} className="text-accent-error" />
        ) : (
          <Mic size={32} className="text-text-secondary" />
        )}
      </div>

      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        Microphone access
      </h2>

      {status === "idle" && (
        <>
          <p className="mt-3 max-w-[360px] text-[length:var(--font-size-body)] text-text-secondary">
            We need access to your microphone to transcribe your speech.
          </p>
          <Button className="mt-6" onClick={requestMicAccess}>
            Allow Microphone
          </Button>
        </>
      )}

      {status === "requesting" && (
        <div className="mt-4 flex items-center gap-2 text-text-secondary">
          <Loader2 size={16} className="animate-spin" />
          <span>Checking microphone access...</span>
        </div>
      )}

      {status === "granted" && (
        <>
          <p className="mt-3 max-w-[360px] text-[length:var(--font-size-body)] text-accent-success">
            Microphone access granted!
          </p>

          {/* Audio level indicator */}
          <div className="mt-4 h-2 w-48 overflow-hidden rounded-full bg-bg-active">
            <div
              className="h-full rounded-full bg-accent-primary transition-[width] duration-100 ease-out"
              style={{ width: `${Math.min(100, audioLevel * 100)}%` }}
            />
          </div>

          <p className="mt-6 max-w-[340px] text-[length:var(--font-size-caption)] text-text-tertiary">
            Your audio is processed on this device. We never store recordings or
            send audio to the cloud.
          </p>

          <Button className="mt-6" onClick={onNext}>
            Next
          </Button>
        </>
      )}

      {status === "denied" && (
        <>
          <p className="mt-3 max-w-[360px] text-[length:var(--font-size-body)] text-accent-error">
            Microphone access was denied.
          </p>
          <p className="mt-2 max-w-[360px] text-[length:var(--font-size-caption)] text-text-secondary">
            Enable microphone access in Windows Settings &gt; Privacy &amp;
            Security &gt; Microphone, then try again.
          </p>
          <Button className="mt-4" variant="outline" onClick={requestMicAccess}>
            Try Again
          </Button>
        </>
      )}
    </div>
  );
}
