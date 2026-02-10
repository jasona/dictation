import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { ActivationMode } from "@/types";
import HotkeyRecorder from "./HotkeyRecorder";

interface GeneralSectionProps {
  hotkey: string;
  activationMode: ActivationMode;
  autostart: boolean;
  onHotkeyChange: (hotkey: string) => Promise<void>;
  onActivationModeChange: (mode: ActivationMode) => Promise<void>;
  onAutostartChange: (enabled: boolean) => Promise<void>;
}

export default function GeneralSection({
  hotkey,
  activationMode,
  autostart,
  onHotkeyChange,
  onActivationModeChange,
  onAutostartChange,
}: GeneralSectionProps) {
  return (
    <section>
      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        General
      </h2>

      <div className="mt-4 space-y-5">
        {/* Launch at startup */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-text-primary">Launch at startup</Label>
            <p className="text-[length:var(--font-size-caption)] text-text-secondary">
              Start the app when you log in
            </p>
          </div>
          <Switch
            checked={autostart}
            onCheckedChange={(checked) => onAutostartChange(checked)}
          />
        </div>

        {/* Activation hotkey */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-text-primary">Activation hotkey</Label>
            <p className="text-[length:var(--font-size-caption)] text-text-secondary">
              Press to start/stop dictation
            </p>
          </div>
          <HotkeyRecorder value={hotkey} onChange={onHotkeyChange} />
        </div>

        {/* Activation mode */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-text-primary">Activation mode</Label>
            <p className="text-[length:var(--font-size-caption)] text-text-secondary">
              How the hotkey triggers recording
            </p>
          </div>
          <Select
            value={activationMode}
            onValueChange={(v) => onActivationModeChange(v as ActivationMode)}
          >
            <SelectTrigger className="w-[160px]">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="toggle">Toggle</SelectItem>
              <SelectItem value="hold">Push-to-hold</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
    </section>
  );
}
