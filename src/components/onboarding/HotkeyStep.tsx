import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Check, Keyboard } from "lucide-react";
import { Button } from "@/components/ui/button";
import HotkeyRecorder from "@/components/settings/HotkeyRecorder";

interface HotkeyStepProps {
  onNext: () => void;
}

/** Format a hotkey string for display */
function formatHotkey(hotkey: string): string {
  return hotkey
    .split("+")
    .map((k) => k.charAt(0).toUpperCase() + k.slice(1))
    .join(" + ");
}

export default function HotkeyStep({ onNext }: HotkeyStepProps) {
  const [hotkey, setHotkey] = useState("ctrl+shift+space");
  const [pressed, setPressed] = useState(false);

  // Load current hotkey
  useEffect(() => {
    invoke<string>("get_hotkey").then(setHotkey).catch(() => {});
  }, []);

  // Listen for dictation start to detect hotkey press
  useEffect(() => {
    let unlisten: (() => void) | null = null;
    listen("dictation://start", () => {
      setPressed(true);
      // Auto-stop the recording since this is just a test
      invoke("set_is_paused", { paused: false }).catch(() => {});
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, []);

  const handleHotkeyChange = async (newHotkey: string) => {
    await invoke("set_hotkey", { shortcut: newHotkey });
    setHotkey(newHotkey);
    setPressed(false);
  };

  return (
    <div className="flex flex-1 flex-col items-center justify-center px-8 text-center">
      <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-2xl bg-bg-elevated">
        {pressed ? (
          <Check size={32} className="text-accent-success" />
        ) : (
          <Keyboard size={32} className="text-text-secondary" />
        )}
      </div>

      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        Your activation hotkey
      </h2>

      <p className="mt-3 text-[length:var(--font-size-body)] text-text-secondary">
        Press this shortcut anywhere to start dictating
      </p>

      {/* Hotkey display */}
      <div className="mt-6 rounded-lg border border-border-subtle bg-bg-elevated px-6 py-3">
        <span className="text-[length:var(--font-size-heading-1)] font-semibold text-text-primary">
          {formatHotkey(hotkey)}
        </span>
      </div>

      {/* Customize option */}
      <div className="mt-4 flex items-center gap-2 text-[length:var(--font-size-caption)] text-text-tertiary">
        <span>Want a different shortcut?</span>
        <HotkeyRecorder value={hotkey} onChange={handleHotkeyChange} />
      </div>

      {/* Try it prompt */}
      {!pressed ? (
        <p className="mt-6 text-[length:var(--font-size-body)] text-accent-primary">
          Try pressing it now!
        </p>
      ) : (
        <p className="mt-6 text-[length:var(--font-size-body)] text-accent-success">
          It works!
        </p>
      )}

      <Button className="mt-6" onClick={onNext}>
        Next
      </Button>
    </div>
  );
}
