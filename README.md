# Vozr

A desktop voice-to-text app that runs entirely on your machine. Press a hotkey, speak, and your words are transcribed and injected into whatever application has focus — Notepad, VS Code, Slack, your browser, anything.

Built with [Tauri v2](https://v2.tauri.app/) (Rust backend) and React/TypeScript (frontend).

## How It Works

```
Hotkey → Mic Capture → Whisper STT → Text Cleanup → Paste into Active App
```

1. Press the hotkey (default: **F9**) to start recording
2. Speak naturally
3. Press the hotkey again to stop
4. Your speech is transcribed locally via [Whisper](https://github.com/ggerganov/whisper.cpp), cleaned up, and pasted into the focused application

All speech processing happens **locally on your device**. No audio is sent to any server. Cloud LLM cleanup (optional) sends only the transcribed text.

## Features

- **Local-first STT** — Whisper models run on-device via whisper.cpp (CPU or GPU)
- **Voice Activity Detection** — Silero VAD filters silence and auto-stops after 10s
- **Three-tier text cleanup** — Rule-based (always available), local LLM, or cloud LLM (OpenAI / Anthropic)
- **Clipboard injection** — Pastes via Ctrl+V for universal app compatibility, with keyboard simulation fallback
- **Floating pill UI** — Transparent overlay shows recording state, waveform, and processing status
- **System tray** — Lives in your taskbar with settings, pause/resume, and quit
- **Configurable hotkey** — Toggle or hold-to-record modes
- **No telemetry** — Zero analytics, crash reporting, or tracking of any kind

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| [Node.js](https://nodejs.org/) | 18+ | Frontend build |
| [Rust](https://rustup.rs/) | 1.70+ | Backend compilation |
| [LLVM/Clang](https://releases.llvm.org/) | 15+ | Required by whisper-rs (bindgen) |
| [CMake](https://cmake.org/download/) | 3.20+ | Required by whisper-rs (builds whisper.cpp) |
| [VS Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) | 2022 | MSVC compiler (Windows) |

### Windows Quick Install

```powershell
# Rust
winget install Rustlang.Rustup

# LLVM + CMake
winget install LLVM.LLVM
winget install Kitware.CMake

# VS Build Tools (if not already installed)
winget install Microsoft.VisualStudio.2022.BuildTools
```

## Getting Started

### 1. Clone and install

```bash
git clone https://github.com/jasona/dictation.git
cd dictation
npm install
```

### 2. Set environment variables

**PowerShell:**
```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
$env:CARGO_TARGET_DIR = "C:\Users\<you>\.cargo-targets\dictation"  # optional, avoids cloud sync lock issues
```

**Bash / Git Bash:**
```bash
export LIBCLANG_PATH="C:/Program Files/LLVM/bin"
export CARGO_TARGET_DIR="C:/Users/<you>/.cargo-targets/dictation"  # optional
```

`LIBCLANG_PATH` is required for whisper-rs compilation. `CARGO_TARGET_DIR` is only needed if your project lives in a synced folder (Dropbox, OneDrive) to avoid file-lock errors.

### 3. Run in development

```powershell
npm run tauri dev
```

The first build takes several minutes (compiling whisper.cpp, ONNX Runtime, etc.). Subsequent builds are incremental and much faster.

### 4. First launch

On first launch, the onboarding wizard walks you through:

1. **Microphone permission** — Grant access to your mic
2. **Hotkey setup** — Choose your activation shortcut
3. **Model download** — The `base.en` Whisper model (~150 MB) is downloaded automatically
4. **Test recording** — Try speaking to verify everything works

After onboarding, the app lives in your **system tray**. Right-click the tray icon for Settings, Pause, or Quit.

## Configuration

### Hotkey

Open **Settings** from the tray icon. Click the hotkey field and press your desired key combination. Modifier keys (Ctrl, Alt, Shift) are recommended to avoid conflicts.

**Activation modes:**
- **Toggle** (default) — Press once to start, press again to stop
- **Hold** — Hold the key to record, release to stop

### Whisper Models

Available models (downloaded on demand from Hugging Face):

| Model | Size | Speed | Accuracy |
|-------|------|-------|----------|
| `tiny.en` | ~75 MB | Fastest | Lower |
| `base.en` | ~150 MB | Fast | Good (default) |
| `small.en` | ~500 MB | Moderate | Better |
| `medium.en` | ~1.5 GB | Slow | Best |

Manage models in **Settings > Transcription**. English-only (`.en`) models are faster and more accurate for English speech.

### GPU Acceleration

GPU backends can be enabled at build time:

```powershell
# CUDA (NVIDIA GPUs)
$env:CARGO_FEATURES = "cuda"

# Vulkan (AMD / NVIDIA / Intel)
$env:CARGO_FEATURES = "vulkan"
```

Select the active backend in **Settings > Transcription > GPU Backend**.

### Text Cleanup Tiers

| Tier | Description | Requires |
|------|-------------|----------|
| **Rule-based** | Removes filler words (um, uh, you know), fixes capitalization, normalizes whitespace | Nothing (always available) |
| **Cloud LLM** | OpenAI (`gpt-4o-mini`) or Anthropic (`claude-haiku`) polishes grammar and phrasing | API key |
| **Local LLM** | On-device LLM cleanup via llama.cpp | `local-llm` feature flag + model download |

Configure in **Settings > Cleanup**. Cloud and local LLM tiers automatically fall back to rule-based if they fail.

### Audio Device

By default, the system default microphone is used. Select a specific device in **Settings > Audio**.

## Building for Production

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
npm run tauri build
```

This produces an NSIS installer in `src-tauri/target/release/bundle/nsis/`.

## Architecture

```
src-tauri/src/
├── main.rs              # Entry point
├── lib.rs               # Plugin registration, state management, app setup
├── pipeline.rs          # Orchestrator: hotkey → audio → STT → cleanup → inject
├── hotkey/              # Global shortcut registration, toggle/hold modes
├── audio/               # Mic capture (cpal), VAD (Silero via ONNX Runtime)
├── stt/                 # Whisper integration, model management
├── cleanup/             # Rule-based, cloud LLM, local LLM text cleanup
├── injection/           # Clipboard paste (arboard) + keyboard fallback (enigo)
├── tray/                # System tray icon and context menu
└── settings/            # Persistent settings store

src/
├── App.tsx              # Window router (pill, settings, onboarding)
├── components/
│   ├── pill/            # Floating overlay (waveform, glow, state machine)
│   ├── settings/        # Settings UI sections
│   ├── onboarding/      # 5-step first-run wizard
│   └── ui/              # shadcn/ui components
└── hooks/
    └── useSettings.ts   # Settings state management
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `whisper-rs` | Whisper.cpp bindings for speech-to-text |
| `cpal` | Cross-platform audio capture |
| `ort` | ONNX Runtime for Silero VAD |
| `arboard` | Clipboard read/write |
| `enigo` | Keyboard/mouse simulation |
| `keyring` | OS credential storage for API keys |
| `reqwest` | HTTP client for model downloads and cloud APIs |

## Privacy

- All speech-to-text processing runs locally via Whisper
- No audio is ever transmitted to any server
- Cloud LLM cleanup (when enabled) sends only transcribed text to OpenAI or Anthropic
- API keys are stored in the OS credential manager (Windows Credential Store)
- No analytics, telemetry, or crash reporting is included
- The auto-updater (when configured) checks for versions only — no user data is sent

## License

MIT
