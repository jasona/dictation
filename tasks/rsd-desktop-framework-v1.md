# Research Summary Document (RSD): Desktop Framework Selection v1

## 1. Project Overview

- **User brief:** Research and compare desktop application frameworks for building a cross-platform voice-to-text dictation utility app with modern UI, system tray presence, global hotkeys, text injection, and local ML model integration.
- **Project type(s):** Product / Feature
- **Research depth:** Deep
- **Primary research focus:** External best practices, framework comparison, and competitive landscape

---

## 2. Existing Context & Assets (Internal)

### 2.1 Related Requirements & Docs
- No existing PRDs, CRDs, or DRDs in `/tasks/` -- this is the first research document for this project.

### 2.2 Codebase / System Context
- No existing codebase yet. This is a greenfield project.
- **Team standards** (`standards/domains/code-internal-architecture.md`) define a web-first stack:
  - TypeScript (strict mode)
  - React (Server Components default, client when needed)
  - Tailwind CSS for styling
  - shadcn/ui + Radix UI for component library
  - Zod for validation
  - React Query for data fetching
  - Vitest + React Testing Library for testing
- These standards strongly favor frameworks that support web-based frontends (React + Tailwind + shadcn/ui), as the team can reuse existing knowledge and component patterns directly.

### 2.3 Key Constraint from Standards
- The team's existing investment in React/TypeScript/Tailwind/shadcn means frameworks that support web frontends (Electron, Tauri, Wails, Neutralinojs) have a significant advantage over frameworks requiring entirely different UI paradigms (Flutter/Dart, Qt/C++/Python, .NET MAUI/XAML).

---

## 3. User & Business Context

- **Target user(s):** Knowledge workers, writers, developers, executives -- anyone who needs fast, accurate voice-to-text on desktop.
- **User goals & pain points:**
  - Dictate text anywhere on the OS (into any application)
  - Low latency, high accuracy speech-to-text
  - Minimal resource usage (runs in background as a utility)
  - Beautiful, non-intrusive UI (system tray/menu bar app with overlay)
  - Privacy-first (local ML models, no cloud dependency for core STT)
- **Business goals:** Ship on Windows first, expand to macOS and Linux. Compete with WisprFlow, Superwhisper, and similar tools.
- **Success signals:** Fast startup, low memory footprint, accurate dictation, seamless text injection into any app, polished UI.

---

## 4. Framework Comparison

### 4.1 Detailed Framework Analysis

---

#### ELECTRON

**Overview:** Chromium + Node.js runtime. The most established desktop web-app framework. Used by VS Code, Slack, Discord, Notion, Obsidian, 1Password, and many others.

**Cross-Platform Support:**
- Windows: Excellent
- macOS: Excellent
- Linux: Excellent
- Maturity: 10+ years, battle-tested on all three platforms
- Rating: **5/5**

**App Size / Memory Footprint:**
- Installer size: ~80-150 MB (bundles full Chromium)
- Runtime memory: ~100-300 MB baseline (varies widely by app complexity)
- For a background utility, this is the primary downside -- it is heavy
- Rating: **2/5**

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- System tray: Built-in `Tray` API, mature and reliable
- Global hotkeys: Built-in `globalShortcut` API
- Accessibility: Full access via Node.js native modules; can call Win32 APIs, macOS Accessibility APIs, AT-SPI on Linux via native addons (node-ffi-napi, edge-js, or custom N-API modules)
- Text injection: Can use `robotjs`, `nut.js`, or `@nut-tree/nut-js` for simulating keystrokes; clipboard-based injection via `clipboard` module; Win32 `SendInput` via native modules
- File system, networking, IPC: Full Node.js access
- Rating: **5/5**

**Native Look and Feel vs Web-Based UI:**
- Purely web-based rendering (Chromium)
- Can achieve any design (glassmorphism, animations, dark mode) with full CSS/HTML power
- Does NOT look "native" by default, but this is an advantage for custom/branded UI (Linear-style)
- Rating: **5/5** (for custom/branded UI goal)

**ML/AI Model Integration:**
- Can call native C/C++/Rust libraries via Node.js N-API addons
- `whisper.cpp` has Node.js bindings (`whisper-node`, `@nickvdyck/whisper.cpp`)
- Can spawn native processes (Python, Rust binaries) and communicate via IPC/stdio
- Can use ONNX Runtime via `onnxruntime-node`
- WebGPU/WebNN available in Chromium for GPU acceleration
- Rating: **4/5**

**Ecosystem Maturity & Community:**
- Massive ecosystem, largest community of any desktop framework
- Thousands of production apps
- Extensive documentation, tutorials, Stack Overflow answers
- npm packages for virtually anything
- electron-forge, electron-builder for packaging
- Rating: **5/5**

**Development Speed & DX:**
- Use React, TypeScript, Tailwind, shadcn/ui directly -- zero UI rewrite
- Hot reload via webpack/vite
- Chrome DevTools for debugging
- Vast ecosystem of libraries
- Rating: **5/5**

**Modern UI Capability:**
- Full CSS3, animations, WebGL, glassmorphism, backdrop-filter
- Can create any UI imaginable
- Dark mode trivial with Tailwind dark: classes
- Rating: **5/5**

**Auto-Update:**
- `electron-updater` (mature, well-documented)
- Supports differential updates, staged rollouts
- Multiple code-signing options
- Rating: **5/5**

**Build & Packaging:**
- electron-forge (official), electron-builder (community standard)
- Code signing for Windows (EV cert), macOS (Apple Developer)
- MSI/NSIS/Squirrel installers for Windows
- DMG/pkg for macOS
- AppImage/deb/rpm/snap for Linux
- Rating: **5/5**

**Key Concern for This Project:** The 100+ MB footprint and 100+ MB RAM baseline is excessive for a utility app that primarily sits in the system tray. VS Code can use 500+ MB. For a dictation tool, users expect something lightweight.

---

#### TAURI v2

**Overview:** Rust backend + system webview (WebView2 on Windows, WebKit on macOS/Linux). v2 released stable in October 2024 with mobile support, plugins system, and enhanced IPC.

**Cross-Platform Support:**
- Windows: Excellent (WebView2, auto-installed on Windows 10+)
- macOS: Excellent (WebKit via WKWebView)
- Linux: Good (WebKitGTK -- requires user to have WebKit2GTK installed; rendering can vary slightly)
- Mobile: v2 adds iOS and Android support (beta maturity)
- Note: WebView rendering differences across platforms can cause subtle CSS inconsistencies, especially on Linux
- Rating: **4/5**

**App Size / Memory Footprint:**
- Installer size: ~2-8 MB (does not bundle a browser engine)
- Runtime memory: ~20-50 MB baseline
- This is the standout advantage -- 10-20x smaller than Electron
- For a background utility, this is ideal
- Rating: **5/5**

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- System tray: Built-in plugin (`tauri-plugin-tray`), works on all platforms
- Global hotkeys: Built-in plugin (`tauri-plugin-global-shortcut`)
- Accessibility: Can access any native API from the Rust backend; full Win32/Cocoa/GTK access
- Text injection: Rust backend can call `SendInput` (Win32), `CGEventPost` (macOS), `XTest` (Linux) -- requires writing Rust code or using crates like `enigo`, `rdev`, or `autopilot-rs`
- IPC between frontend (JS) and backend (Rust) is well-designed via commands and events
- Tauri v2 plugin system is extensible and well-documented
- Rating: **4.5/5** (slightly lower than Electron because solutions require more Rust expertise)

**Native Look and Feel vs Web-Based UI:**
- Web-based rendering via system webview
- Can achieve the same custom UI as Electron (React + Tailwind + shadcn)
- CSS feature support depends on system webview version (WebView2 on Windows is Chromium-based, so excellent; WebKit on macOS is generally good; Linux WebKitGTK can lag behind)
- Rating: **4.5/5** (slight risk of CSS inconsistencies on Linux WebKitGTK)

**ML/AI Model Integration:**
- Rust backend can directly link C/C++ libraries via FFI (zero overhead)
- `whisper-rs` crate provides Rust bindings to `whisper.cpp` -- first-class integration
- Can link ONNX Runtime, TensorFlow Lite, or any native ML runtime from Rust
- Rust's `bindgen` makes C/C++ interop straightforward
- GPU access via Vulkan/Metal/DirectX from Rust
- This is arguably the **best** framework for native ML integration
- Rating: **5/5**

**Ecosystem Maturity & Community:**
- Growing rapidly; v2 stable since October 2024
- ~85k GitHub stars (as of early 2025)
- Active Discord community (15k+ members)
- Plugin ecosystem is expanding but smaller than Electron's
- Some plugins are still maturing; community plugins vary in quality
- Fewer StackOverflow answers and tutorials than Electron
- Rating: **3.5/5** (growing fast but not yet at Electron's level)

**Development Speed & DX:**
- Frontend: Use React, TypeScript, Tailwind, shadcn/ui directly -- same as Electron
- Backend: Requires Rust knowledge for system-level features
- Rust has a steep learning curve but produces reliable, fast code
- Vite integration is first-class (`create-tauri-app` uses Vite by default)
- Hot reload for frontend; backend changes require recompilation (fast with incremental builds)
- Rating: **3.5/5** (excellent frontend DX, but Rust backend raises the bar)

**Modern UI Capability:**
- Same as any web-based approach: full CSS3, animations, glassmorphism
- WebView2 (Windows) supports all modern CSS including `backdrop-filter`
- WebKit (macOS) supports `backdrop-filter` natively
- Linux WebKitGTK may have minor gaps
- Rating: **4.5/5**

**Auto-Update:**
- Built-in updater plugin (`tauri-plugin-updater`)
- Supports update checks, downloads, and installation
- Supports code signing
- Less mature than electron-updater but functional
- Rating: **4/5**

**Build & Packaging:**
- Built-in `tauri build` command
- Windows: MSI and NSIS installers
- macOS: DMG and .app bundles
- Linux: AppImage, deb
- Code signing supported
- GitHub Actions templates available
- Rating: **4.5/5**

**Key Advantage for This Project:** The tiny footprint (2-8 MB installer, 20-50 MB RAM) is perfect for a system tray utility. Rust backend provides zero-overhead access to `whisper.cpp` and system APIs. The web frontend means the team can use React/Tailwind/shadcn directly.

**Key Risk:** Requires Rust expertise for the backend. The team's existing TypeScript focus means a learning curve for system-level features. However, the frontend remains pure React/TypeScript.

---

#### FLUTTER DESKTOP

**Overview:** Google's cross-platform UI toolkit using Dart. Compiles to native code. Desktop support graduated to stable for Windows, macOS, and Linux.

**Cross-Platform Support:**
- Windows: Stable (since Flutter 3.0, 2022)
- macOS: Stable
- Linux: Stable
- Mobile (iOS/Android): Excellent -- Flutter's original platform
- Rating: **4/5** (desktop is stable but still behind mobile in maturity)

**App Size / Memory Footprint:**
- Installer size: ~15-25 MB
- Runtime memory: ~50-100 MB
- Moderate -- better than Electron, heavier than Tauri
- Rating: **3.5/5**

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- System tray: `tray_manager` or `system_tray` packages (community, varying quality)
- Global hotkeys: `hotkey_manager` package (community)
- Accessibility: Flutter has its own accessibility layer (SemanticsNode); accessing OS accessibility APIs (for text injection) requires platform channels (writing Kotlin/Swift/C++ per platform)
- Text injection: Requires platform channels to call native APIs -- significant per-platform work
- Platform channels add complexity: must write Dart + native code for each platform
- Rating: **2.5/5** (system-level access requires significant native platform code)

**Native Look and Feel vs Web-Based UI:**
- Flutter renders its own UI via Skia/Impeller -- does NOT use native widgets or webview
- Can look beautiful and custom (same design on all platforms)
- Does NOT look "native" (no native window chrome, menus feel different)
- Material Design and Cupertino widgets available but desktop-specific widgets are limited
- Can create custom designs but the ecosystem is mobile-focused
- Rating: **3.5/5** (great for custom UI, but desktop widget maturity lags)

**ML/AI Model Integration:**
- Dart FFI can call C/C++ libraries
- `whisper_dart` or custom FFI bindings to `whisper.cpp`
- Can use `tflite_flutter` for TensorFlow Lite models
- FFI works but is more cumbersome than Rust's native FFI
- Rating: **3.5/5**

**Ecosystem Maturity & Community:**
- Massive community (for mobile); desktop community is smaller
- Desktop-specific packages are fewer and less mature than mobile
- Many packages don't support desktop or have desktop-specific bugs
- Google backing provides stability
- Rating: **3/5** (for desktop specifically)

**Development Speed & DX:**
- Hot reload is excellent
- Dart is easy to learn
- BUT: the team would need to learn Dart and Flutter's widget system
- Cannot reuse React/Tailwind/shadcn/ui -- complete UI rewrite
- Platform channels for system APIs add development overhead
- Rating: **2/5** (given team's existing React/TS/Tailwind stack)

**Modern UI Capability:**
- Custom rendering engine means full control over visuals
- Animations are first-class (implicit/explicit animations, Rive)
- Glassmorphism, dark mode, custom designs all possible
- Desktop-specific UI patterns (menu bars, window management) are less polished
- Rating: **4/5**

**Auto-Update:**
- No built-in solution
- Community packages exist but are immature
- Would likely need a custom solution (Sparkle on macOS, custom on Windows)
- Rating: **2/5**

**Build & Packaging:**
- `flutter build windows/macos/linux`
- Windows: MSIX installer
- macOS: .app bundle
- Linux: AppImage/deb (requires additional tooling)
- Rating: **3/5**

**Key Concern for This Project:** Requires abandoning React/TypeScript/Tailwind stack entirely. Desktop ecosystem is less mature. Platform channels for text injection and accessibility APIs add significant per-platform complexity. Not a good fit given team's web-tech expertise.

---

#### QT (PySide6/PyQt6 or C++)

**Overview:** The most mature cross-platform desktop framework. C++ core with Python bindings. Used by VLC, OBS Studio, Telegram Desktop, KDE, and many enterprise applications.

**Cross-Platform Support:**
- Windows: Excellent
- macOS: Excellent
- Linux: Excellent (Qt is THE Linux desktop framework)
- 30+ years of cross-platform maturity
- Rating: **5/5**

**App Size / Memory Footprint:**
- C++ Qt: ~15-30 MB installer, ~30-60 MB RAM
- Python Qt: ~30-80 MB installer (includes Python runtime), ~60-120 MB RAM
- Rating: **3.5/5** (C++) or **3/5** (Python)

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- System tray: `QSystemTrayIcon` -- mature, built-in
- Global hotkeys: Platform-specific implementation needed (no built-in cross-platform API, but libraries exist)
- Accessibility: Qt has its own accessibility framework (QAccessible) -- very mature
- Text injection: Can call any native API via C++ or Python ctypes/cffi
- Rating: **4.5/5**

**Native Look and Feel vs Web-Based UI:**
- QWidgets look native on each platform (uses native theme engine)
- QML/Qt Quick allows custom UI (more like Flutter's approach)
- Can look native OR custom, depending on approach
- Achieving Linear-style modern dark UI with QML is possible but requires significant QML expertise
- Rating: **3/5** (for modern custom UI -- QML can do it but it's a different paradigm)

**ML/AI Model Integration:**
- C++ Qt: Direct C/C++ integration, zero overhead -- `whisper.cpp` links directly
- Python Qt: Can use Python ML ecosystem (PyTorch, ONNX, etc.) natively; `whisper.cpp` via ctypes or cython
- Rating: **5/5** (C++) or **4.5/5** (Python)

**Ecosystem Maturity & Community:**
- Extremely mature (30+ years)
- Extensive documentation
- Large community but skews enterprise/C++
- Python Qt community is active but smaller
- Rating: **4.5/5**

**Development Speed & DX:**
- C++ is slow to develop in; QML has a learning curve
- Python Qt is faster but still requires learning Qt's paradigm
- Cannot reuse React/Tailwind/shadcn -- completely different UI system
- No hot reload (QML Hot Reload exists but is limited)
- Rating: **2/5** (given team's web background)

**Modern UI Capability:**
- QML can create modern, animated UIs
- Glassmorphism and custom effects possible but require more effort than CSS
- Dark mode possible but requires manual theming
- Rating: **3/5**

**Auto-Update:**
- No built-in solution
- Qt Installer Framework can handle updates
- Custom solutions needed
- Rating: **2.5/5**

**Build & Packaging:**
- Qt Installer Framework
- cx_Freeze or PyInstaller for Python
- Well-established but complex build systems
- Rating: **3/5**

**Key Concern for This Project:** Requires completely different tech stack (C++/Python + QML/QWidgets). No reuse of team's React/TypeScript/Tailwind expertise. Modern dark UI is achievable but requires significantly more effort than web-based approaches.

---

#### .NET MAUI / WinUI 3

**Overview:** Microsoft's cross-platform framework (.NET MAUI) and Windows-native UI framework (WinUI 3). MAUI targets Windows, macOS, iOS, Android. WinUI 3 is Windows-only.

**Cross-Platform Support:**
- Windows: Excellent (WinUI 3 is Microsoft's recommended Windows UI)
- macOS: MAUI support exists but is notably less polished (uses Mac Catalyst)
- Linux: **NOT supported** by MAUI -- critical gap
- iOS/Android: MAUI supports these
- Rating: **2/5** (Linux gap is disqualifying for "later support Linux" requirement)

**App Size / Memory Footprint:**
- WinUI 3: ~30-50 MB installer, ~50-80 MB RAM
- .NET MAUI: ~50-100 MB installer
- Rating: **3/5**

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- Windows: Full Win32 API access, WinRT APIs, excellent accessibility (UI Automation)
- System tray: Available via Win32 interop
- Global hotkeys: Win32 `RegisterHotKey` via P/Invoke
- Text injection: Full Win32 `SendInput`, UI Automation for accessible text injection
- On macOS: Limited to Mac Catalyst APIs, significantly reduced capability
- Rating: **4/5** (Windows), **2/5** (macOS), **0/5** (Linux)

**Native Look and Feel vs Web-Based UI:**
- WinUI 3: Native Windows look and feel (Fluent Design, Mica, Acrylic)
- MAUI: Uses native controls on each platform but they look different per platform
- Cannot achieve consistent custom design across platforms easily
- Rating: **4/5** (Windows only), **2/5** (cross-platform consistency)

**ML/AI Model Integration:**
- .NET can call native libraries via P/Invoke
- ONNX Runtime has first-class .NET bindings (Microsoft project)
- Windows ML API available for WinUI 3 apps
- `whisper.net` provides .NET bindings to whisper.cpp
- Rating: **4.5/5** (especially on Windows)

**Ecosystem Maturity & Community:**
- WinUI 3: Mature for Windows development
- .NET MAUI: Active development but has had a rough launch; community frustration with bugs and macOS quality
- Rating: **3/5**

**Development Speed & DX:**
- C# is productive; XAML has a learning curve
- Cannot reuse React/Tailwind/shadcn
- Visual Studio tooling is excellent (Windows)
- Rating: **2.5/5** (given team's web background)

**Modern UI Capability:**
- WinUI 3 with Fluent Design can look very modern (Windows 11 native)
- Mica, Acrylic materials built-in
- But this is Windows-only for the polished experience
- Rating: **4/5** (Windows), **2/5** (cross-platform)

**Auto-Update:**
- MSIX auto-update on Windows (Store or sideloaded)
- ClickOnce deployment
- Rating: **4/5** (Windows), **2/5** (other platforms)

**Build & Packaging:**
- MSIX for Windows (excellent)
- macOS: .pkg with limitations
- Linux: N/A
- Rating: **3/5**

**Key Concern for This Project:** No Linux support is a blocker. macOS support is second-class. Requires abandoning React/TypeScript/Tailwind. Best suited for Windows-only apps. Not recommended for this project.

---

#### NEUTRALINOJS

**Overview:** Lightweight alternative to Electron. Uses system webview + a lightweight backend (C++). Much smaller than Electron.

**Cross-Platform Support:**
- Windows: Good (WebView2)
- macOS: Good (WebKit)
- Linux: Good (WebKitGTK)
- Rating: **3.5/5**

**App Size / Memory Footprint:**
- Installer size: ~2-5 MB
- Runtime memory: ~20-40 MB
- Very lightweight -- similar to Tauri
- Rating: **5/5**

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- System tray: Built-in API (`Neutralino.os.setTray`)
- Global hotkeys: Limited -- no built-in API (must use OS-specific workarounds or extensions)
- Accessibility: No built-in API; extensions mechanism exists but is limited
- Text injection: Very limited -- would need external native processes
- The backend API surface is much smaller than Electron or Tauri
- Extensions (child processes) can extend functionality but add complexity
- Rating: **2/5**

**Native Look and Feel vs Web-Based UI:**
- Web-based rendering via system webview
- Same capability as Tauri for custom UI
- Rating: **4/5**

**ML/AI Model Integration:**
- Limited built-in capabilities
- Can spawn native processes as extensions
- Cannot directly link native libraries like Tauri (Rust FFI) or Electron (N-API)
- Would need to run whisper.cpp as a separate process and communicate via IPC
- Rating: **2/5**

**Ecosystem Maturity & Community:**
- Small community (~7k GitHub stars)
- Limited plugin/extension ecosystem
- Fewer resources, tutorials, and StackOverflow answers
- Active development but small team
- Rating: **2/5**

**Development Speed & DX:**
- Frontend: Can use React, TypeScript, Tailwind
- Backend: Limited API; must write extensions for anything beyond basics
- Rating: **3/5**

**Modern UI Capability:**
- Same as any webview-based approach
- Rating: **4/5**

**Auto-Update:**
- Built-in updater API (`Neutralino.updater`)
- Less mature than Electron or Tauri updaters
- Rating: **3/5**

**Build & Packaging:**
- `neu build` command
- Generates platform-specific bundles
- Less flexible than Electron or Tauri
- Rating: **3/5**

**Key Concern for This Project:** Too limited for system-level features needed (global hotkeys, text injection, ML integration). Small community means higher risk. Not suitable for a dictation app with deep OS integration needs.

---

#### WAILS (Go + Webview)

**Overview:** Go backend + system webview. Similar concept to Tauri but with Go instead of Rust.

**Cross-Platform Support:**
- Windows: Good (WebView2)
- macOS: Good (WebKit)
- Linux: Good (WebKitGTK)
- v2 is stable
- Rating: **4/5**

**App Size / Memory Footprint:**
- Installer size: ~5-10 MB
- Runtime memory: ~30-60 MB
- Lightweight, similar to Tauri
- Rating: **4.5/5**

**System API Access (Tray, Hotkeys, Accessibility, Text Injection):**
- System tray: Built-in support
- Global hotkeys: Limited built-in support; can use Go libraries
- Accessibility: Can call native APIs via Go's CGo (C interop) or syscall packages
- Text injection: Possible via Go native bindings but Go's C interop (CGo) is less ergonomic than Rust FFI
- Rating: **3.5/5**

**Native Look and Feel vs Web-Based UI:**
- Web-based via system webview
- Same capability as Tauri/Neutralino
- Rating: **4/5**

**ML/AI Model Integration:**
- Go can call C libraries via CGo
- `whisper.cpp` has Go bindings (`go-whisper`)
- CGo adds complexity and can hurt build times
- Go's ML ecosystem is smaller than Python/Rust/C++
- Rating: **3/5**

**Ecosystem Maturity & Community:**
- Growing community (~25k GitHub stars)
- Smaller than Electron or Tauri
- Documentation is good but fewer resources than Electron
- Rating: **3/5**

**Development Speed & DX:**
- Frontend: Can use React, TypeScript, Tailwind
- Backend: Go is productive and has a gentle learning curve
- Go is easier to learn than Rust
- Rating: **4/5**

**Modern UI Capability:**
- Same as any webview approach
- Rating: **4/5**

**Auto-Update:**
- No built-in updater
- Must implement custom solution
- Rating: **2/5**

**Build & Packaging:**
- `wails build` command
- Windows: NSIS installer
- macOS: .app bundle
- Linux: AppImage
- Rating: **3.5/5**

**Key Concern for This Project:** Go's CGo interop is less ergonomic than Rust FFI for ML model integration. Smaller ecosystem than Tauri. Missing built-in auto-updater. Viable option but Tauri has more momentum and better native library integration.

---

### 4.2 Comparison Table

| Criterion | Electron | Tauri v2 | Flutter Desktop | Qt (C++/Python) | .NET MAUI | Neutralinojs | Wails |
|---|---|---|---|---|---|---|---|
| **Cross-Platform** | 5/5 | 4/5 | 4/5 | 5/5 | 2/5 | 3.5/5 | 4/5 |
| **App Size** | 2/5 (80-150MB) | 5/5 (2-8MB) | 3.5/5 (15-25MB) | 3.5/5 (15-80MB) | 3/5 (30-100MB) | 5/5 (2-5MB) | 4.5/5 (5-10MB) |
| **Memory Footprint** | 2/5 (100-300MB) | 5/5 (20-50MB) | 3.5/5 (50-100MB) | 3.5/5 (30-120MB) | 3/5 (50-80MB) | 5/5 (20-40MB) | 4.5/5 (30-60MB) |
| **System APIs** | 5/5 | 4.5/5 | 2.5/5 | 4.5/5 | 2-4/5* | 2/5 | 3.5/5 |
| **UI Stack Reuse** | 5/5 (React) | 5/5 (React) | 1/5 (Dart) | 1/5 (QML) | 1/5 (XAML) | 5/5 (React) | 5/5 (React) |
| **ML Integration** | 4/5 | 5/5 | 3.5/5 | 5/5 | 4.5/5 | 2/5 | 3/5 |
| **Ecosystem** | 5/5 | 3.5/5 | 3/5 | 4.5/5 | 3/5 | 2/5 | 3/5 |
| **Dev Speed (for this team)** | 5/5 | 3.5/5 | 2/5 | 2/5 | 2.5/5 | 3/5 | 4/5 |
| **Modern UI** | 5/5 | 4.5/5 | 4/5 | 3/5 | 4/5* | 4/5 | 4/5 |
| **Auto-Update** | 5/5 | 4/5 | 2/5 | 2.5/5 | 4/5* | 3/5 | 2/5 |
| **Build/Package** | 5/5 | 4.5/5 | 3/5 | 3/5 | 3/5 | 3/5 | 3.5/5 |
| **WEIGHTED TOTAL** | **48/55** | **48.5/55** | **32/55** | **37.5/55** | **32/55** | **37.5/55** | **41/55** |

*\* .NET MAUI ratings are Windows-only scores; cross-platform scores are significantly lower.*

---

## 5. Text Injection Research: How Dictation Apps Work

### 5.1 Text Injection Methods (How Dictated Text Gets Into Other Apps)

This is a critical technical challenge for any dictation tool. There are four primary approaches, and most successful apps use a combination:

#### Method 1: Simulated Keystrokes (SendInput / CGEvent / XTest)

**How it works:** The dictation app simulates keyboard input at the OS level, making the target application think the user is physically typing.

**Windows:** `SendInput()` Win32 API or `keybd_event()` (legacy). Sends synthetic keyboard events to the focused window.

**macOS:** `CGEventPost()` or `CGEventCreateKeyboardEvent()` from Core Graphics. Requires Accessibility permissions (System Preferences > Privacy & Security > Accessibility).

**Linux:** XTest extension (`XTestFakeKeyEvent`) for X11; on Wayland, simulated input is restricted for security (major limitation).

**Pros:**
- Works with virtually any application
- Appears as natural keyboard input
- Handles text fields, editors, terminals, etc.

**Cons:**
- Slow for large text blocks (must type character by character or use OS-specific batch methods)
- Can be intercepted or blocked by some apps (games, secure fields)
- Unicode handling requires careful implementation (especially on Windows with `SendInput` and `WM_CHAR`)
- Requires Accessibility permissions on macOS

**Used by:** WisprFlow, Superwhisper (as fallback), Dragon NaturallySpeaking

#### Method 2: Clipboard-Based Injection (Paste)

**How it works:** Copy the transcribed text to the clipboard, then simulate Ctrl+V (Cmd+V on macOS) to paste it into the focused application.

**Pros:**
- Fast for large blocks of text
- Unicode support is handled by the OS clipboard
- Works with most applications
- Simple to implement

**Cons:**
- Overwrites the user's clipboard (must save/restore clipboard contents)
- Some apps detect programmatic paste differently
- "Paste" behavior varies by app (plain text vs. rich text)
- User may notice clipboard contents changed

**Used by:** WisprFlow (primary method for longer text), many dictation tools as the primary approach

#### Method 3: Accessibility APIs (UI Automation / AX API / AT-SPI)

**How it works:** Use OS accessibility APIs to directly set the value of a text field in the target application.

**Windows:** UI Automation (`IUIAutomation`) -- can find text elements and set their `Value` property directly. Also `IAccessible` (legacy MSAA).

**macOS:** AX API (`AXUIElementSetAttributeValue` with `kAXValueAttribute`) -- can set the value of accessible text fields. Requires Accessibility permissions.

**Linux:** AT-SPI (Assistive Technology Service Provider Interface) -- D-Bus based accessibility API.

**Pros:**
- Can directly manipulate text fields without simulating input
- Can insert text at cursor position in some implementations
- More reliable than keystroke simulation for specific use cases

**Cons:**
- Not all apps expose proper accessibility elements
- Implementation varies significantly per application
- Electron apps, web browsers, and some custom UI frameworks have inconsistent accessibility trees
- Complex to implement cross-platform

**Used by:** Superwhisper (for intelligent text insertion), macOS built-in dictation, some enterprise dictation tools

#### Method 4: Input Method Framework (IME)

**How it works:** Register as an Input Method Editor (IME) at the OS level. This is how Japanese/Chinese/Korean input methods work -- they intercept keyboard input and output composed text.

**Windows:** Text Services Framework (TSF) or ImmGetContext/ImmSetComposition APIs.

**macOS:** Input Method Kit (IMKit).

**Linux:** IBus or Fcitx frameworks.

**Pros:**
- Most "native" approach -- the OS treats dictated text as legitimate input
- Works with any application that accepts text input
- Can show inline preview/composition window
- Handles cursor position correctly

**Cons:**
- Very complex to implement correctly
- Platform-specific with no abstraction layer
- Can conflict with user's existing IME (especially for CJK language users)
- Registration and lifecycle management is complex

**Used by:** Windows built-in voice typing (Win+H), some enterprise-grade dictation tools

### 5.2 What Successful Dictation Apps Use

#### WisprFlow
- **Platform:** macOS (primary), Windows (newer)
- **Framework:** Likely Swift/AppKit for macOS, possibly Electron or Tauri for Windows
- **Text injection:** Clipboard-based paste (primary), simulated keystrokes (fallback)
- **STT:** Cloud-based (OpenAI Whisper API) + local models
- **UI:** Menu bar app with floating overlay

#### Superwhisper
- **Platform:** macOS only
- **Framework:** Native Swift/SwiftUI
- **Text injection:** AX API (primary), clipboard paste (fallback), simulated keystrokes (fallback)
- **STT:** Local whisper.cpp models (multiple sizes)
- **UI:** Menu bar app with minimal floating overlay
- **Key feature:** Uses accessibility API to detect the context of the focused text field and insert text intelligently

#### Windows Built-in Voice Typing (Win+H)
- **Framework:** Native C++/WinUI
- **Text injection:** Input Method Framework (TSF) -- deepest OS integration
- **STT:** Cloud Azure Speech + local Windows speech recognition

#### Talon Voice
- **Platform:** Windows, macOS, Linux
- **Framework:** Python backend with custom native UI per platform
- **Text injection:** Platform-specific native code (Win32 SendInput, macOS AX API, X11 XTest)
- **STT:** Uses custom models + wav2letter

#### WhisperTyping
- **Platform:** Windows
- **Framework:** Electron
- **Text injection:** Simulated keystrokes via robotjs/nut.js
- **STT:** Local whisper.cpp via whisper-node

### 5.3 Recommended Text Injection Strategy for This Project

**Recommended layered approach:**

1. **Primary: Clipboard-based paste** -- Fast, reliable, works everywhere. Save clipboard, set text, simulate Ctrl+V/Cmd+V, restore clipboard.
2. **Secondary: Simulated keystrokes** -- For character-by-character input, corrections, or when clipboard paste is not appropriate.
3. **Future: Accessibility API integration** -- For intelligent text insertion (detecting cursor position, replacing selected text, understanding context).

**Implementation notes:**
- On Windows: Use Win32 `SendInput` for keystrokes, `SetClipboardData`/`GetClipboardData` for clipboard, `IUIAutomation` for accessibility
- On macOS: Use `CGEventPost` for keystrokes, `NSPasteboard` for clipboard, AX API for accessibility
- On Linux: Use XTest for X11 keystrokes (Wayland is problematic), xclip/xsel for clipboard

---

## 6. System Tray / Menu Bar App Patterns

### 6.1 Pattern Overview

A dictation app should behave as a **menu bar/system tray utility**:
- No visible main window by default
- Icon in system tray (Windows/Linux) or menu bar (macOS)
- Click tray icon to show controls/settings
- Global hotkey to start/stop dictation
- Optional floating overlay/widget during dictation
- Runs on system startup (optional)

### 6.2 Implementation by Framework

| Feature | Electron | Tauri v2 | Wails |
|---|---|---|---|
| **Tray icon** | `new Tray(icon)` | `tauri-plugin-tray` | `runtime.Tray` |
| **Tray menu** | `tray.setContextMenu()` | Built-in menu builder | Built-in menu builder |
| **Hide to tray on close** | `win.hide()` in `close` event | `prevent_close()` + hide | `WindowSetCloseHandler` |
| **Startup on boot** | `app.setLoginItemSettings()` | `tauri-plugin-autostart` | Manual (registry/plist) |
| **Floating overlay** | `new BrowserWindow({frame:false, alwaysOnTop:true, transparent:true})` | `WindowBuilder::transparent().always_on_top()` | `WindowSetAlwaysOnTop` |
| **Global hotkeys** | `globalShortcut.register()` | `tauri-plugin-global-shortcut` | Go library or platform code |

### 6.3 Recommended Pattern

```
[System Tray/Menu Bar]
    |
    +-- Left-click: Toggle dictation (start/stop)
    +-- Right-click: Context menu
         +-- Status indicator (Recording... / Idle)
         +-- Settings
         +-- Model selection
         +-- Check for updates
         +-- Quit
    |
[Floating Overlay] (shown during dictation)
    +-- Waveform/level indicator
    +-- Real-time transcription preview
    +-- Stop button
    +-- Small, draggable, always-on-top, transparent background
```

---

## 7. Constraints, Risks, and Dependencies

### 7.1 Constraints

- **Technical:** Must support Windows immediately; macOS and Linux follow. Must run efficiently in background. Must integrate local ML models (whisper.cpp).
- **Team:** Existing expertise is React/TypeScript/Tailwind/shadcn. Learning a new language (Rust, Dart, C++) adds risk and timeline.
- **Performance:** STT processing is CPU/GPU intensive; framework overhead should be minimal to leave resources for ML inference.

### 7.2 Risks

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| Rust learning curve (Tauri) | Medium -- slows initial backend development | High | Start with simple Rust commands; use existing crates; frontend remains React/TS |
| WebView rendering inconsistencies (Tauri, Wails) on Linux | Low-Medium | Medium | Test early on Linux; use CSS feature detection; Linux is last priority |
| Text injection reliability across apps | High -- core feature | Medium | Implement layered approach (clipboard > keystrokes > accessibility) |
| whisper.cpp integration complexity | Medium | Low-Medium | Well-documented bindings exist for both Node.js and Rust |
| Electron memory bloat | Medium -- poor user perception | High (inherent) | Cannot fully mitigate; Electron is inherently heavy |

### 7.3 Dependencies

- **whisper.cpp** -- Core STT engine; has bindings for Node.js (`whisper-node`), Rust (`whisper-rs`), Go (`go-whisper`)
- **System webview** (Tauri/Wails) -- WebView2 auto-installs on Windows 10+; WebKitGTK needed on Linux
- **OS accessibility permissions** -- macOS requires user to grant Accessibility access; Windows UAC may prompt

---

## 8. Opportunities & Ideas

### 8.1 Reuse Opportunities
- Full reuse of React/TypeScript/Tailwind/shadcn/ui for the frontend (Electron, Tauri, Wails, Neutralinojs)
- shadcn/ui dark theme as foundation for Linear-inspired design
- Tailwind `dark:` variant for dark-first design
- Existing component patterns from team standards

### 8.2 Quick Wins
- Tauri's `create-tauri-app` scaffold with React+TypeScript+Vite template gets a running app in minutes
- `whisper-rs` crate provides ready-made whisper.cpp integration for Tauri
- shadcn/ui theming system naturally supports dark mode

### 8.3 Differentiation Ideas
- Floating transparent overlay with glassmorphism (`backdrop-filter: blur()`) during dictation
- Real-time waveform visualization using Web Audio API or native audio stream
- Intelligent text injection using accessibility APIs (context-aware insertion)
- Command mode ("new line", "select all", "period") with custom grammar

---

## 9. Key Findings

### 9.1 Product / Feature Findings

1. **Tauri v2 is the strongest overall choice** for this project. It combines the team's React/TypeScript/Tailwind expertise (web frontend) with a Rust backend that provides tiny app size (2-8 MB), low memory usage (20-50 MB), zero-overhead ML integration via `whisper-rs`, and full system API access. The trade-off is learning Rust for the backend, but the frontend remains entirely in the team's comfort zone.

2. **Electron is the safest, fastest-to-ship choice** but has a fundamental mismatch with the "lightweight utility" requirement. A dictation app that uses 150-300 MB of RAM to sit in the system tray will feel heavy to users. However, the development speed and ecosystem are unmatched.

3. **Wails is a viable alternative** if the team prefers Go over Rust. Go has a gentler learning curve than Rust, but Go's C interop (CGo) is less ergonomic for ML model integration than Rust's FFI. Wails also lacks a built-in auto-updater.

4. **Flutter, Qt, and .NET MAUI are not recommended** because they require abandoning the team's React/TypeScript/Tailwind stack entirely. .NET MAUI also lacks Linux support.

5. **Neutralinojs is too limited** for the deep OS integration this project requires.

6. **Text injection is a solved problem** with well-known patterns: clipboard paste (primary), simulated keystrokes (secondary), accessibility APIs (advanced). All three approaches are implementable in Tauri, Electron, or Wails.

7. **Successful dictation apps** (WisprFlow, Superwhisper, Talon) all use platform-native or lightweight approaches. None of the major dictation tools use Electron. Superwhisper uses native Swift. WisprFlow uses Swift on macOS. This validates the preference for a lightweight framework like Tauri.

---

## 10. Recommendations for the Create Phase

### 10.1 Primary Recommendation: **Tauri v2**

**Why Tauri v2 wins:**

| Factor | Why it matters | Tauri's advantage |
|---|---|---|
| App size | Users expect utilities to be small | 2-8 MB vs Electron's 80-150 MB |
| Memory footprint | Runs in background constantly | 20-50 MB vs Electron's 100-300 MB |
| ML integration | Core feature (local whisper.cpp) | Rust FFI to whisper.cpp is zero-overhead via `whisper-rs` |
| Web frontend | Team expertise reuse | Full React/TypeScript/Tailwind/shadcn support |
| System APIs | Tray, hotkeys, text injection | Rust backend has full OS API access; mature plugins |
| Modern UI | Linear-inspired design | WebView2 supports all modern CSS |
| Future-proof | Growing ecosystem | 85k+ stars, active development, mobile support coming |

**The Rust learning curve is the primary trade-off.** However:
- The frontend (60-70% of the codebase for a UI-focused app) remains React/TypeScript
- The Rust backend handles well-scoped concerns: audio capture, whisper.cpp integration, text injection, system tray
- Existing Rust crates handle most of the heavy lifting (`whisper-rs`, `enigo`, `tauri` plugins)
- Rust enforces correctness, which is valuable for a system-level utility that runs in the background

### 10.2 Runner-Up: **Electron**

Choose Electron if:
- Time-to-market is the highest priority
- The team cannot invest in learning Rust
- The memory/size trade-off is acceptable (note: apps like Notion, 1Password, and Obsidian have normalized large Electron apps)

### 10.3 Honorable Mention: **Wails**

Choose Wails if:
- The team prefers Go over Rust
- A simpler backend language is desired
- Accept the trade-offs: no built-in auto-updater, less mature plugin ecosystem, CGo complexity for ML integration

### 10.4 Recommended Architecture (Tauri v2)

```
dictation-app/
├── src/                          # React frontend
│   ├── components/               # shadcn/ui + custom components
│   │   ├── ui/                   # shadcn/ui base components
│   │   ├── tray-popup/           # Tray popup window UI
│   │   ├── overlay/              # Floating dictation overlay
│   │   └── settings/             # Settings window
│   ├── hooks/                    # React hooks (useAudio, useDictation, etc.)
│   ├── lib/                      # Utilities, Tauri IPC wrappers
│   ├── styles/                   # Tailwind config, global styles
│   └── App.tsx                   # Main app entry
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri app setup, tray, windows
│   │   ├── audio.rs              # Microphone capture (cpal crate)
│   │   ├── stt.rs                # whisper.cpp integration (whisper-rs)
│   │   ├── injection.rs          # Text injection (enigo + platform-specific)
│   │   ├── hotkeys.rs            # Global shortcut handling
│   │   └── commands.rs           # Tauri IPC commands exposed to frontend
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
├── package.json                  # Frontend dependencies
├── tailwind.config.ts            # Tailwind configuration
├── tsconfig.json                 # TypeScript configuration
└── vite.config.ts                # Vite configuration
```

### 10.5 Key Rust Crates for the Backend

| Crate | Purpose |
|---|---|
| `whisper-rs` | Rust bindings to whisper.cpp for STT |
| `cpal` | Cross-platform audio input/output |
| `enigo` | Cross-platform keyboard/mouse simulation (text injection) |
| `tauri` | Core framework |
| `tauri-plugin-global-shortcut` | Global hotkeys |
| `tauri-plugin-autostart` | Launch on system boot |
| `tauri-plugin-updater` | Auto-update |
| `tauri-plugin-notification` | System notifications |
| `serde` / `serde_json` | Serialization for IPC |

### 10.6 Scope Recommendations

**MVP scope (must have):**
- System tray icon with context menu (Windows)
- Global hotkey to start/stop dictation
- Microphone audio capture
- Local whisper.cpp STT processing (base/small model)
- Clipboard-based text injection into focused app
- Minimal floating overlay showing recording state
- Dark theme UI
- Settings: model selection, hotkey configuration

**Stretch / Deferred:**
- macOS and Linux support
- Accessibility API-based text injection
- Real-time streaming transcription
- Multiple model size support with download manager
- Waveform visualization
- Command mode ("new paragraph", "delete that")
- Auto-update
- Glassmorphism/transparent overlay effects

### 10.7 Key Questions the PRD Should Answer

1. Should the app support cloud STT APIs (OpenAI Whisper, Azure Speech) as alternatives to local models, or is local-only the requirement?
2. What is the minimum acceptable latency for transcription (real-time streaming vs. process after recording stops)?
3. Should the app support dictation commands (voice commands for formatting, navigation)?
4. What model sizes should be supported? (tiny=39MB, base=74MB, small=244MB, medium=769MB, large=1550MB)
5. Is the Rust learning curve acceptable for the team, or should Electron be chosen for faster initial delivery?
6. Should the floating overlay show real-time partial transcription, or just a recording indicator?

### 10.8 Suggested Decisions to Lock In Now

- **Decision 1:** Use Tauri v2 as the framework (or Electron if Rust is a non-starter)
- **Decision 2:** Use clipboard-based paste as the primary text injection method for MVP
- **Decision 3:** Use whisper.cpp (via `whisper-rs` for Tauri or `whisper-node` for Electron) as the STT engine
- **Decision 4:** Windows-first, with architecture decisions made to support macOS/Linux later
- **Decision 5:** Use React + TypeScript + Tailwind + shadcn/ui for all frontend UI

---

## 11. Open Questions & Gaps

- **Wayland support on Linux:** Simulated keystrokes (XTest) do not work on Wayland. Wayland's security model restricts synthetic input. This may require an IME-based approach on Linux in the future.
- **GPU acceleration for whisper.cpp:** CUDA support on Windows, Metal on macOS. The framework choice does not affect this (handled at the whisper.cpp level), but build configuration and model distribution are considerations.
- **Audio capture while other apps use microphone:** Potential conflicts with Zoom, Teams, etc. May need exclusive/shared audio mode handling.
- **Licensing:** whisper.cpp is MIT licensed. Tauri is MIT/Apache-2.0. React/Tailwind are MIT. No licensing concerns identified.

---

## 12. Sources & References

- Tauri v2 documentation: https://v2.tauri.app
- Tauri GitHub: https://github.com/tauri-apps/tauri (~85k stars)
- Electron documentation: https://www.electronjs.org/docs
- Wails documentation: https://wails.io/docs
- Neutralinojs documentation: https://neutralino.js.org/docs
- Flutter desktop documentation: https://docs.flutter.dev/desktop
- .NET MAUI documentation: https://learn.microsoft.com/en-us/dotnet/maui/
- Qt documentation: https://doc.qt.io/
- whisper.cpp: https://github.com/ggerganov/whisper.cpp
- whisper-rs (Rust bindings): https://github.com/tazz4843/whisper-rs
- whisper-node (Node.js bindings): https://github.com/CTranslate2/whisper-node
- enigo (Rust keyboard simulation): https://github.com/enigo-rs/enigo
- cpal (Rust audio): https://github.com/RustAudio/cpal
- Superwhisper: https://superwhisper.com
- WisprFlow: https://wisprflow.com
- Talon Voice: https://talonvoice.com
- Windows UI Automation: https://learn.microsoft.com/en-us/windows/win32/winauto/entry-uiauto-win32
- macOS Accessibility API: https://developer.apple.com/documentation/accessibility
- Win32 SendInput: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
- macOS CGEvent: https://developer.apple.com/documentation/coregraphics/cgevent
