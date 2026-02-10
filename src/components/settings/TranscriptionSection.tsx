import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Download, Trash2, Loader2, Check } from "lucide-react";
import type { WhisperModelInfo, GpuBackendInfo } from "@/hooks/useSettings";

interface TranscriptionSectionProps {
  whisperModels: WhisperModelInfo[];
  currentWhisperModel: string | null;
  gpuBackends: GpuBackendInfo[];
  gpuBackend: string;
  onGpuBackendChange: (backend: string) => Promise<void>;
  onDownloadModel: (modelId: string) => Promise<void>;
  onDeleteModel: (modelId: string) => Promise<void>;
  onLoadModel: (modelId: string) => Promise<void>;
}

function formatSize(bytes: number): string {
  const mb = bytes / (1024 * 1024);
  return mb >= 1024 ? `${(mb / 1024).toFixed(1)} GB` : `${Math.round(mb)} MB`;
}

export default function TranscriptionSection({
  whisperModels,
  currentWhisperModel,
  gpuBackends,
  gpuBackend,
  onGpuBackendChange,
  onDownloadModel,
  onDeleteModel,
  onLoadModel,
}: TranscriptionSectionProps) {
  const [busyModel, setBusyModel] = useState<string | null>(null);

  const handleDownload = async (modelId: string) => {
    setBusyModel(modelId);
    try {
      await onDownloadModel(modelId);
    } catch (e) {
      console.error("Download failed:", e);
    } finally {
      setBusyModel(null);
    }
  };

  const handleDelete = async (modelId: string) => {
    setBusyModel(modelId);
    try {
      await onDeleteModel(modelId);
    } catch (e) {
      console.error("Delete failed:", e);
    } finally {
      setBusyModel(null);
    }
  };

  const handleLoad = async (modelId: string) => {
    setBusyModel(modelId);
    try {
      await onLoadModel(modelId);
    } catch (e) {
      console.error("Load failed:", e);
    } finally {
      setBusyModel(null);
    }
  };

  return (
    <section>
      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        Transcription
      </h2>

      <div className="mt-4 space-y-5">
        {/* Whisper models */}
        <div>
          <Label className="text-text-primary">Whisper model</Label>
          <p className="mb-3 text-[length:var(--font-size-caption)] text-text-secondary">
            Larger models are more accurate but slower
          </p>
          <div className="space-y-2">
            {whisperModels.map((model) => {
              const isLoaded = currentWhisperModel === model.id;
              const isBusy = busyModel === model.id;
              return (
                <div
                  key={model.id}
                  className="flex items-center justify-between rounded-md border border-border-subtle bg-bg-elevated px-3 py-2"
                >
                  <div className="min-w-0 flex-1">
                    <div className="flex items-center gap-2">
                      <span className="text-[length:var(--font-size-body-small)] text-text-primary">
                        {model.name}
                      </span>
                      <span className="text-[length:var(--font-size-caption)] text-text-tertiary">
                        {formatSize(model.sizeBytes)}
                      </span>
                      {isLoaded && (
                        <span className="flex items-center gap-1 text-[length:var(--font-size-caption)] text-accent-success">
                          <Check size={12} /> Active
                        </span>
                      )}
                    </div>
                    <p className="text-[length:var(--font-size-caption)] text-text-muted">
                      {model.description}
                    </p>
                  </div>
                  <div className="ml-3 flex items-center gap-1">
                    {isBusy ? (
                      <Loader2 size={16} className="animate-spin text-text-secondary" />
                    ) : model.downloaded ? (
                      <>
                        {!isLoaded && (
                          <Button
                            variant="ghost"
                            size="xs"
                            onClick={() => handleLoad(model.id)}
                          >
                            Load
                          </Button>
                        )}
                        <Button
                          variant="ghost"
                          size="xs"
                          onClick={() => handleDelete(model.id)}
                          className="text-accent-error hover:text-accent-error"
                        >
                          <Trash2 size={14} />
                        </Button>
                      </>
                    ) : (
                      <Button
                        variant="ghost"
                        size="xs"
                        onClick={() => handleDownload(model.id)}
                      >
                        <Download size={14} className="mr-1" /> Download
                      </Button>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* GPU backend */}
        <div className="flex items-center justify-between">
          <div>
            <Label className="text-text-primary">GPU acceleration</Label>
            <p className="text-[length:var(--font-size-caption)] text-text-secondary">
              Hardware backend for transcription
            </p>
          </div>
          <Select value={gpuBackend} onValueChange={onGpuBackendChange}>
            <SelectTrigger className="w-[160px]">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {gpuBackends.map((b) => (
                <SelectItem key={b.id} value={b.id} disabled={!b.available}>
                  {b.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>
    </section>
  );
}
