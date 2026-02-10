import { useState } from "react";
import { Separator } from "@/components/ui/separator";
import { useSettings } from "@/hooks/useSettings";
import GeneralSection from "./GeneralSection";
import TranscriptionSection from "./TranscriptionSection";
import CleanupSection from "./CleanupSection";
import AudioSection from "./AudioSection";
import VocabularySection from "./VocabularySection";

export default function Settings() {
  const settings = useSettings();
  const [vocabulary, setVocabulary] = useState<string[]>([]);

  if (settings.loading) {
    return (
      <div className="flex h-full items-center justify-center bg-bg-surface">
        <span className="text-text-secondary">Loading settings...</span>
      </div>
    );
  }

  return (
    <div className="h-full overflow-y-auto bg-bg-surface">
      <div className="mx-auto max-w-[480px] space-y-6 px-6 py-8">
        <h1 className="text-[length:var(--font-size-heading-1)] font-semibold text-text-primary">
          Settings
        </h1>

        <GeneralSection
          hotkey={settings.hotkey}
          activationMode={settings.activationMode}
          autostart={settings.autostart}
          onHotkeyChange={settings.setHotkey}
          onActivationModeChange={settings.setActivationMode}
          onAutostartChange={settings.setAutostart}
        />

        <Separator className="bg-border-subtle" />

        <AudioSection
          audioDevices={settings.audioDevices}
          audioDevice={settings.audioDevice}
          onAudioDeviceChange={settings.setAudioDevice}
          onRefreshDevices={settings.refreshAudioDevices}
        />

        <Separator className="bg-border-subtle" />

        <TranscriptionSection
          whisperModels={settings.whisperModels}
          currentWhisperModel={settings.currentWhisperModel}
          gpuBackends={settings.gpuBackends}
          gpuBackend={settings.gpuBackend}
          onGpuBackendChange={settings.setGpuBackend}
          onDownloadModel={settings.downloadWhisperModel}
          onDeleteModel={settings.deleteWhisperModel}
          onLoadModel={settings.loadWhisperModel}
        />

        <Separator className="bg-border-subtle" />

        <CleanupSection
          cleanupTier={settings.cleanupTier}
          cloudProvider={settings.cloudProvider}
          apiKeyExists={settings.apiKeyExists}
          onCleanupTierChange={settings.setCleanupTier}
          onCloudProviderChange={settings.setCloudProvider}
          onSaveApiKey={settings.saveApiKey}
          onDeleteApiKey={settings.deleteApiKey}
          onTestApiKey={settings.testApiKey}
        />

        <Separator className="bg-border-subtle" />

        <VocabularySection
          vocabulary={vocabulary}
          onVocabularyChange={setVocabulary}
        />

        {/* Bottom padding for scroll */}
        <div className="h-6" />
      </div>
    </div>
  );
}
