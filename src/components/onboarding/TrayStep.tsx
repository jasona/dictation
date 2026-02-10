import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { MonitorSpeaker, ArrowDown } from "lucide-react";
import { Button } from "@/components/ui/button";

interface TrayStepProps {
  onFinish: () => void;
}

/** Format a hotkey string for display */
function formatHotkey(hotkey: string): string {
  return hotkey
    .split("+")
    .map((k) => k.charAt(0).toUpperCase() + k.slice(1))
    .join(" + ");
}

export default function TrayStep({ onFinish }: TrayStepProps) {
  const [hotkey, setHotkey] = useState("ctrl+shift+space");

  useEffect(() => {
    invoke<string>("get_hotkey").then(setHotkey).catch(() => {});
  }, []);

  return (
    <div className="flex flex-1 flex-col items-center justify-center px-8 text-center">
      {/* Tray illustration */}
      <div className="mb-6 flex flex-col items-center gap-2">
        <div className="flex items-center gap-3 rounded-lg border border-border-subtle bg-bg-elevated px-4 py-2">
          <MonitorSpeaker size={20} className="text-text-secondary" />
          <div className="flex items-center gap-2">
            <div className="h-4 w-4 rounded bg-text-tertiary/30" />
            <div className="h-4 w-4 rounded bg-text-tertiary/30" />
            <div className="flex h-5 w-5 items-center justify-center rounded bg-accent-primary">
              <span className="text-[10px] font-bold text-white">D</span>
            </div>
            <div className="h-4 w-4 rounded bg-text-tertiary/30" />
          </div>
        </div>
        <ArrowDown size={16} className="text-accent-primary" />
        <span className="text-[length:var(--font-size-caption)] text-accent-primary">
          Find us here
        </span>
      </div>

      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        We'll be here whenever you need us
      </h2>

      <p className="mt-3 max-w-[360px] text-[length:var(--font-size-body)] text-text-secondary">
        Press{" "}
        <span className="font-medium text-text-primary">
          {formatHotkey(hotkey)}
        </span>{" "}
        anywhere to dictate. Look for the Dictation icon in your system tray.
      </p>

      <Button className="mt-8" size="lg" onClick={onFinish}>
        Finish
      </Button>
    </div>
  );
}
