import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import WelcomeStep from "./WelcomeStep";
import MicrophoneStep from "./MicrophoneStep";
import HotkeyStep from "./HotkeyStep";
import FirstDictationStep from "./FirstDictationStep";
import TrayStep from "./TrayStep";
import { cn } from "@/lib/utils";

const STEP_COUNT = 5;

export default function Onboarding() {
  const [step, setStep] = useState(0);
  const [micGranted, setMicGranted] = useState(false);

  const goNext = () => setStep((s) => Math.min(s + 1, STEP_COUNT - 1));

  const handleMicNext = () => {
    setMicGranted(true);
    goNext();
  };

  const handleSkip = () => {
    // Skip jumps to the tray step (last step)
    setStep(STEP_COUNT - 1);
  };

  const handleFinish = async () => {
    try {
      await invoke("set_onboarding_completed");
    } catch (e) {
      console.error("Failed to set onboarding completed:", e);
    }
    try {
      await invoke("hide_onboarding_window");
    } catch (e) {
      console.error("Failed to hide onboarding:", e);
    }
  };

  return (
    <div className="flex h-full flex-col bg-bg-base">
      {/* Step content */}
      <div className="flex flex-1 flex-col">
        {step === 0 && <WelcomeStep onNext={goNext} />}
        {step === 1 && <MicrophoneStep onNext={handleMicNext} />}
        {step === 2 && <HotkeyStep onNext={goNext} />}
        {step === 3 && <FirstDictationStep onNext={goNext} />}
        {step === 4 && <TrayStep onFinish={handleFinish} />}
      </div>

      {/* Bottom bar: progress dots + skip */}
      <div className="flex items-center justify-between px-8 pb-6">
        {/* Progress dots */}
        <div className="flex gap-2">
          {Array.from({ length: STEP_COUNT }, (_, i) => (
            <div
              key={i}
              className={cn(
                "h-2 w-2 rounded-full transition-colors",
                i === step ? "bg-accent-primary" : "bg-bg-active",
                i < step && "bg-accent-primary/40",
              )}
            />
          ))}
        </div>

        {/* Skip link - available after mic step (step >= 2) */}
        {micGranted && step >= 2 && step < STEP_COUNT - 1 && (
          <button
            type="button"
            onClick={handleSkip}
            className="text-[length:var(--font-size-caption)] text-text-tertiary hover:text-text-secondary"
          >
            Skip
          </button>
        )}
      </div>
    </div>
  );
}
