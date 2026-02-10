import { Mic } from "lucide-react";
import { Button } from "@/components/ui/button";

interface WelcomeStepProps {
  onNext: () => void;
}

export default function WelcomeStep({ onNext }: WelcomeStepProps) {
  return (
    <div className="flex flex-1 flex-col items-center justify-center px-8 text-center">
      <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-2xl bg-accent-primary/15">
        <Mic size={32} className="text-accent-primary" />
      </div>

      <h1 className="text-[length:var(--font-size-heading-1)] font-semibold text-text-primary">
        Dictate anywhere on your computer
      </h1>

      <p className="mt-3 max-w-[360px] text-[length:var(--font-size-body)] text-text-secondary">
        3x faster than typing. Private by default.
      </p>

      <Button className="mt-8" size="lg" onClick={onNext}>
        Get Started
      </Button>
    </div>
  );
}
