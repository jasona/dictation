# Product Requirements Document (PRD): Dictation MVP v1

> **Working title:** "Dictation" (final name TBD)
> **Based on:** `tasks/rsd-voice-dictation-app-v1.md`, `tasks/rsd-dictation-app-v1.md`, `tasks/rsd-desktop-framework-v1.md`, `tasks/rsd-dictation-ui-ux-v1.md`

---

## 1. Overview

Dictation is a desktop voice-to-text application for Windows 11 that lets users speak and have polished, formatted text appear wherever their cursor is. It competes with WisprFlow (macOS-only) by bringing AI-enhanced dictation to the Windows market, where no premium solution exists.

The app runs as a lightweight system tray utility built with Tauri v2 (Rust backend, React/TypeScript/Tailwind frontend). Speech-to-text is powered by whisper.cpp running locally, with optional cloud enhancement. An AI text cleanup pipeline transforms raw transcription into clean, well-formatted written text across three tiers: rule-based, local LLM, and cloud LLM.

The product is paid from day one, targeting knowledge workers, developers, and accessibility users who want fast, accurate, private dictation on Windows.

---

## 2. Problem Statement

Windows users who want to dictate text have three options, all inadequate:

1. **Windows Voice Typing (Win+H):** Cloud-dependent, mediocre accuracy, no AI text cleanup, no customization.
2. **Dragon NaturallySpeaking:** Being sunset by Microsoft, expensive ($200-500), legacy UI, no AI cleanup.
3. **Open-source Whisper tools:** Require developer setup, no system-wide integration, no AI cleanup, no polish.

Meanwhile, macOS users have WisprFlow and Superwhisper -- polished, AI-enhanced dictation apps that produce clean written text from natural speech. No equivalent exists on Windows.

---

## 3. Goals

| ID | Goal | Metric |
|----|------|--------|
| G-1 | Deliver a WisprFlow-quality dictation experience on Windows 11 | Users can activate dictation via hotkey and have AI-cleaned text appear in any text field within 3 seconds of finishing speech |
| G-2 | Achieve transcription accuracy that makes corrections infrequent | >95% word accuracy on the base.en Whisper model in quiet environments |
| G-3 | Make the app feel invisible and premium | Hotkey response <100ms, cold start <3 seconds, idle memory <80MB |
| G-4 | Support three tiers of AI text cleanup | Users can choose rule-based (instant), local LLM (2-5s), or cloud LLM (<2s) cleanup |
| G-5 | Generate revenue from day one | Paid product with pricing that attracts early adopters |

---

## 4. User Stories

### US-1: Basic Dictation
As a knowledge worker, I want to press a keyboard shortcut, speak naturally, and have clean text appear in my email/document/chat so that I can compose text 3x faster than typing.

### US-2: Privacy-Conscious Dictation
As a privacy-conscious professional, I want all speech processing to happen on my device so that my voice data never leaves my computer.

### US-3: AI-Enhanced Output
As a user who dictates stream-of-consciousness, I want AI to clean up my grammar, remove filler words, and format my text so that the output reads like polished writing, not raw speech.

### US-4: Quick Correction
As a user who occasionally gets a bad transcription, I want to press a hotkey to undo the last dictation and re-record so that I can correct mistakes without touching my mouse.

### US-5: Accessibility
As a user with RSI, I want dictation to be my primary text input method so that I can reduce strain from extended typing. The app must be fully keyboard-navigable and reliable enough to depend on.

### US-6: Customization
As a developer who uses domain-specific terminology, I want to add custom words to the vocabulary so that technical terms are transcribed correctly.

---

## 5. Non-Goals (Out of Scope for MVP)

| ID | Non-Goal | Rationale |
|----|----------|-----------|
| NG-1 | macOS or Linux support | Windows-first strategy; cross-platform comes in v2/v3 |
| NG-2 | Voice commands ("delete that", "select last sentence") | Adds significant complexity; defer to post-MVP |
| NG-3 | Always-on / wake-word activation | Privacy and complexity concerns; hotkey-only for MVP |
| NG-4 | Real-time streaming text injection | MVP uses batch delivery (transcribe after speech ends); streaming preview in the pill only |
| NG-5 | Context-aware formatting by target app | Requires per-app detection logic; defer to post-MVP |
| NG-6 | Multi-language simultaneous dictation | Single language at a time for MVP; language selectable in settings |
| NG-7 | Plugin/extension system | Defer to v2+ |
| NG-8 | Mobile companion app | Desktop-only |
| NG-9 | Meeting transcription / long-form recording | This is a dictation tool, not a transcription service |
| NG-10 | Light mode theme | Dark mode only for MVP per design standards (DESIGN-1) |

---

## 6. Functional Requirements

### 6.1 Application Shell

**FR-1: System Tray Application** (Must Have)
The app MUST run as a Windows 11 system tray application with no main window. The system tray icon MUST indicate app state: idle, listening, processing, error. Right-clicking the tray icon MUST show a context menu with: Settings, Pause/Resume, About, Quit.
*Traces to: US-1, G-3*

**FR-2: Launch at Startup** (Must Have)
The app MUST support optional launch at Windows startup via a setting. Default: enabled. Uses `tauri-plugin-autostart`.
*Traces to: US-1, US-5*

**FR-3: Auto-Update** (Should Have)
The app SHOULD check for updates on launch and notify the user when an update is available. Uses `tauri-plugin-updater`. The user MUST be able to dismiss the notification and update later.
*Traces to: G-3*

**FR-4: Cold Start Performance** (Must Have)
The app MUST be ready to accept dictation within 3 seconds of launch. The Whisper model MUST be preloaded into memory on startup (not loaded on first dictation).
*Traces to: G-3*

### 6.2 Dictation Activation

**FR-5: Global Hotkey (Toggle Mode)** (Must Have)
The app MUST register a global hotkey (default: `Ctrl+Shift+Space`) that toggles dictation on/off. Press once to start recording, press again to stop and process. The hotkey MUST work when any application is focused, including fullscreen apps. The hotkey MUST be configurable in settings. If the hotkey conflicts with another application, the app MUST show an error and prompt the user to choose a different hotkey.
*Traces to: US-1, US-5, G-3*

**FR-6: Global Hotkey (Push-to-Hold Mode)** (Must Have)
The app MUST support an alternative push-to-hold mode: hold the hotkey to record, release to stop and process. The user MUST be able to choose between toggle and push-to-hold modes in settings. Default: toggle.
*Traces to: US-1*

**FR-7: Hotkey Response Time** (Must Have)
The floating pill MUST appear within 100ms of the hotkey being pressed. Audio recording MUST begin within 200ms of the hotkey being pressed.
*Traces to: G-3*

### 6.3 Audio Capture

**FR-8: Microphone Recording** (Must Have)
The app MUST capture audio from the system's default input device using WASAPI (Windows Audio Session API) via the `cpal` Rust crate. Audio MUST be captured at 16kHz mono (Whisper's expected input format), or resampled to 16kHz if the device does not support it natively.
*Traces to: US-1*

**FR-9: Microphone Selection** (Should Have)
The app SHOULD allow the user to select a specific microphone input device in settings. The app SHOULD detect when the selected device is disconnected and fall back to the system default, displaying a notification.
*Traces to: US-1, US-5*

**FR-10: Audio Level Indicator** (Should Have)
The floating pill SHOULD display a waveform visualization that responds to actual audio input levels during recording. This gives the user confidence that their microphone is working.
*Traces to: US-1, US-5*

**FR-11: Silence Detection** (Must Have)
The app MUST use Silero VAD (voice activity detection, via ONNX Runtime) to detect speech boundaries. If no speech is detected for 5 seconds after recording starts, the pill MUST display "Waiting for you to speak..." with a flatlined waveform. If no speech is detected for 10 seconds, the app MUST auto-stop recording and dismiss the pill with a "No speech detected" message.
*Traces to: US-1, US-5*

### 6.4 Speech-to-Text

**FR-12: Local STT via whisper.cpp** (Must Have)
The app MUST perform speech-to-text using whisper.cpp (integrated via the `whisper-rs` Rust crate). The default model MUST be `base.en` (English-only, ~150MB). The app MUST support models: `tiny.en`, `base.en`, `small.en`, `small`, `medium`, and `medium.en`. The user MUST be able to select the model in settings. Model files MUST be downloaded on first use (not bundled in the installer) with a progress indicator.
*Traces to: US-1, US-2, G-2*

**FR-13: GPU Acceleration** (Should Have)
The app SHOULD detect available GPU hardware and use it for Whisper inference when possible. Support CUDA (NVIDIA) and Vulkan as backends. Fall back to CPU (AVX2) when no compatible GPU is available. The user SHOULD be able to override the backend selection in settings (Auto, GPU, CPU).
*Traces to: G-2, G-3*

**FR-14: Transcription Latency** (Must Have)
Using the `base.en` model, transcription of a 10-second audio clip MUST complete within 3 seconds on CPU (modern x86 with AVX2) and within 1.5 seconds on GPU (NVIDIA GTX 1060 or equivalent).
*Traces to: G-1, G-3*

### 6.5 AI Text Cleanup

**FR-15: Tier 1 -- Rule-Based Cleanup** (Must Have)
The app MUST apply rule-based text cleanup to all transcriptions by default:
- Remove filler words ("um", "uh", "like", "you know", "so", "basically", "actually") when used as fillers.
- Auto-capitalize the first word of each sentence.
- Auto-capitalize "I" when used as a pronoun.
- Insert periods at sentence boundaries detected by Whisper.
- Normalize whitespace (remove double spaces, trim leading/trailing).
Rule-based cleanup MUST add no perceptible latency (<50ms).
*Traces to: US-3, G-4*

**FR-16: Tier 2 -- Local LLM Cleanup** (Must Have)
The app MUST support local LLM-based text cleanup as an optional tier. The local LLM (e.g., Phi-3-mini, Llama 3.2 1B/3B, or similar small model) runs on-device via a local inference engine (llama.cpp via Rust bindings or ONNX Runtime). The LLM MUST:
- Fix grammar and punctuation beyond what rule-based cleanup handles.
- Restructure awkward phrasing while preserving the user's meaning.
- NOT change technical terms, proper nouns, or domain-specific vocabulary.
- NOT add content the user did not say.
Local LLM cleanup MUST complete within 5 seconds for a 100-word passage on supported hardware. The user MUST be able to select their cleanup tier in settings (Rule-based, Local LLM, Cloud LLM).
*Traces to: US-3, G-4*

**FR-17: Tier 3 -- Cloud LLM Cleanup** (Must Have)
The app MUST support cloud LLM-based text cleanup as the highest quality tier. The app MUST support at minimum one cloud LLM provider (OpenAI GPT-4o-mini or Anthropic Claude Haiku). The user MUST provide their own API key in settings. Cloud LLM cleanup MUST complete within 2 seconds for a 100-word passage on a standard internet connection. The cloud LLM prompt MUST instruct the model to clean up dictated text without changing meaning, and MUST NOT send any context beyond the transcribed text (no system information, no app state, no user identity).
*Traces to: US-3, G-4*

**FR-18: Cleanup Tier Selection** (Must Have)
The user MUST be able to select their default cleanup tier in settings: "Basic" (rule-based only), "Enhanced" (local LLM), or "Maximum" (cloud LLM). The app MUST clearly indicate in settings which tiers are available based on the user's hardware (local LLM requires sufficient RAM/VRAM) and configuration (cloud LLM requires API key).
*Traces to: US-3, G-4*

### 6.6 Text Delivery

**FR-19: Clipboard-Based Text Injection** (Must Have)
After transcription and cleanup, the app MUST inject text into the currently focused text field using clipboard injection:
1. Save the current clipboard contents (text and format metadata).
2. Copy the processed text to the clipboard.
3. Simulate `Ctrl+V` keystroke via the Windows `SendInput` API (using the `enigo` crate).
4. After a brief delay (100-200ms), restore the original clipboard contents.
The entire injection sequence MUST complete within 500ms.
*Traces to: US-1, G-1*

**FR-20: Keyboard Simulation Fallback** (Should Have)
If clipboard injection fails (detected by monitoring clipboard state), the app SHOULD fall back to simulating individual keystrokes via `SendInput`. This fallback is slower but works in applications that intercept clipboard paste. The user SHOULD be able to force keyboard simulation mode in settings.
*Traces to: US-1, US-5*

**FR-21: Quick Redo** (Must Have)
The app MUST support a quick-redo hotkey (default: `Ctrl+Shift+Z` or configurable). When pressed within 10 seconds of the last text injection, it MUST:
1. Simulate `Ctrl+Z` enough times to undo the injected text (or select and delete the injected character count).
2. Immediately start a new recording session.
This allows the user to correct bad transcriptions without using the mouse.
*Traces to: US-4, US-5*

### 6.7 Floating Pill UI

**FR-22: Floating Pill Window** (Must Have)
During dictation, the app MUST display a floating pill indicator:
- Size: 280px wide, 52px tall (approximately).
- Position: top-center of the active monitor, 60px from the top edge. User-draggable; position persists across sessions.
- Always-on-top, no taskbar entry, no title bar, no window chrome.
- Background: `bg-elevated` (#18181B) with backdrop blur (glassmorphism). 1px `border-subtle` (#27272A) border. Rounded corners (24px radius).
- The pill MUST appear within 100ms of hotkey press (FR-7).
*Traces to: US-1, G-3*

**FR-23: Pill State Transitions** (Must Have)
The pill MUST visually indicate these states:

| State | Visual | Color |
|-------|--------|-------|
| **Recording** | Animated waveform bars + "Listening..." | `accent-primary` (#6366F1) with soft pulsing glow |
| **Processing** | Spinner + "Processing..." | `text-secondary` (#A1A1AA) |
| **Success** | Checkmark + "Done" | `accent-success` (#22C55E) |
| **Error** | Error icon + brief message | `accent-error` (#EF4444) |
| **No speech** | Flat waveform + "Waiting for you..." | `accent-warning` (#F59E0B) |

The pill MUST auto-dismiss 1.5 seconds after showing the success state. Error state MUST persist until the user presses the hotkey again or clicks the pill to dismiss.
*Traces to: US-1, US-5*

**FR-24: Recording Glow Effect** (Must Have)
During the recording state, the pill MUST display a soft indigo glow effect around its border (`accent-primary` #6366F1 at 30-40% opacity, blurred 8-12px). This is the product's signature visual element.
*Traces to: G-3*

### 6.8 Settings

**FR-25: Settings Window** (Must Have)
The app MUST provide a settings window accessible from the system tray context menu and via a keyboard shortcut (default: `Ctrl+Shift+,`). The settings window MUST be a single-page layout (not tabbed) with the following groups:

**General:**
- Launch at startup (toggle, default: on)
- Activation hotkey (shortcut recorder widget)
- Activation mode (toggle vs. push-to-hold)

**Transcription:**
- Whisper model selection (dropdown: tiny, base, small, medium; with size/speed/accuracy indicators)
- Language (dropdown, default: English)
- GPU acceleration (Auto / GPU / CPU)

**AI Cleanup:**
- Cleanup tier (Basic / Enhanced / Maximum)
- Cloud LLM provider (dropdown)
- Cloud LLM API key (password input)

**Audio:**
- Microphone selection (dropdown of available devices)
- Audio level meter (visual indicator, not a setting)

All settings changes MUST apply immediately with no "Save" button. Settings MUST persist across app restarts (stored in the Tauri app data directory as JSON).
*Traces to: US-6*

**FR-26: Settings Window Design** (Must Have)
The settings window MUST follow the design token system:
- Background: `bg-surface` (#111113).
- Text: `text-primary` (#FAFAFA) for labels, `text-secondary` (#A1A1AA) for descriptions.
- Inputs: `bg-elevated` (#18181B), borderless, `ring-accent-primary` on focus.
- Toggles: `accent-primary` (#6366F1) when active.
- Dark mode only. Width: 480px. Resizable height.
*Traces to: G-3*

### 6.9 Onboarding

**FR-27: First-Run Onboarding** (Must Have)
On first launch, the app MUST display a 5-step onboarding flow:

1. **Welcome:** Product name + value proposition ("Dictate anywhere on your computer. 3x faster than typing.") + single "Get Started" button.
2. **Microphone Permission:** Trigger the Windows microphone permission dialog. Display: "Your audio is processed on this device. We never store recordings or send audio to the cloud." Include a "Learn More" link to a privacy explanation. Block progression until permission is granted.
3. **Hotkey Setup:** Display the default hotkey (`Ctrl+Shift+Space`) prominently. Allow the user to record a custom hotkey. "Try pressing it now!" prompt.
4. **First Dictation:** Prompt the user to press their hotkey and say something. Show the floating pill with live waveform. After successful transcription, display the result with a subtle celebration animation. "You just dictated your first text!"
5. **System Tray:** Show where the tray icon lives. "We'll be here whenever you need us. Press [hotkey] anywhere to dictate." + "Finish" button.

The onboarding MUST be completable in under 60 seconds. The onboarding MUST be skippable after step 2 (microphone permission). The onboarding MUST only appear once (persisted flag).
*Traces to: US-1, G-1*

### 6.10 Model Management

**FR-28: Model Download on First Use** (Must Have)
Whisper models MUST NOT be bundled in the installer. On first launch (or when the user selects a new model), the app MUST download the model from a CDN or GitHub releases. A progress bar MUST be displayed. The `base.en` model MUST be downloaded during onboarding (step 4) if not already present. Models MUST be cached in the Tauri app data directory.
*Traces to: G-3*

**FR-29: LLM Model Management** (Must Have)
For the local LLM cleanup tier, the app MUST manage LLM model downloads similarly to Whisper models. The user MUST be able to select from available local LLM models in settings. The app MUST display model size and estimated VRAM/RAM requirements before download.
*Traces to: US-3, G-4*

### 6.11 Error Handling

**FR-30: Microphone Errors** (Must Have)
If no microphone is detected or the microphone permission is denied, the app MUST display a clear error in the pill ("No microphone found" or "Microphone access denied") with guidance on how to resolve (link to Windows sound settings or privacy settings).
*Traces to: US-5*

**FR-31: Model Load Failure** (Must Have)
If the Whisper model fails to load (corrupted file, insufficient memory), the app MUST display an error notification, suggest re-downloading the model or selecting a smaller model, and remain functional (tray icon shows error state).
*Traces to: US-5*

**FR-32: Cloud API Errors** (Must Have)
If cloud LLM cleanup fails (invalid API key, network error, rate limit), the app MUST fall back to the next available cleanup tier (local LLM, then rule-based). The pill MUST briefly indicate the fallback ("Cloud unavailable, using local cleanup"). The user MUST NOT lose their transcription due to a cleanup failure.
*Traces to: US-3, US-5*

### 6.12 Privacy and Security

**FR-33: Local-First Architecture** (Must Have)
All audio capture and Whisper STT processing MUST happen on-device. Audio MUST never be sent to any server. Audio MUST NOT be persisted to disk after transcription (process in-memory only).
*Traces to: US-2, SEC-1, SEC-2*

**FR-34: Cloud LLM Data Minimization** (Must Have)
When cloud LLM cleanup is used, the app MUST send only the transcribed text (not audio). The app MUST NOT send any user metadata, system information, or application context to the cloud LLM API. The request MUST use HTTPS (SEC-6). The API key MUST be stored encrypted in the app data directory (SEC-1).
*Traces to: US-2, SEC-1, SEC-2, SEC-6*

**FR-35: No Telemetry Without Consent** (Must Have)
The app MUST NOT collect or transmit any usage analytics, crash reports, or telemetry without explicit user opt-in. If telemetry is added in the future, it MUST be opt-in with a clear explanation of what is collected.
*Traces to: US-2, SEC-2*

### 6.13 Custom Vocabulary

**FR-36: Custom Word List** (Should Have)
The user SHOULD be able to add custom words/phrases to a vocabulary list in settings. Custom words are used to bias Whisper's output (via the initial prompt or suppress tokens feature in whisper.cpp). The list MUST persist across sessions. Import/export as a plain text file (one term per line).
*Traces to: US-6*

---

## 7. Design Considerations

### Visual Design
The app MUST follow the design token system defined in `/standards/domains/design-ui.md`:
- **Dark mode only** (DESIGN-1). Background hierarchy: `bg-base` (#0A0A0B), `bg-surface` (#111113), `bg-elevated` (#18181B).
- **System font stack** (DESIGN-5): Inter, SF Pro, or system-ui.
- **Muted color palette** with indigo accent (#6366F1) reserved for primary actions and the recording glow (DESIGN-3).
- **Generous whitespace** (DESIGN-4).
- **Keyboard shortcuts displayed** in menus and tooltips (DESIGN-9).
- **Smooth animations** at 150-300ms with ease-out curves (DESIGN-13). Respect `prefers-reduced-motion`.

### Key UI Surfaces
1. **Floating pill** -- the primary UI during dictation (see FR-22 through FR-24).
2. **Settings window** -- single-page layout, 480px wide (see FR-25, FR-26).
3. **Onboarding flow** -- 5-step sequence (see FR-27).
4. **System tray icon + context menu** -- minimal, state-indicating (see FR-1).

### Detailed Design Reference
See `tasks/rsd-dictation-ui-ux-v1.md` for comprehensive UX patterns, pill state specifications, component mockups, and design inspiration analysis.

---

## 8. Technical Considerations

### Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Tauri v2 App                        │
│                                                     │
│  ┌──────────────────┐  ┌────────────────────────┐   │
│  │  React Frontend   │  │     Rust Backend       │   │
│  │  (WebView2)       │  │                        │   │
│  │                   │  │  ┌──────────────────┐  │   │
│  │  - Floating Pill  │  │  │ Audio Capture    │  │   │
│  │  - Settings       │◄─┤  │ (cpal + WASAPI)  │  │   │
│  │  - Onboarding     │  │  └──────────────────┘  │   │
│  │                   │  │  ┌──────────────────┐  │   │
│  │  React + TS +     │  │  │ STT Engine       │  │   │
│  │  Tailwind +       │  │  │ (whisper-rs)     │  │   │
│  │  shadcn/ui        │  │  └──────────────────┘  │   │
│  │                   │  │  ┌──────────────────┐  │   │
│  └──────────────────┘  │  │ Text Cleanup      │  │   │
│                         │  │ (rule + LLM)     │  │   │
│                         │  └──────────────────┘  │   │
│                         │  ┌──────────────────┐  │   │
│                         │  │ Text Injection    │  │   │
│                         │  │ (enigo + SendInput│  │   │
│                         │  └──────────────────┘  │   │
│                         │  ┌──────────────────┐  │   │
│                         │  │ VAD (Silero ONNX) │  │   │
│                         │  └──────────────────┘  │   │
│                         └────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### Key Rust Crates
| Crate | Purpose |
|-------|---------|
| `whisper-rs` | whisper.cpp bindings for STT |
| `cpal` | Cross-platform audio capture |
| `enigo` | Keyboard simulation for text injection |
| `ort` (ONNX Runtime) | Silero VAD inference |
| `tauri-plugin-global-shortcut` | Global hotkey registration |
| `tauri-plugin-autostart` | Launch on boot |
| `tauri-plugin-updater` | Auto-update |
| `arboard` | Clipboard read/write |
| `reqwest` | HTTP client for cloud LLM API calls and model downloads |
| `serde` / `serde_json` | Settings serialization |
| `keyring` | Encrypted API key storage |

### Frontend Stack
- React 19 + TypeScript (strict mode)
- Tailwind CSS 4
- shadcn/ui + Radix UI for settings UI components
- Tauri v2 JS API for frontend-backend communication

### Platform Requirements
- **OS:** Windows 11 only (build 22000+)
- **Runtime:** WebView2 (pre-installed on Windows 11)
- **CPU:** x86_64 with AVX2 support (for whisper.cpp CPU inference)
- **RAM:** 4GB minimum (8GB recommended for local LLM)
- **GPU (optional):** NVIDIA (CUDA 11.7+) or Vulkan-compatible for GPU acceleration
- **Disk:** ~500MB for app + base.en model + local LLM model
- **Network:** Required only for cloud LLM cleanup, model downloads, and auto-update checks

### Installer and Distribution
- **Installer:** NSIS or WiX via Tauri's built-in bundler. MSI format for enterprise compatibility.
- **Distribution:** Direct download from website. WinGet package for developers.
- **Size target:** <10MB installer (models downloaded separately).

---

## 9. Assumptions

| ID | Assumption | Impact if Wrong |
|----|-----------|----------------|
| A-1 | whisper.cpp `base.en` model provides >95% word accuracy in quiet environments on modern hardware | Would need to default to a larger model (more RAM, slower) or add cloud STT fallback to MVP |
| A-2 | Tauri v2 on Windows 11 is stable enough for production use | Would need to switch to Electron, delaying launch |
| A-3 | Clipboard injection works reliably in the top 20 Windows apps (VS Code, Chrome, Outlook, Slack, Word, Notepad, Terminal, Teams, etc.) | Would need keyboard simulation as primary method, which is slower and less reliable |
| A-4 | A small local LLM (1-3B parameters) can run alongside Whisper without excessive resource contention on a machine with 8GB RAM | Would need to make local LLM tier GPU-only or raise minimum RAM requirement |
| A-5 | Users will pay for a dictation tool on Windows | Would need to pivot to freemium or ad-supported model |
| A-6 | `enigo` crate provides reliable keyboard simulation on Windows 11 for text injection | Would need to use raw Win32 `SendInput` API directly |

---

## 10. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Dictation-to-text latency** | <3 seconds end-to-end (speech end to text appearance) with `base.en` + rule-based cleanup | Automated benchmarks on reference hardware |
| **Hotkey response time** | <100ms (pill appears) | Instrumented timing in the app |
| **Transcription accuracy** | >95% word accuracy on clean English speech with `base.en` | Tested against a standardized 500-word dictation passage |
| **Text injection success rate** | >99% across top 20 Windows apps | Manual QA testing matrix |
| **Idle memory usage** | <80MB with model loaded | Windows Task Manager / Process Explorer |
| **Installer size** | <10MB (before model download) | Build output measurement |
| **Cold start time** | <3 seconds to ready state (model loaded, hotkey registered) | Automated startup benchmark |
| **Crash rate** | <1% of sessions | Opt-in crash reporting (if user consents) |
| **Onboarding completion** | >80% of users complete all 5 steps | In-app event tracking (opt-in only, FR-35) |

---

## 11. Open Questions

| ID | Question | Impact | Owner |
|----|----------|--------|-------|
| OQ-1 | What is the final product name? | Branding, domain, marketing materials | Product |
| OQ-2 | What is the exact pricing model? (One-time purchase, subscription, or hybrid? What price points?) | Revenue, positioning, feature gating | Product |
| OQ-3 | Which specific local LLM model should be the default for Tier 2 cleanup? (Phi-3-mini, Llama 3.2 1B, other?) | Model size, RAM requirements, cleanup quality | Engineering |
| OQ-4 | Should the local LLM run via llama.cpp (Rust bindings) or ONNX Runtime? | Dependency management, model compatibility, performance | Engineering |
| OQ-5 | How will Whisper model files be hosted for download? (GitHub releases, own CDN, bundled in a separate installer?) | Infrastructure cost, download reliability, update mechanism | Engineering |
| OQ-6 | What is the licensing model? (Proprietary, source-available, open-source core?) | Community building, competitive moat, monetization | Product |
| OQ-7 | Should GPU acceleration support DirectML (for AMD/Intel GPUs) in addition to CUDA and Vulkan? | Hardware compatibility, engineering effort | Engineering |
| OQ-8 | How should the app handle Windows Defender / SmartScreen warnings for a new unsigned app? | First-run UX, trust, code signing certificate cost | Engineering |

---

## 12. Standards Compliance

```
standards_version: 1.0.0
applied_standards:
  - global/principles.md
  - global/security-privacy.md
  - global/terminology.md
  - domains/code-internal-architecture.md
  - phases/create-prd.md
```

### Applied Rules
- [PRIN-1] User-First: All requirements prioritize user experience (hotkey response time, invisible operation, privacy).
- [PRIN-2] Quality Over Speed: Defined specific, measurable performance targets rather than shipping fast with vague quality.
- [PRIN-5] Incremental Delivery: Clear MVP scope with deferred items in Non-Goals.
- [PRIN-10] Simplicity: Minimal UI surfaces (pill + settings + tray). No main window.
- [PRIN-14] Reuse Before Build: Uses existing open-source engines (whisper.cpp, Silero VAD, existing Rust crates).
- [SEC-1] No Secrets in Code: API keys stored encrypted via `keyring` crate (FR-34).
- [SEC-2] PII Protection: Audio never persisted, no telemetry without consent (FR-33, FR-35).
- [SEC-6] Encryption in Transit: Cloud LLM calls via HTTPS only (FR-34).
- [INT-1] TypeScript Strict Mode: Frontend uses strict mode.
- [INT-8] Tailwind for Styling: All frontend styling via Tailwind CSS.
- [INT-6] Use shadcn/ui Components: Settings UI built with shadcn/ui.
- [PRD-1] Standard Sections: All required sections present.
- [PRD-2] Numbered Requirements: All requirements numbered (FR-1 through FR-36).
- [PRD-3] Testable Requirements: Requirements include specific targets (latency, accuracy, sizes).
- [PRD-4] Link to User Need: Each requirement traces to user stories and/or goals.
- [PRD-5] Research Reference: References four RSD documents.
- [PRD-6] Open Questions: Section 11 documents unresolved decisions.
- [PRD-7] Assumptions: Section 9 lists key assumptions with impact analysis.

### Deviations
- None.
