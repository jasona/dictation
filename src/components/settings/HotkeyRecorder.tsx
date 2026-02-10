import { useCallback, useEffect, useRef, useState } from "react";
import { cn } from "@/lib/utils";

interface HotkeyRecorderProps {
  value: string;
  onChange: (hotkey: string) => Promise<void>;
}

/** Format a hotkey string for display (e.g., "ctrl+shift+space" → "Ctrl + Shift + Space") */
function formatHotkey(hotkey: string): string {
  return hotkey
    .split("+")
    .map((k) => k.charAt(0).toUpperCase() + k.slice(1))
    .join(" + ");
}

/** Convert a KeyboardEvent to a hotkey string */
function eventToHotkey(e: KeyboardEvent): string | null {
  const parts: string[] = [];
  if (e.ctrlKey) parts.push("ctrl");
  if (e.altKey) parts.push("alt");
  if (e.shiftKey) parts.push("shift");
  if (e.metaKey) parts.push("super");

  // Must have at least one modifier
  if (parts.length === 0) return null;

  // Ignore standalone modifier keys
  const key = e.key.toLowerCase();
  if (["control", "alt", "shift", "meta"].includes(key)) return null;

  // Map special keys
  const keyMap: Record<string, string> = {
    " ": "space",
    arrowup: "up",
    arrowdown: "down",
    arrowleft: "left",
    arrowright: "right",
    escape: "escape",
    enter: "enter",
    backspace: "backspace",
    delete: "delete",
    tab: "tab",
  };

  parts.push(keyMap[key] || key);
  return parts.join("+");
}

export default function HotkeyRecorder({ value, onChange }: HotkeyRecorderProps) {
  const [recording, setRecording] = useState(false);
  const [error, setError] = useState("");
  const btnRef = useRef<HTMLButtonElement>(null);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      // Escape cancels recording
      if (e.key === "Escape") {
        setRecording(false);
        return;
      }

      const hotkey = eventToHotkey(e);
      if (!hotkey) return; // Not a complete combo yet

      setRecording(false);
      setError("");
      onChange(hotkey).catch((err) => {
        setError(String(err));
      });
    },
    [onChange],
  );

  useEffect(() => {
    if (!recording) return;
    window.addEventListener("keydown", handleKeyDown, true);
    return () => window.removeEventListener("keydown", handleKeyDown, true);
  }, [recording, handleKeyDown]);

  // Click outside cancels recording
  useEffect(() => {
    if (!recording) return;
    const handleClick = (e: MouseEvent) => {
      if (btnRef.current && !btnRef.current.contains(e.target as Node)) {
        setRecording(false);
      }
    };
    window.addEventListener("mousedown", handleClick);
    return () => window.removeEventListener("mousedown", handleClick);
  }, [recording]);

  return (
    <div className="flex flex-col items-end gap-1">
      <button
        ref={btnRef}
        type="button"
        onClick={() => setRecording(!recording)}
        className={cn(
          "rounded-md border px-3 py-1.5 text-[length:var(--font-size-body-small)] transition-colors",
          recording
            ? "border-accent-primary bg-accent-primary/10 text-accent-primary"
            : "border-border-default bg-bg-elevated text-text-primary hover:bg-bg-hover",
        )}
      >
        {recording ? "Press a shortcut..." : formatHotkey(value)}
      </button>
      {error && (
        <span className="text-[length:var(--font-size-caption)] text-accent-error">
          {error}
        </span>
      )}
    </div>
  );
}
