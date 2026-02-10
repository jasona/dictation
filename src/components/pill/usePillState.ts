import { useEffect, useRef, useState, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import type { PillState, AudioLevel } from "@/types";

/** Auto-dismiss delay for success state (ms) */
const SUCCESS_DISMISS_MS = 1500;
/** Fade-out animation duration (ms) */
const FADE_OUT_MS = 200;

export interface PillStateHook {
  state: PillState;
  audioLevel: number;
  errorMessage: string;
  isFadingOut: boolean;
}

export function usePillState(): PillStateHook {
  const [state, setState] = useState<PillState>("idle");
  const [audioLevel, setAudioLevel] = useState(0);
  const [errorMessage, setErrorMessage] = useState("");
  const [isFadingOut, setIsFadingOut] = useState(false);
  const dismissTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const fadeTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearTimers = useCallback(() => {
    if (dismissTimer.current) {
      clearTimeout(dismissTimer.current);
      dismissTimer.current = null;
    }
    if (fadeTimer.current) {
      clearTimeout(fadeTimer.current);
      fadeTimer.current = null;
    }
  }, []);

  const transitionTo = useCallback(
    (next: PillState, error?: string) => {
      clearTimers();
      setIsFadingOut(false);
      setState(next);
      if (error) setErrorMessage(error);

      if (next === "success") {
        dismissTimer.current = setTimeout(() => {
          setIsFadingOut(true);
          fadeTimer.current = setTimeout(() => {
            setState("idle");
            setIsFadingOut(false);
          }, FADE_OUT_MS);
        }, SUCCESS_DISMISS_MS);
      }
    },
    [clearTimers],
  );

  useEffect(() => {
    const unlisteners: Array<() => void> = [];

    const setup = async () => {
      // Dictation lifecycle events
      unlisteners.push(
        await listen("dictation://start", () => {
          transitionTo("recording");
        }),
      );

      unlisteners.push(
        await listen("dictation://stop", () => {
          transitionTo("processing");
        }),
      );

      // Pipeline result events
      unlisteners.push(
        await listen("pill://success", () => {
          transitionTo("success");
        }),
      );

      unlisteners.push(
        await listen<string>("pill://error", (event) => {
          transitionTo("error", event.payload || "An error occurred");
        }),
      );

      // Audio events
      unlisteners.push(
        await listen<AudioLevel>("audio://level", (event) => {
          setAudioLevel(event.payload.rms);
        }),
      );

      unlisteners.push(
        await listen("audio://no-speech", () => {
          setState((prev) => (prev === "recording" ? "noSpeech" : prev));
        }),
      );

      // Dismiss pill on hotkey press while in error/noSpeech
      unlisteners.push(
        await listen("pill://dismiss", () => {
          setIsFadingOut(true);
          fadeTimer.current = setTimeout(() => {
            setState("idle");
            setIsFadingOut(false);
          }, FADE_OUT_MS);
        }),
      );
    };

    setup();

    return () => {
      clearTimers();
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [transitionTo, clearTimers]);

  return { state, audioLevel, errorMessage, isFadingOut };
}
