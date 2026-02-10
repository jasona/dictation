/** State of the floating pill UI */
export type PillState =
  | "idle"
  | "recording"
  | "processing"
  | "success"
  | "error"
  | "noSpeech";

/** Activation mode for the global hotkey */
export type ActivationMode = "toggle" | "hold";

/** AI text cleanup tier */
export type CleanupTier = "rules" | "localLlm" | "cloudLlm";

/** Cloud LLM provider for Tier 3 cleanup */
export type CloudProvider = "openai" | "anthropic";

/** Whisper model identifier */
export type WhisperModel =
  | "tiny.en"
  | "base.en"
  | "small.en"
  | "small"
  | "medium.en"
  | "medium";

/** GPU backend for Whisper inference */
export type GpuBackend = "cpu" | "cuda" | "vulkan";

/** Real-time audio level data for waveform visualization */
export interface AudioLevel {
  /** RMS level normalized to 0.0–1.0 */
  rms: number;
  /** Timestamp in milliseconds */
  timestamp: number;
}

/** Application settings persisted to disk */
export interface Settings {
  /** Global hotkey combination (e.g., "Ctrl+Shift+Space") */
  hotkey: string;
  /** Activation mode: toggle or push-to-hold */
  activationMode: ActivationMode;
  /** Launch app at system startup */
  launchAtStartup: boolean;
  /** Selected Whisper model */
  whisperModel: WhisperModel;
  /** GPU backend preference */
  gpuBackend: GpuBackend;
  /** Selected input device ID, null for system default */
  inputDeviceId: string | null;
  /** Active cleanup tier */
  cleanupTier: CleanupTier;
  /** Cloud LLM provider (when cleanupTier is "cloudLlm") */
  cloudProvider: CloudProvider;
  /** Custom vocabulary words for improved recognition */
  customVocabulary: string[];
}

/** Audio input device info */
export interface AudioDevice {
  id: string;
  name: string;
  isDefault: boolean;
}

/** Whisper model info for the settings UI */
export interface ModelInfo {
  id: WhisperModel;
  name: string;
  sizeMb: number;
  downloaded: boolean;
}
