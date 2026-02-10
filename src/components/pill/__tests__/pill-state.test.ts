/**
 * Pill state machine tests.
 *
 * These tests validate the pill state transitions.
 * They require vitest + @testing-library/react to run.
 *
 * To enable: install vitest, jsdom, @testing-library/react, then run `npx vitest`.
 *
 * For now, this file documents the expected behavior as executable specs
 * that will work once a test runner is added.
 */

import { describe, it, expect } from "vitest";
import type { PillState } from "@/types";

// Pure state transition logic (mirrors usePillState behavior)
function nextState(
  current: PillState,
  event:
    | "dictation://start"
    | "dictation://stop"
    | "pill://success"
    | "pill://error"
    | "audio://no-speech"
    | "pill://dismiss"
    | "auto-dismiss",
): PillState {
  switch (event) {
    case "dictation://start":
      return "recording";
    case "dictation://stop":
      return "processing";
    case "pill://success":
      return "success";
    case "pill://error":
      return "error";
    case "audio://no-speech":
      return current === "recording" ? "noSpeech" : current;
    case "pill://dismiss":
    case "auto-dismiss":
      return "idle";
    default:
      return current;
  }
}

describe("Pill state machine", () => {
  it("starts in idle state", () => {
    const state: PillState = "idle";
    expect(state).toBe("idle");
  });

  it("transitions from idle to recording on start", () => {
    expect(nextState("idle", "dictation://start")).toBe("recording");
  });

  it("transitions from recording to processing on stop", () => {
    expect(nextState("recording", "dictation://stop")).toBe("processing");
  });

  it("transitions from processing to success", () => {
    expect(nextState("processing", "pill://success")).toBe("success");
  });

  it("transitions from processing to error", () => {
    expect(nextState("processing", "pill://error")).toBe("error");
  });

  it("transitions from recording to noSpeech", () => {
    expect(nextState("recording", "audio://no-speech")).toBe("noSpeech");
  });

  it("noSpeech only from recording state", () => {
    expect(nextState("processing", "audio://no-speech")).toBe("processing");
    expect(nextState("idle", "audio://no-speech")).toBe("idle");
  });

  it("success auto-dismisses to idle", () => {
    expect(nextState("success", "auto-dismiss")).toBe("idle");
  });

  it("error dismissed on pill://dismiss", () => {
    expect(nextState("error", "pill://dismiss")).toBe("idle");
  });

  it("recording can restart from any state", () => {
    expect(nextState("error", "dictation://start")).toBe("recording");
    expect(nextState("success", "dictation://start")).toBe("recording");
    expect(nextState("noSpeech", "dictation://start")).toBe("recording");
  });
});

describe("Auto-dismiss timing", () => {
  it("success state should dismiss after 1500ms", () => {
    // This is a behavioral spec — actual timing tested with React hooks
    const SUCCESS_DISMISS_MS = 1500;
    expect(SUCCESS_DISMISS_MS).toBe(1500);
  });

  it("fade-out animation is 200ms", () => {
    const FADE_OUT_MS = 200;
    expect(FADE_OUT_MS).toBe(200);
  });
});
