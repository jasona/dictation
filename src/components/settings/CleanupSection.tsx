import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Eye, EyeOff, Loader2, Check } from "lucide-react";
import type { CleanupTier, CloudProvider } from "@/types";

interface CleanupSectionProps {
  cleanupTier: CleanupTier;
  cloudProvider: CloudProvider;
  apiKeyExists: { openai: boolean; anthropic: boolean };
  onCleanupTierChange: (tier: CleanupTier) => Promise<void>;
  onCloudProviderChange: (provider: CloudProvider) => Promise<void>;
  onSaveApiKey: (provider: CloudProvider, key: string) => Promise<void>;
  onDeleteApiKey: (provider: CloudProvider) => Promise<void>;
  onTestApiKey: (provider: CloudProvider) => Promise<void>;
}

const TIER_INFO: Record<CleanupTier, { label: string; description: string }> = {
  rules: {
    label: "Basic (Rules)",
    description: "Fast rule-based cleanup: filler removal, capitalization, punctuation",
  },
  localLlm: {
    label: "Enhanced (Local LLM)",
    description: "On-device AI for grammar, rephrasing, and context-aware cleanup",
  },
  cloudLlm: {
    label: "Maximum (Cloud LLM)",
    description: "Cloud AI for highest quality cleanup with advanced rephrasing",
  },
};

export default function CleanupSection({
  cleanupTier,
  cloudProvider,
  apiKeyExists,
  onCleanupTierChange,
  onCloudProviderChange,
  onSaveApiKey,
  onDeleteApiKey,
  onTestApiKey,
}: CleanupSectionProps) {
  const [apiKey, setApiKey] = useState("");
  const [showKey, setShowKey] = useState(false);
  const [testStatus, setTestStatus] = useState<"idle" | "testing" | "success" | "error">("idle");

  const currentKeyExists = cloudProvider === "openai" ? apiKeyExists.openai : apiKeyExists.anthropic;

  const handleSaveKey = async () => {
    if (!apiKey.trim()) return;
    try {
      await onSaveApiKey(cloudProvider, apiKey.trim());
      setApiKey("");
    } catch (e) {
      console.error("Failed to save key:", e);
    }
  };

  const handleTestKey = async () => {
    setTestStatus("testing");
    try {
      await onTestApiKey(cloudProvider);
      setTestStatus("success");
      setTimeout(() => setTestStatus("idle"), 2000);
    } catch {
      setTestStatus("error");
      setTimeout(() => setTestStatus("idle"), 2000);
    }
  };

  return (
    <section>
      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        AI Cleanup
      </h2>

      <div className="mt-4 space-y-5">
        {/* Cleanup tier */}
        <div>
          <Label className="text-text-primary">Cleanup tier</Label>
          <div className="mt-2 space-y-2">
            {(Object.keys(TIER_INFO) as CleanupTier[]).map((tier) => {
              const info = TIER_INFO[tier];
              const isActive = cleanupTier === tier;
              return (
                <button
                  key={tier}
                  type="button"
                  onClick={() => onCleanupTierChange(tier)}
                  className={`w-full rounded-md border px-3 py-2.5 text-left transition-colors ${
                    isActive
                      ? "border-accent-primary bg-accent-primary/10"
                      : "border-border-subtle bg-bg-elevated hover:bg-bg-hover"
                  }`}
                >
                  <span
                    className={`text-[length:var(--font-size-body-small)] ${
                      isActive ? "text-accent-primary" : "text-text-primary"
                    }`}
                  >
                    {info.label}
                  </span>
                  <p className="text-[length:var(--font-size-caption)] text-text-secondary">
                    {info.description}
                  </p>
                </button>
              );
            })}
          </div>
        </div>

        {/* Cloud provider + API key (only shown when cloud tier selected) */}
        {cleanupTier === "cloudLlm" && (
          <>
            <div className="flex items-center justify-between">
              <div>
                <Label className="text-text-primary">Cloud provider</Label>
                <p className="text-[length:var(--font-size-caption)] text-text-secondary">
                  AI service for cloud cleanup
                </p>
              </div>
              <Select
                value={cloudProvider}
                onValueChange={(v) => onCloudProviderChange(v as CloudProvider)}
              >
                <SelectTrigger className="w-[160px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="openai">OpenAI</SelectItem>
                  <SelectItem value="anthropic">Anthropic</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {/* API key management */}
            <div>
              <Label className="text-text-primary">API key</Label>
              {currentKeyExists ? (
                <div className="mt-2 flex items-center gap-2">
                  <span className="flex items-center gap-1 text-[length:var(--font-size-body-small)] text-accent-success">
                    <Check size={14} /> Key saved
                  </span>
                  <Button
                    variant="ghost"
                    size="xs"
                    onClick={handleTestKey}
                    disabled={testStatus === "testing"}
                  >
                    {testStatus === "testing" ? (
                      <Loader2 size={14} className="mr-1 animate-spin" />
                    ) : null}
                    {testStatus === "success"
                      ? "Valid!"
                      : testStatus === "error"
                        ? "Failed"
                        : "Test"}
                  </Button>
                  <Button
                    variant="ghost"
                    size="xs"
                    className="text-accent-error hover:text-accent-error"
                    onClick={() => onDeleteApiKey(cloudProvider)}
                  >
                    Remove
                  </Button>
                </div>
              ) : (
                <div className="mt-2 flex items-center gap-2">
                  <div className="relative flex-1">
                    <Input
                      type={showKey ? "text" : "password"}
                      placeholder={`Enter ${cloudProvider === "openai" ? "OpenAI" : "Anthropic"} API key`}
                      value={apiKey}
                      onChange={(e) => setApiKey(e.target.value)}
                      onKeyDown={(e) => e.key === "Enter" && handleSaveKey()}
                      className="pr-8"
                    />
                    <button
                      type="button"
                      onClick={() => setShowKey(!showKey)}
                      className="absolute right-2 top-1/2 -translate-y-1/2 text-text-tertiary hover:text-text-primary"
                    >
                      {showKey ? <EyeOff size={14} /> : <Eye size={14} />}
                    </button>
                  </div>
                  <Button size="sm" onClick={handleSaveKey} disabled={!apiKey.trim()}>
                    Save
                  </Button>
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </section>
  );
}
