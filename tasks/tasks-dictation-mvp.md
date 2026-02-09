# Task List: Dictation MVP

> **Source:** `tasks/prd-dictation-mvp-v1.md`
> **Standards version:** 1.0.0
> **Applied standards:** global/principles.md, phases/generate-tasks.md

## Relevant Files

### Rust Backend (`src-tauri/`)

- `src-tauri/Cargo.toml` - Rust dependencies (whisper-rs, cpal, enigo, ort, arboard, keyring, reqwest, serde)
- `src-tauri/tauri.conf.json` - Tauri app configuration, permissions, windows, tray
- `src-tauri/capabilities/default.json` - Tauri v2 capability permissions
- `src-tauri/src/main.rs` - Tauri app entry point
- `src-tauri/src/lib.rs` - Module declarations and Tauri command registrations
- `src-tauri/src/audio/mod.rs` - Audio module exports
- `src-tauri/src/audio/capture.rs` - Microphone recording via cpal/WASAPI
- `src-tauri/src/audio/capture_test.rs` - Tests for audio capture
- `src-tauri/src/audio/vad.rs` - Silero VAD integration via ort (ONNX Runtime)
- `src-tauri/src/audio/vad_test.rs` - Tests for VAD
- `src-tauri/src/stt/mod.rs` - STT module exports
- `src-tauri/src/stt/whisper.rs` - whisper-rs integration and transcription pipeline
- `src-tauri/src/stt/whisper_test.rs` - Tests for Whisper transcription
- `src-tauri/src/stt/models.rs` - Model download, caching, and management
- `src-tauri/src/stt/models_test.rs` - Tests for model management
- `src-tauri/src/cleanup/mod.rs` - Cleanup module exports and tier abstraction trait
- `src-tauri/src/cleanup/rules.rs` - Rule-based text cleanup (filler removal, capitalization, punctuation)
- `src-tauri/src/cleanup/rules_test.rs` - Tests for rule-based cleanup
- `src-tauri/src/cleanup/local_llm.rs` - Local LLM cleanup via llama.cpp bindings
- `src-tauri/src/cleanup/local_llm_test.rs` - Tests for local LLM cleanup
- `src-tauri/src/cleanup/cloud_llm.rs` - Cloud LLM cleanup (OpenAI, Anthropic)
- `src-tauri/src/cleanup/cloud_llm_test.rs` - Tests for cloud LLM cleanup
- `src-tauri/src/injection/mod.rs` - Text injection module exports
- `src-tauri/src/injection/clipboard.rs` - Clipboard save/restore and paste injection
- `src-tauri/src/injection/clipboard_test.rs` - Tests for clipboard injection
- `src-tauri/src/injection/keyboard.rs` - Keyboard simulation fallback via enigo
- `src-tauri/src/injection/keyboard_test.rs` - Tests for keyboard simulation
- `src-tauri/src/settings/mod.rs` - Settings module exports
- `src-tauri/src/settings/store.rs` - Settings JSON persistence and Tauri commands
- `src-tauri/src/settings/store_test.rs` - Tests for settings persistence
- `src-tauri/src/hotkey/mod.rs` - Global hotkey registration and mode management
- `src-tauri/src/hotkey/mod_test.rs` - Tests for hotkey logic
- `src-tauri/src/tray/mod.rs` - System tray icon, state management, context menu
- `src-tauri/src/pipeline.rs` - End-to-end dictation pipeline orchestrator
- `src-tauri/src/pipeline_test.rs` - Integration tests for full pipeline

### React Frontend (`src/`)

- `package.json` - Frontend dependencies (React, Tailwind, shadcn/ui, Radix, etc.)
- `tsconfig.json` - TypeScript strict mode configuration
- `vite.config.ts` - Vite build configuration for Tauri
- `tailwind.config.ts` - Tailwind config with design token theme (colors, typography, spacing)
- `src/main.tsx` - React entry point
- `src/App.tsx` - Root component with window routing (pill, settings, onboarding)
- `src/index.css` - Tailwind base styles and global CSS
- `src/lib/utils.ts` - cn() utility (clsx + tailwind-merge)
- `src/lib/tauri.ts` - Tauri API wrappers and event type definitions
- `src/types/index.ts` - Shared TypeScript types (settings, pill state, audio level, etc.)
- `src/hooks/useSettings.ts` - Settings state hook (read/write via Tauri commands)
- `src/hooks/useSettings.test.ts` - Tests for settings hook
- `src/hooks/useTauriEvent.ts` - Hook for subscribing to Tauri backend events
- `src/hooks/useAudioLevel.ts` - Hook for real-time audio level data from backend
- `src/components/ui/` - shadcn/ui component directory (button, dropdown, toggle, input, dialog, etc.)
- `src/components/pill/Pill.tsx` - Main floating pill container component
- `src/components/pill/Pill.test.tsx` - Tests for pill component
- `src/components/pill/PillWaveform.tsx` - Animated waveform visualization
- `src/components/pill/PillWaveform.test.tsx` - Tests for waveform
- `src/components/pill/PillGlow.tsx` - Indigo glow effect component
- `src/components/settings/Settings.tsx` - Settings page layout component
- `src/components/settings/Settings.test.tsx` - Tests for settings
- `src/components/settings/GeneralSection.tsx` - General settings (startup, hotkey, mode)
- `src/components/settings/TranscriptionSection.tsx` - Transcription settings (model, language, GPU)
- `src/components/settings/CleanupSection.tsx` - AI cleanup settings (tier, provider, API key)
- `src/components/settings/AudioSection.tsx` - Audio settings (mic selection, level meter)
- `src/components/settings/VocabularySection.tsx` - Custom vocabulary management
- `src/components/settings/HotkeyRecorder.tsx` - Keyboard shortcut recorder widget
- `src/components/settings/HotkeyRecorder.test.tsx` - Tests for hotkey recorder
- `src/components/onboarding/Onboarding.tsx` - Onboarding flow container
- `src/components/onboarding/Onboarding.test.tsx` - Tests for onboarding
- `src/components/onboarding/WelcomeStep.tsx` - Step 1: Welcome + value prop
- `src/components/onboarding/MicrophoneStep.tsx` - Step 2: Mic permission
- `src/components/onboarding/HotkeyStep.tsx` - Step 3: Hotkey setup
- `src/components/onboarding/FirstDictationStep.tsx` - Step 4: Magic moment
- `src/components/onboarding/TrayStep.tsx` - Step 5: Tray location

### Notes

- Rust tests use inline `#[cfg(test)]` modules or separate `_test.rs` files in the same directory.
- Frontend tests use Vitest + React Testing Library. Run with `npx vitest` or `npx vitest run`.
- Run Rust tests with `cd src-tauri && cargo test`.
- Run the dev server with `npx tauri dev`.
- Build the installer with `npx tauri build`.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 Create and checkout a new branch: `git checkout -b feature/dictation-mvp`

- [ ] 1.0 Scaffold Tauri v2 project with React/TypeScript/Tailwind frontend (FR-1, FR-4, FR-26)
  - [ ] 1.1 Initialize a new Tauri v2 project using `npm create tauri-app@latest` with the React + TypeScript + Vite template. Verify the project builds and the dev window opens.
  - [ ] 1.2 Configure TypeScript strict mode in `tsconfig.json` (`"strict": true`) per INT-1.
  - [ ] 1.3 Install and configure Tailwind CSS 4. Set up `tailwind.config.ts` with the design token theme from `/standards/domains/design-ui.md`: background colors (`bg-base` #0A0A0B, `bg-surface` #111113, `bg-elevated` #18181B, etc.), text colors, accent colors, border colors, typography scale, and spacing scale.
  - [ ] 1.4 Install and initialize shadcn/ui. Add core components needed across the app: Button, DropdownMenu, Select, Switch/Toggle, Input, Label, Separator. Configure dark theme as default.
  - [ ] 1.5 Create the `cn()` utility in `src/lib/utils.ts` (clsx + tailwind-merge) per INT-19.
  - [ ] 1.6 Set up the Rust backend directory structure: create module directories for `audio/`, `stt/`, `cleanup/`, `injection/`, `settings/`, `hotkey/`, `tray/`. Add initial `mod.rs` files. Add all required crate dependencies to `Cargo.toml` (whisper-rs, cpal, enigo, ort, arboard, keyring, reqwest, serde, serde_json, tauri-plugin-global-shortcut, tauri-plugin-autostart, tauri-plugin-updater).
  - [ ] 1.7 Configure Tauri v2 permissions and capabilities in `capabilities/default.json`: allow global-shortcut, autostart, clipboard-read, clipboard-write, shell (for URL opening), updater.
  - [ ] 1.8 Configure `tauri.conf.json` with three windows: `main` (hidden, for routing), `pill` (frameless, transparent, always-on-top, skip-taskbar, 280x52), `settings` (titled, 480x640). Set app identifier, product name ("Dictation"), and tray icon.
  - [ ] 1.9 Create shared TypeScript types in `src/types/index.ts`: `PillState` enum (recording, processing, success, error, noSpeech), `Settings` interface, `CleanupTier` enum, `ActivationMode` enum, `AudioLevel` type.
  - [ ] 1.10 Verify the complete scaffolding builds successfully on Windows 11: `npx tauri dev` starts without errors, tray icon appears, pill and settings windows can be shown/hidden.

- [ ] 2.0 Implement system tray application shell and global hotkey registration (FR-1, FR-2, FR-5, FR-6, FR-7)
  - [ ] 2.1 Implement the system tray icon in `src-tauri/src/tray/mod.rs`. Create four icon variants representing states: idle (default), listening (active/recording), processing, error. Register the tray icon on app startup. The tray icon must update dynamically based on app state.
  - [ ] 2.2 Implement the tray context menu with items: "Settings" (opens settings window), separator, "Pause / Resume" (toggles dictation availability), separator, "About Dictation" (shows version info), "Quit" (exits app). Display the configured hotkey next to relevant menu items.
  - [ ] 2.3 Implement global hotkey registration in `src-tauri/src/hotkey/mod.rs` using `tauri-plugin-global-shortcut`. Default hotkey: `Ctrl+Shift+Space`. On hotkey press, emit a Tauri event that the frontend and pipeline can listen to. Handle registration failure (hotkey conflict) by emitting an error event.
  - [ ] 2.4 Implement toggle mode: first hotkey press starts the dictation pipeline (audio capture begins, pill shows recording state). Second hotkey press stops recording and triggers processing.
  - [ ] 2.5 Implement push-to-hold mode: hotkey down starts recording, hotkey up stops recording and triggers processing. Detect hold vs. tap using a 300ms threshold.
  - [ ] 2.6 Implement hotkey conflict detection. On registration failure, notify the frontend to display an error and prompt the user to configure an alternative hotkey in settings.
  - [ ] 2.7 Implement launch-at-startup using `tauri-plugin-autostart`. Expose a Tauri command to enable/disable autostart from the settings UI. Default: enabled.
  - [ ] 2.8 Implement the main app initialization sequence in `src-tauri/src/main.rs`: register tray → register hotkey → preload Whisper model (FR-4) → check for first-run flag → emit "ready" event. Ensure the app is ready to accept dictation within 3 seconds of launch.
  - [ ] 2.9 Write tests for hotkey registration, mode switching (toggle vs. hold), and tray menu actions.

- [ ] 3.0 Build audio capture pipeline with voice activity detection (FR-8, FR-9, FR-10, FR-11)
  - [ ] 3.1 Implement microphone audio capture in `src-tauri/src/audio/capture.rs` using the `cpal` crate with the WASAPI backend. Capture audio at 16kHz mono 16-bit PCM. If the device doesn't support 16kHz natively, capture at the device's native rate and resample to 16kHz using linear interpolation.
  - [ ] 3.2 Implement audio device enumeration: list all available input devices, identify the system default, and expose a Tauri command to get the device list for the settings UI (FR-9).
  - [ ] 3.3 Implement device selection: allow setting a specific input device via a Tauri command. On device disconnect, fall back to the system default and emit a notification event to the frontend.
  - [ ] 3.4 Integrate Silero VAD in `src-tauri/src/audio/vad.rs` using the `ort` crate (ONNX Runtime). Load the Silero VAD ONNX model on startup. Process audio frames through VAD to detect speech presence/absence.
  - [ ] 3.5 Implement speech boundary detection: detect when the user starts speaking (transition from silence to speech) and when they stop (transition from speech to silence, with a 500ms trailing silence buffer to avoid cutting off mid-sentence).
  - [ ] 3.6 Implement silence timeout logic (FR-11): if no speech is detected for 5 seconds after recording starts, emit a "noSpeech" event (pill shows "Waiting for you to speak..."). If no speech detected for 10 seconds, auto-stop recording and emit a "timeout" event.
  - [ ] 3.7 Implement audio level monitoring: compute RMS audio level from captured frames and emit level values to the frontend via Tauri events at ~30fps for waveform visualization (FR-10).
  - [ ] 3.8 Implement microphone error handling (FR-30): detect no-device and permission-denied scenarios. Emit descriptive error events that the pill UI can display.
  - [ ] 3.9 Write tests for the audio capture pipeline, VAD integration, silence timeout logic, and error handling.

- [ ] 4.0 Integrate whisper.cpp STT engine with model management (FR-12, FR-13, FR-14, FR-28)
  - [ ] 4.1 Implement Whisper model loading in `src-tauri/src/stt/whisper.rs` using the `whisper-rs` crate. Load a GGML model file from the app's model cache directory. Expose model loading as a Tauri command that reports success/failure.
  - [ ] 4.2 Implement the model download system in `src-tauri/src/stt/models.rs`. Download model files from Hugging Face (ggerganov/whisper.cpp releases) via `reqwest`. Show download progress via Tauri events (bytes downloaded, total size, percentage). Support models: `tiny.en`, `base.en`, `small.en`, `small`, `medium.en`, `medium`. Cache downloaded models in the Tauri app data directory under a `models/whisper/` subdirectory.
  - [ ] 4.3 Implement model management commands: list available models (with size and description), list downloaded models, delete a model, get model path. Display model sizes in the UI: tiny.en (~75MB), base.en (~150MB), small.en (~500MB), medium.en (~1.5GB).
  - [ ] 4.4 Implement the transcription pipeline: accept an audio buffer (f32 PCM samples at 16kHz), run whisper inference, return the transcribed text. Use whisper-rs full transcription mode with English language setting. Set reasonable parameters: beam_size=5, no_timestamps=true.
  - [ ] 4.5 Implement GPU backend detection and selection (FR-13). Check for CUDA availability (NVIDIA), Vulkan support, or fall back to CPU (AVX2). Expose a Tauri command to get available backends and set the preferred backend. Store the preference in settings.
  - [ ] 4.6 Implement model preloading on app startup (FR-4). During initialization (task 2.8), load the user's selected Whisper model into memory so the first dictation has no model-loading delay. If the model isn't downloaded yet, skip preloading and trigger download on first use or during onboarding.
  - [ ] 4.7 Benchmark transcription latency on reference hardware. Verify `base.en` processes 10 seconds of audio in <3 seconds on CPU and <1.5 seconds on a CUDA-capable GPU (FR-14). Log timing data for future optimization.
  - [ ] 4.8 Write tests for model download, model loading, transcription pipeline, and GPU backend selection.

- [ ] 5.0 Implement three-tier AI text cleanup pipeline (FR-15, FR-16, FR-17, FR-18, FR-29)
  - [ ] 5.1 Define the cleanup pipeline abstraction in `src-tauri/src/cleanup/mod.rs`. Create a `TextCleaner` trait with an `async fn clean(text: &str) -> Result<String>` method. Implement a pipeline orchestrator that selects the appropriate tier based on user settings and falls back on failure (cloud → local LLM → rule-based).
  - [ ] 5.2 Implement Tier 1 rule-based cleanup in `src-tauri/src/cleanup/rules.rs` (FR-15). Remove filler words ("um", "uh", "like", "you know", "so", "basically", "actually") when used as fillers (not as meaningful words). Auto-capitalize sentence starts and the pronoun "I". Normalize whitespace. Ensure Whisper's sentence-boundary periods are preserved. Target: <50ms latency.
  - [ ] 5.3 Write comprehensive tests for rule-based cleanup: filler removal (including edge cases where "like" or "so" are meaningful), capitalization, whitespace normalization, punctuation handling.
  - [ ] 5.4 Implement local LLM model download and management in `src-tauri/src/cleanup/local_llm.rs` (FR-29). Similar to Whisper model management: download GGUF model files, report progress, cache in app data under `models/llm/`. Support at least one small model (e.g., Phi-3-mini-4k-instruct GGUF, ~2GB). Display model size and RAM/VRAM requirements before download.
  - [ ] 5.5 Implement Tier 2 local LLM cleanup (FR-16). Load the local LLM model via llama.cpp Rust bindings (e.g., `llama-cpp-2` or `candle` crate). Send a cleanup prompt: "Clean up the following dictated text. Fix grammar and punctuation. Remove filler words. Do NOT change technical terms, names, or meaning. Do NOT add content. Return only the cleaned text." Target: <5 seconds for 100 words.
  - [ ] 5.6 Implement Tier 3 cloud LLM cleanup in `src-tauri/src/cleanup/cloud_llm.rs` (FR-17). Implement HTTP client calls to OpenAI API (GPT-4o-mini) using `reqwest`. Send only the transcribed text with the cleanup system prompt. Parse the response. Handle errors (invalid key, rate limit, network failure). Target: <2 seconds for 100 words.
  - [ ] 5.7 Add Anthropic Claude Haiku as a second cloud provider option. Implement provider selection in the cleanup config.
  - [ ] 5.8 Implement encrypted API key storage using the `keyring` crate (FR-34). Expose Tauri commands to save and retrieve API keys. Keys are stored in the Windows Credential Manager, never in plain text files.
  - [ ] 5.9 Implement cleanup tier fallback logic (FR-32): if cloud fails, try local LLM; if local LLM fails (not installed, insufficient resources), use rule-based. Emit a Tauri event indicating which tier was actually used so the pill can show a brief fallback notice.
  - [ ] 5.10 Write tests for each cleanup tier, the fallback logic, and API key storage.

- [ ] 6.0 Build text injection system with clipboard and keyboard fallback (FR-19, FR-20, FR-21)
  - [ ] 6.1 Implement clipboard save/restore in `src-tauri/src/injection/clipboard.rs` using the `arboard` crate. Save the current clipboard contents (text and image data), store temporarily in memory. Implement restore after injection with a configurable delay (100-200ms).
  - [ ] 6.2 Implement clipboard-based text injection (FR-19): set cleaned text to clipboard → simulate `Ctrl+V` via `SendInput` (Windows API) using the `enigo` crate → wait 100-200ms → restore original clipboard. Ensure the entire sequence completes within 500ms.
  - [ ] 6.3 Implement keyboard simulation fallback in `src-tauri/src/injection/keyboard.rs` (FR-20). Type text character-by-character via `enigo` crate keystroke simulation. Add a configurable inter-keystroke delay (5-10ms). Use this fallback when clipboard injection fails or for text under 50 characters.
  - [ ] 6.4 Implement injection failure detection: after clipboard paste, briefly check if the clipboard was consumed by the target app. If injection appears to have failed, attempt keyboard simulation fallback.
  - [ ] 6.5 Implement quick-redo hotkey (FR-21). Register a secondary global hotkey (default: `Ctrl+Shift+Z`, configurable). When pressed within 10 seconds of the last injection: simulate `Ctrl+Z` repeated for the character count of the last injection (or select-all-and-delete if undo is unreliable), then immediately start a new recording session.
  - [ ] 6.6 Track the last injection metadata (text content, character count, timestamp) in memory to support quick-redo.
  - [ ] 6.7 Test text injection across the top Windows 11 apps: Notepad, VS Code, Chrome (address bar + web forms + Google Docs), Microsoft Word, Outlook, Slack, Teams, Windows Terminal, Discord, Notion. Document any app-specific quirks or failures.

- [ ] 7.0 Create floating pill UI with state transitions and waveform visualization (FR-22, FR-23, FR-24)
  - [ ] 7.1 Configure the pill Tauri window: frameless, transparent background, always-on-top, skip-taskbar, no decorations, 280x52 default size. Set up window show/hide commands callable from the Rust backend. Ensure the window appears on the active monitor.
  - [ ] 7.2 Implement the pill container component in `src/components/pill/Pill.tsx`. Apply glassmorphism styling: `bg-elevated` (#18181B) at ~90% opacity, `backdrop-blur-md`, 1px `border-subtle` (#27272A) border, 24px border-radius. Pill dimensions: 280px wide, 52px tall.
  - [ ] 7.3 Implement a pill state machine using React state. States: `idle` (hidden), `recording`, `processing`, `success`, `error`, `noSpeech`. Subscribe to Tauri events from the backend to drive state transitions.
  - [ ] 7.4 Implement the recording state UI in `src/components/pill/PillWaveform.tsx`. Render 12-16 thin vertical bars that animate based on real-time audio level data received via Tauri events (from task 3.7). Bars use `accent-primary` (#6366F1) color. When silent, bars settle to a flat line. Animate with CSS transitions (100-150ms ease-out).
  - [ ] 7.5 Implement the indigo glow effect in `src/components/pill/PillGlow.tsx` (FR-24). During recording, render a soft glow around the pill border: `accent-primary` (#6366F1) at 30-40% opacity, blurred 8-12px. Animate with a subtle pulse (opacity 30%→40%→30% over 2 seconds, ease-in-out).
  - [ ] 7.6 Implement the processing state: replace waveform with a spinner animation + "Processing..." text in `text-secondary` (#A1A1AA).
  - [ ] 7.7 Implement the success state: checkmark icon + "Done" text in `accent-success` (#22C55E). Auto-dismiss the pill after 1.5 seconds with a fade-out animation (200ms, ease-out).
  - [ ] 7.8 Implement the error state: error icon + brief message text in `accent-error` (#EF4444). Persist until user presses the hotkey again or clicks the pill to dismiss.
  - [ ] 7.9 Implement the "no speech" state: flatlined waveform + "Waiting for you..." text in `accent-warning` (#F59E0B) with a gentle pulse.
  - [ ] 7.10 Implement pill positioning: default position top-center of active monitor, 60px from top. Make the pill draggable (mouse down on pill → track mouse movement → update window position). Persist position to settings so it's remembered across sessions.
  - [ ] 7.11 Implement `prefers-reduced-motion` support: disable waveform animation, glow pulse, and non-essential transitions when the OS accessibility setting is enabled.
  - [ ] 7.12 Write tests for the pill state machine, state transitions, and auto-dismiss timing.

- [ ] 8.0 Build settings UI window (FR-25, FR-26, FR-33, FR-34, FR-35, FR-36)
  - [ ] 8.1 Implement the settings data model and JSON persistence in `src-tauri/src/settings/store.rs`. Define a `Settings` struct with all configurable fields (hotkey, activation mode, launch at startup, whisper model, language, GPU backend, cleanup tier, cloud provider, pill position, custom vocabulary list). Serialize/deserialize to `settings.json` in the Tauri app data directory. Expose Tauri commands: `get_settings`, `update_settings`, `reset_settings`.
  - [ ] 8.2 Create the settings window layout in `src/components/settings/Settings.tsx`. Single-page scrollable layout, 480px wide. Dark background (`bg-surface` #111113). Section headers in `text-primary` (#FAFAFA), descriptions in `text-secondary` (#A1A1AA). Sections separated by `border-subtle` (#27272A) dividers. No "Save" button -- all changes apply immediately via Tauri commands.
  - [ ] 8.3 Build General settings section in `src/components/settings/GeneralSection.tsx`: "Launch at startup" toggle (shadcn Switch), activation hotkey with the HotkeyRecorder widget, activation mode dropdown (Toggle / Push-to-hold).
  - [ ] 8.4 Implement the hotkey recorder widget in `src/components/settings/HotkeyRecorder.tsx`. Click the field to enter recording mode → display "Press a shortcut..." → capture the next key combination → validate (must include a modifier) → display the new shortcut → save. Show the current hotkey when not recording.
  - [ ] 8.5 Build Transcription settings section in `src/components/settings/TranscriptionSection.tsx`: Whisper model dropdown (show model name, size, and speed/accuracy indicator for each), language dropdown (English default, add common languages), GPU acceleration dropdown (Auto / GPU / CPU).
  - [ ] 8.6 Build AI Cleanup settings section in `src/components/settings/CleanupSection.tsx`: cleanup tier selector (Basic / Enhanced / Maximum) with descriptions for each tier. Show availability status (Enhanced grayed out if no local LLM model downloaded, Maximum grayed out if no API key). Cloud provider dropdown (OpenAI / Anthropic). API key password input with a "Test" button to verify the key.
  - [ ] 8.7 Build Audio settings section in `src/components/settings/AudioSection.tsx`: microphone selection dropdown (populated from device enumeration in task 3.2), real-time audio level meter visualization (horizontal bar that responds to mic input).
  - [ ] 8.8 Implement custom vocabulary management in `src/components/settings/VocabularySection.tsx` (FR-36). Display the vocabulary list with add/remove buttons. Text input to add new terms. Import from file (plain text, one term per line). Export to file. Persist the list in settings.
  - [ ] 8.9 Implement the `useSettings` React hook in `src/hooks/useSettings.ts`. Load settings on mount via Tauri command, provide setter functions that call the backend and update local state. Handle optimistic updates.
  - [ ] 8.10 Write tests for settings persistence (Rust), settings UI components (React), and the hotkey recorder widget.

- [ ] 9.0 Implement onboarding flow (FR-27)
  - [ ] 9.1 Create the onboarding container in `src/components/onboarding/Onboarding.tsx`. Implement a step-based flow with a progress indicator (5 dots). Track current step in state. Persist a "onboarding_completed" flag in settings so onboarding only shows once.
  - [ ] 9.2 Implement Step 1 (Welcome) in `src/components/onboarding/WelcomeStep.tsx`: app name/logo, headline "Dictate anywhere on your computer", subtext "3x faster than typing. Private by default.", single "Get Started" button using `accent-primary`.
  - [ ] 9.3 Implement Step 2 (Microphone) in `src/components/onboarding/MicrophoneStep.tsx`: trigger the Windows microphone permission dialog via a Tauri command that attempts audio capture. Display privacy statement: "Your audio is processed on this device. We never store recordings or send audio to the cloud." Block the "Next" button until permission is granted. Show a "Permission denied" state with instructions to enable in Windows Settings > Privacy > Microphone.
  - [ ] 9.4 Implement Step 3 (Hotkey) in `src/components/onboarding/HotkeyStep.tsx`: display the default hotkey (`Ctrl+Shift+Space`) prominently in large text. Include the HotkeyRecorder widget to customize. Display "Try pressing it now!" prompt. Detect when the user presses the hotkey and show a success checkmark.
  - [ ] 9.5 Implement Step 4 (First Dictation) in `src/components/onboarding/FirstDictationStep.tsx`: prompt "Press your hotkey and say something!" Trigger the full dictation pipeline when the user presses the hotkey. Show the floating pill with live waveform. After successful transcription, display the result text in the onboarding window with a subtle celebration animation (confetti or glow). If the `base.en` model is not downloaded, trigger download with a progress bar before the dictation prompt.
  - [ ] 9.6 Implement Step 5 (Tray) in `src/components/onboarding/TrayStep.tsx`: show an illustration or screenshot of the system tray area with the Dictation icon highlighted. Text: "We'll be here whenever you need us. Press [hotkey] anywhere to dictate." "Finish" button that closes onboarding and sets the completion flag.
  - [ ] 9.7 Implement skip logic: show a "Skip" link after Step 2 is complete (microphone permission granted). Skipping jumps to the tray step and sets the completion flag.
  - [ ] 9.8 On app startup, check the onboarding completion flag. If not set, show the onboarding window instead of going straight to tray-only mode.
  - [ ] 9.9 Write tests for onboarding step navigation, skip logic, and completion flag persistence.

- [ ] 10.0 End-to-end integration, error handling, testing, and release packaging (FR-3, FR-30, FR-31, FR-32)
  - [ ] 10.1 Implement the end-to-end dictation pipeline orchestrator in `src-tauri/src/pipeline.rs`. Wire together: hotkey event → show pill (recording) → start audio capture → VAD monitors speech → hotkey event (stop) or auto-stop → pill (processing) → run Whisper STT → run cleanup tier → inject text → pill (success) → auto-dismiss. Handle every error branch (no mic, no model, STT failure, cleanup failure, injection failure) and route to the appropriate pill error state.
  - [ ] 10.2 Implement model load failure handling (FR-31): if Whisper model fails to load on startup (corrupted, insufficient memory), show a notification suggesting re-download or a smaller model. Set the tray icon to error state. Allow the user to open settings to fix the issue.
  - [ ] 10.3 Implement cloud API error handling with fallback (FR-32): on cloud LLM failure, automatically fall back to local LLM or rule-based cleanup. Emit a brief notification via the pill ("Cloud unavailable, used local cleanup"). Never lose the user's transcription due to a cleanup failure.
  - [ ] 10.4 Implement auto-update via `tauri-plugin-updater` (FR-3). Check for updates on app launch (non-blocking). If an update is available, show a notification via the tray menu with "Update available" and a menu item to install. The user must be able to dismiss and update later.
  - [ ] 10.5 Create app icons: system tray icons for all four states (idle, listening, processing, error) as .ico files at required sizes (16x16, 32x32, 48x48). Create the installer icon (256x256). Create the about dialog icon.
  - [ ] 10.6 Configure the Windows installer in `tauri.conf.json`: NSIS installer, set app name, version, publisher, install directory, desktop shortcut option, start menu entry. Target installer size <10MB (models downloaded separately).
  - [ ] 10.7 Implement no-telemetry compliance (FR-35): audit the codebase to ensure no analytics, crash reporting, or telemetry is sent. Add a comment in `main.rs` documenting this decision for future developers.
  - [ ] 10.8 Performance validation: measure and verify hotkey response <100ms (pill appearance), cold start <3s (model loaded, hotkey registered), idle memory <80MB (via Windows Task Manager), installer size <10MB. Document results and optimize if targets are missed.
  - [ ] 10.9 Write integration tests for the three critical user journeys: (1) full dictation flow (hotkey → speak → text appears), (2) quick redo flow (hotkey → speak → redo hotkey → speak again → corrected text appears), (3) cleanup tier fallback (simulate cloud failure → verify fallback to rule-based).
  - [ ] 10.10 Manual QA on a clean Windows 11 installation: install from the built NSIS installer, complete onboarding, test dictation in Notepad, VS Code, Chrome, Word, Slack. Verify tray icon states, pill animations, settings persistence, and startup behavior.
