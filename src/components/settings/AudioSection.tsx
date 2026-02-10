import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { RefreshCw } from "lucide-react";
import type { AudioDeviceInfo } from "@/hooks/useSettings";
import type { AudioLevel } from "@/types";

interface AudioSectionProps {
  audioDevices: AudioDeviceInfo[];
  audioDevice: string | null;
  onAudioDeviceChange: (deviceId: string | null) => Promise<void>;
  onRefreshDevices: () => Promise<void>;
}

export default function AudioSection({
  audioDevices,
  audioDevice,
  onAudioDeviceChange,
  onRefreshDevices,
}: AudioSectionProps) {
  const [audioLevel, setAudioLevel] = useState(0);

  useEffect(() => {
    let unlisten: (() => void) | null = null;
    listen<AudioLevel>("audio://level", (event) => {
      setAudioLevel(event.payload.rms);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, []);

  // Decay audio level when no events come in
  useEffect(() => {
    const interval = setInterval(() => {
      setAudioLevel((prev) => Math.max(0, prev - 0.02));
    }, 50);
    return () => clearInterval(interval);
  }, []);

  return (
    <section>
      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        Audio
      </h2>

      <div className="mt-4 space-y-5">
        {/* Microphone selection */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-text-primary">Microphone</Label>
            <p className="text-[length:var(--font-size-caption)] text-text-secondary">
              Input device for recording
            </p>
          </div>
          <div className="flex items-center gap-2">
            <Select
              value={audioDevice ?? "default"}
              onValueChange={(v) =>
                onAudioDeviceChange(v === "default" ? null : v)
              }
            >
              <SelectTrigger className="w-[200px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="default">System default</SelectItem>
                {audioDevices.map((d) => (
                  <SelectItem key={d.id} value={d.id}>
                    {d.name}
                    {d.isDefault ? " (Default)" : ""}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button
              variant="ghost"
              size="icon"
              onClick={onRefreshDevices}
              className="h-8 w-8"
            >
              <RefreshCw size={14} />
            </Button>
          </div>
        </div>

        {/* Audio level meter */}
        <div>
          <Label className="text-text-primary">Audio level</Label>
          <div className="mt-2 h-2 overflow-hidden rounded-full bg-bg-active">
            <div
              className="h-full rounded-full bg-accent-primary transition-[width] duration-100 ease-out"
              style={{ width: `${Math.min(100, audioLevel * 100)}%` }}
            />
          </div>
          <p className="mt-1 text-[length:var(--font-size-caption)] text-text-muted">
            Speak to test your microphone
          </p>
        </div>
      </div>
    </section>
  );
}
