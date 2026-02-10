import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ActivationMode, CleanupTier, CloudProvider } from "@/types";

// ---- Backend return types ----

export interface AudioDeviceInfo {
  id: string;
  name: string;
  isDefault: boolean;
}

export interface WhisperModelInfo {
  id: string;
  name: string;
  filename: string;
  sizeBytes: number;
  description: string;
  downloaded: boolean;
}

export interface GpuBackendInfo {
  id: string;
  name: string;
  available: boolean;
}

// ---- Hook state ----

export interface SettingsState {
  // General
  hotkey: string;
  activationMode: ActivationMode;
  autostart: boolean;

  // Transcription
  whisperModels: WhisperModelInfo[];
  currentWhisperModel: string | null;
  gpuBackends: GpuBackendInfo[];
  gpuBackend: string;

  // Cleanup
  cleanupTier: CleanupTier;
  cloudProvider: CloudProvider;
  apiKeyExists: { openai: boolean; anthropic: boolean };

  // Audio
  audioDevices: AudioDeviceInfo[];
  audioDevice: string | null;

  // Loading
  loading: boolean;
}

export interface SettingsActions {
  setHotkey: (hotkey: string) => Promise<void>;
  setActivationMode: (mode: ActivationMode) => Promise<void>;
  setAutostart: (enabled: boolean) => Promise<void>;
  setGpuBackend: (backend: string) => Promise<void>;
  setCleanupTier: (tier: CleanupTier) => Promise<void>;
  setCloudProvider: (provider: CloudProvider) => Promise<void>;
  setAudioDevice: (deviceId: string | null) => Promise<void>;
  saveApiKey: (provider: CloudProvider, key: string) => Promise<void>;
  deleteApiKey: (provider: CloudProvider) => Promise<void>;
  testApiKey: (provider: CloudProvider) => Promise<void>;
  downloadWhisperModel: (modelId: string) => Promise<void>;
  deleteWhisperModel: (modelId: string) => Promise<void>;
  loadWhisperModel: (modelId: string) => Promise<void>;
  refreshAudioDevices: () => Promise<void>;
  refreshWhisperModels: () => Promise<void>;
}

const defaultState: SettingsState = {
  hotkey: "ctrl+shift+space",
  activationMode: "toggle",
  autostart: false,
  whisperModels: [],
  currentWhisperModel: null,
  gpuBackends: [],
  gpuBackend: "cpu",
  cleanupTier: "rules",
  cloudProvider: "openai",
  apiKeyExists: { openai: false, anthropic: false },
  audioDevices: [],
  audioDevice: null,
  loading: true,
};

export function useSettings(): SettingsState & SettingsActions {
  const [state, setState] = useState<SettingsState>(defaultState);

  // ---- Load all settings on mount ----
  useEffect(() => {
    const load = async () => {
      try {
        const [
          hotkey,
          activationMode,
          autostart,
          whisperModels,
          currentWhisperModel,
          gpuBackends,
          gpuBackend,
          cleanupTier,
          cloudProvider,
          openaiKeyExists,
          anthropicKeyExists,
          audioDevices,
          audioDevice,
        ] = await Promise.all([
          invoke<string>("get_hotkey"),
          invoke<string>("get_activation_mode"),
          invoke<boolean>("get_autostart"),
          invoke<WhisperModelInfo[]>("list_whisper_models"),
          invoke<string | null>("get_current_whisper_model"),
          invoke<GpuBackendInfo[]>("get_gpu_backends"),
          invoke<string>("get_gpu_backend"),
          invoke<CleanupTier>("get_cleanup_tier"),
          invoke<CloudProvider>("get_cloud_provider"),
          invoke<boolean>("get_api_key_exists", { provider: "openAi" }),
          invoke<boolean>("get_api_key_exists", { provider: "anthropic" }),
          invoke<AudioDeviceInfo[]>("list_audio_devices"),
          invoke<string | null>("get_audio_device"),
        ]);

        setState({
          hotkey,
          activationMode: activationMode as ActivationMode,
          autostart,
          whisperModels,
          currentWhisperModel,
          gpuBackends,
          gpuBackend,
          cleanupTier,
          cloudProvider,
          apiKeyExists: { openai: openaiKeyExists, anthropic: anthropicKeyExists },
          audioDevices,
          audioDevice,
          loading: false,
        });
      } catch (e) {
        console.error("Failed to load settings:", e);
        setState((prev) => ({ ...prev, loading: false }));
      }
    };
    load();
  }, []);

  // ---- Setters ----

  const setHotkey = useCallback(async (hotkey: string) => {
    await invoke("set_hotkey", { shortcut: hotkey });
    setState((prev) => ({ ...prev, hotkey }));
  }, []);

  const setActivationMode = useCallback(async (mode: ActivationMode) => {
    await invoke("set_activation_mode", { mode });
    setState((prev) => ({ ...prev, activationMode: mode }));
  }, []);

  const setAutostart = useCallback(async (enabled: boolean) => {
    await invoke("set_autostart", { enabled });
    setState((prev) => ({ ...prev, autostart: enabled }));
  }, []);

  const setGpuBackend = useCallback(async (backend: string) => {
    await invoke("set_gpu_backend", { backend });
    setState((prev) => ({ ...prev, gpuBackend: backend }));
  }, []);

  const setCleanupTier = useCallback(async (tier: CleanupTier) => {
    await invoke("set_cleanup_tier", { tier });
    setState((prev) => ({ ...prev, cleanupTier: tier }));
  }, []);

  const setCloudProvider = useCallback(async (provider: CloudProvider) => {
    await invoke("set_cloud_provider", { provider });
    setState((prev) => ({ ...prev, cloudProvider: provider }));
  }, []);

  const setAudioDevice = useCallback(async (deviceId: string | null) => {
    await invoke("set_audio_device", { deviceId });
    setState((prev) => ({ ...prev, audioDevice: deviceId }));
  }, []);

  const saveApiKey = useCallback(async (provider: CloudProvider, key: string) => {
    await invoke("save_api_key", { provider, key });
    setState((prev) => ({
      ...prev,
      apiKeyExists: {
        ...prev.apiKeyExists,
        [provider === "openai" ? "openai" : "anthropic"]: true,
      },
    }));
  }, []);

  const deleteApiKey = useCallback(async (provider: CloudProvider) => {
    await invoke("delete_api_key", { provider });
    setState((prev) => ({
      ...prev,
      apiKeyExists: {
        ...prev.apiKeyExists,
        [provider === "openai" ? "openai" : "anthropic"]: false,
      },
    }));
  }, []);

  const testApiKey = useCallback(async (provider: CloudProvider) => {
    await invoke("test_api_key", { provider });
  }, []);

  const downloadWhisperModel = useCallback(async (modelId: string) => {
    await invoke("download_whisper_model", { modelId });
    // Refresh models list after download
    const models = await invoke<WhisperModelInfo[]>("list_whisper_models");
    setState((prev) => ({ ...prev, whisperModels: models }));
  }, []);

  const deleteWhisperModel = useCallback(async (modelId: string) => {
    await invoke("delete_whisper_model", { modelId });
    const models = await invoke<WhisperModelInfo[]>("list_whisper_models");
    const current = await invoke<string | null>("get_current_whisper_model");
    setState((prev) => ({ ...prev, whisperModels: models, currentWhisperModel: current }));
  }, []);

  const loadWhisperModel = useCallback(async (modelId: string) => {
    await invoke("load_whisper_model", { modelId });
    setState((prev) => ({ ...prev, currentWhisperModel: modelId }));
  }, []);

  const refreshAudioDevices = useCallback(async () => {
    const devices = await invoke<AudioDeviceInfo[]>("list_audio_devices");
    setState((prev) => ({ ...prev, audioDevices: devices }));
  }, []);

  const refreshWhisperModels = useCallback(async () => {
    const models = await invoke<WhisperModelInfo[]>("list_whisper_models");
    const current = await invoke<string | null>("get_current_whisper_model");
    setState((prev) => ({ ...prev, whisperModels: models, currentWhisperModel: current }));
  }, []);

  return {
    ...state,
    setHotkey,
    setActivationMode,
    setAutostart,
    setGpuBackend,
    setCleanupTier,
    setCloudProvider,
    setAudioDevice,
    saveApiKey,
    deleteApiKey,
    testApiKey,
    downloadWhisperModel,
    deleteWhisperModel,
    loadWhisperModel,
    refreshAudioDevices,
    refreshWhisperModels,
  };
}
