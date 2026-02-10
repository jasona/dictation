import { Button } from "@/components/ui/button";
import vozrLogo from "@/assets/vozr-logo.png";

interface WelcomeStepProps {
  onNext: () => void;
}

export default function WelcomeStep({ onNext }: WelcomeStepProps) {
  return (
    <div className="flex flex-1 flex-col items-center justify-center px-8 text-center">
      <img src={vozrLogo} alt="Vozr" className="mb-6 h-20 w-20" />

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
