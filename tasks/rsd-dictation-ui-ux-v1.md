# Research Summary Document (RSD): Dictation App UI/UX Patterns v1

## 1. Project Overview

- **User brief:** Research UI/UX patterns and design considerations for a desktop voice-to-text dictation application targeting Windows first. The app should be beautiful, minimal, and efficient -- inspired by Linear.app's design aesthetic (dark-first, keyboard-first, clean, fast). This is a WisprFlow competitor.
- **Project type(s):** Product + Design
- **Research depth:** Deep dive
- **Primary research focus:** External best practices, reference implementations, and UX patterns

---

## 2. Existing Context & Assets (Internal)

### 2.1 Related Requirements & Docs
- No existing PRDs, CRDs, DRDs, or task files in `/tasks/` -- this is the first research document for this project.

### 2.2 Design System Context
- **`standards/domains/design-ui.md`** -- A comprehensive Linear-inspired design system is already defined, including:
  - Dark-first philosophy (bg-base: `#0A0A0B`, bg-surface: `#111113`, etc.)
  - Complete color token system (backgrounds, text, accents, borders)
  - Typography scale (Inter/SF Pro/system-ui, 14px body at 500 weight)
  - Spacing scale (4px base unit)
  - Component standards (buttons, inputs, cards, modals, sidebar)
  - Animation guidelines (100-150ms for micro-interactions, 200-300ms for UI transitions)
  - Keyboard-first philosophy with required shortcuts (Cmd/Ctrl+K command palette)
  - Glassmorphism accents (backdrop blur, transparency for overlays)
  - Glow effects on primary actions and active states

- **`standards/global/accessibility.md`** -- WCAG 2.1 Level AA target, keyboard navigation requirements, color contrast ratios, focus indicators, semantic HTML requirements.

- **`standards/global/principles.md`** -- User-first, quality over speed, simplicity, incremental delivery.

These existing standards provide a strong foundation and should be directly applied to the dictation app's UI.

---

## 3. User & Business Context

### 3.1 Target Users
- **Primary:** Knowledge workers who type extensively -- developers, writers, executives, customer support, legal professionals
- **Secondary:** Users with RSI (repetitive strain injury) or other physical conditions that make typing painful
- **Tertiary:** Accessibility users who rely on voice input as their primary computer interaction method
- **Platform focus:** Windows first (underserved market -- most polished dictation apps are macOS-only)

### 3.2 User Goals & Pain Points
- **Speed:** Voice is 3-5x faster than typing for many users; they want that speed advantage without friction
- **Accuracy:** Transcription errors destroy the speed advantage; users need high-accuracy, context-aware transcription
- **Seamlessness:** Users want text to appear in whatever app they are using, not in a separate editor
- **Invisibility:** The best dictation tool is one you forget is there -- it should not demand attention
- **Corrections:** When errors occur, fixing them must be fast and not break flow
- **Privacy:** Many users dictate sensitive content (emails, code, legal documents); on-device processing matters

### 3.3 Business Goals
- Compete with WisprFlow (macOS-only) by capturing the Windows market first
- Build a premium, paid utility ($10-20/month or $100-200 lifetime)
- Differentiate on design quality and Windows-native feel
- Build toward cross-platform (Windows first, macOS second)

### 3.4 Success Signals
- Users complete dictation sessions without touching the mouse
- Time-to-first-dictation under 30 seconds after install
- Dictation feels faster than typing for a new user within their first week
- Users describe the app as "beautiful" or "polished" unprompted

---

## 4. External Research: Dictation App UX Patterns

### 4.1 Reference App Analysis

#### WisprFlow (Primary Competitor)
- **Platform:** macOS only
- **Presentation:** Menu bar app with a small floating overlay
- **Activation:** Global hotkey (configurable, default is a double-tap of a modifier key or dedicated key). Push-to-hold and toggle modes both supported.
- **Visual feedback:** A small, minimal floating pill/bar appears near the cursor or at a fixed screen position showing a pulsing waveform indicator during recording, then briefly shows "processing" state
- **Text delivery:** Direct injection into the currently focused text field -- text appears as if typed. This is the key UX differentiator. Uses clipboard-based injection under the hood (copies to clipboard, pastes, restores original clipboard).
- **Transcription model:** Whisper-based (on-device and cloud options). Includes an LLM post-processing step for formatting, punctuation, and context-aware corrections.
- **Settings:** Minimal settings accessible from menu bar dropdown -- model selection, hotkey configuration, language, formatting preferences
- **Key insight:** WisprFlow's genius is its *invisibility*. It does not have a main window. It does not have a dedicated text editor. You speak, text appears where your cursor is. The entire UX is: hotkey -> speak -> text appears.

#### Superwhisper
- **Platform:** macOS only
- **Presentation:** Menu bar app with floating recording indicator
- **Activation:** Global hotkey, push-to-talk, or toggle mode
- **Visual feedback:** Floating capsule/pill showing recording state with waveform
- **Text delivery:** Direct injection into focused field (clipboard-paste method) or dedicated text area
- **Modes:** Multiple modes -- "Superwhisper" (AI-enhanced transcription), "Whisper" (raw transcription), "GPT" (AI rewriting of dictation)
- **Key insight:** Superwhisper differentiates with AI modes -- it does not just transcribe, it can reformat, summarize, or rewrite your dictation. This adds complexity but also value.

#### Talon Voice
- **Platform:** Cross-platform (Windows, macOS, Linux)
- **Presentation:** System tray with a small status indicator overlay
- **Activation:** Always-on voice command recognition with wake word / command grammar
- **Text delivery:** Direct keyboard simulation (not clipboard-based)
- **Key insight:** Talon is for power users and accessibility users who control their entire computer by voice. It is not a dictation app -- it is a voice operating system. Different product category, but its keyboard simulation approach to text delivery is worth studying.

#### macOS Dictation (Built-in)
- **Activation:** Double-tap Fn key or dedicated dictation key
- **Visual feedback:** Small microphone icon appears near text cursor
- **Text delivery:** Direct inline insertion with real-time streaming preview
- **Key insight:** Apple's approach of showing a subtle microphone icon near the cursor is the gold standard for "invisible" dictation UX. Text streams in real-time, which feels natural. However, accuracy lags behind dedicated tools.

#### Windows Voice Typing (Built-in, Win+H)
- **Activation:** Win+H keyboard shortcut
- **Visual feedback:** A floating toolbar appears at the top of the screen with a microphone icon
- **Text delivery:** Direct injection into focused field with real-time preview in the floating toolbar
- **Key insight:** The Windows built-in experience is functional but not polished. The floating toolbar is large and distracting. Accuracy is mediocre. This represents the baseline our app must exceed dramatically.

#### Dragon NaturallySpeaking (Nuance)
- **Presentation:** Full application window with dictation box, plus floating toolbar
- **Activation:** Microphone button or hotkey
- **Text delivery:** Dictation box editor or direct injection into target app
- **Key insight:** Dragon is the legacy incumbent. It is powerful but has a dated, complex UI. It proves the market exists but shows how NOT to design a modern dictation tool. Too many features exposed, too much chrome, too steep a learning curve.

### 4.2 Activation Patterns (Ranked by UX Quality)

| Pattern | Description | Pros | Cons | Best For |
|---------|-------------|------|------|----------|
| **Global hotkey (toggle)** | Press once to start, press again to stop | Simple mental model, hands-free during dictation | Must remember to stop, can accidentally leave recording | Default mode for most users |
| **Global hotkey (push-to-hold)** | Hold key to record, release to stop | Natural walkie-talkie feel, impossible to forget to stop | Ties up a key, some users find holding awkward for long dictation | Quick insertions, commands |
| **Double-tap modifier** | Double-tap Ctrl, Alt, or Fn | Discoverable, no dedicated key needed | Can conflict with other shortcuts, timing sensitivity | Secondary activation method |
| **Voice activation (wake word)** | Say a wake word to start dictation | Fully hands-free | False activations, privacy concerns, always listening | Accessibility-first users |
| **System tray click** | Click tray icon to toggle | Very discoverable | Requires mouse, defeats purpose | Fallback only |

**Recommendation:** Default to **global hotkey toggle** (e.g., `Ctrl+Shift+Space` or a configurable key) with **push-to-hold** as a secondary mode. Allow users to choose. WisprFlow's approach of making the hotkey the *entire interaction surface* is the right model.

### 4.3 Visual Feedback Patterns During Dictation

#### The Floating Pill / Capsule (Recommended)
The dominant pattern among modern dictation apps is a small, floating pill-shaped indicator:
- **Size:** Approximately 200-300px wide, 40-60px tall
- **Position:** Near cursor, top-center of screen, or user-configurable
- **Content during recording:** Animated waveform or pulsing dot + "Listening..." text
- **Content during processing:** Spinner or progress indicator + "Processing..." text
- **Content on completion:** Brief checkmark or fade-out
- **Behavior:** Appears on activation, disappears shortly after text is delivered
- **Interaction:** Can be dragged to reposition; click to cancel

```
Recording state:
┌──────────────────────────────┐
│  ●  |||||||||||  Listening   │
└──────────────────────────────┘

Processing state:
┌──────────────────────────────┐
│  ◌  Processing...            │
└──────────────────────────────┘

Success state (brief flash, then fade):
┌──────────────────────────────┐
│  ✓  Done                     │
└──────────────────────────────┘
```

**Design application (using existing design tokens):**
- Background: `bg-elevated` (#18181B) with `backdrop-blur-sm` (glassmorphism per DESIGN-11)
- Border: `border-subtle` (#27272A) with 1px
- Text: `text-secondary` (#A1A1AA) for status labels
- Recording indicator: `accent-primary` (#6366F1) pulsing glow (per DESIGN-12)
- Border radius: 20-24px (pill shape)
- Shadow: subtle drop shadow for elevation
- Animation: 150ms ease-out for appear/disappear (per animation guidelines)

#### Audio Waveform Visualization
- Real-time waveform adds confidence that the app is hearing the user
- Should be subtle -- thin bars, muted colors, small amplitude
- Use `accent-primary` (#6366F1) for active waveform bars
- Bars should smoothly animate based on actual audio input levels
- When silent, bars should settle to a flat line (visual cue to speak)

#### Alternative: Cursor-Adjacent Indicator
- A tiny dot or icon that appears next to the text cursor in the target app
- Extremely minimal, nearly invisible
- Technically harder to implement on Windows (requires knowing cursor position in any app)
- Worth exploring as an advanced/optional mode

### 4.4 Text Delivery Methods

| Method | How It Works | Pros | Cons |
|--------|-------------|------|------|
| **Clipboard injection** | Copy text to clipboard, simulate Ctrl+V, restore original clipboard | Works in virtually any app, reliable | Overwrites clipboard temporarily, perceptible paste action |
| **Keyboard simulation** | Simulate individual keystrokes via OS input API | No clipboard interference, character-by-character feels like typing | Slow for long text, can trigger autocomplete/shortcuts in target apps |
| **Hybrid** | Keyboard simulation for short text, clipboard for long text | Best of both worlds | More complex to implement |
| **Accessibility API injection** | Use UI Automation APIs to set text directly | Clean, no side effects | Not universally supported, app-specific behavior |
| **Dedicated editor** | Show transcribed text in app's own editor, user copies manually | Full control over editing/correction UX | Extra step, breaks flow |

**Recommendation for Windows:**
1. **Primary: Clipboard injection** -- This is what WisprFlow uses and it works. The clipboard overwrite is a minor annoyance that can be mitigated by saving and restoring the clipboard contents.
2. **Secondary: Keyboard simulation** -- For short insertions (< 50 characters) to avoid clipboard disruption.
3. **Future: UI Automation API** -- Windows has a rich UI Automation framework that could enable direct text field manipulation without clipboard or keyboard simulation. This is a differentiator opportunity.

### 4.5 Text Delivery -- Real-Time Streaming vs. Batch

| Approach | Description | UX Feel | Technical Complexity |
|----------|-------------|---------|---------------------|
| **Batch (post-recording)** | User records full utterance, then entire text appears at once | Feels like a "paste" -- fast but disconnected from speech | Lower -- process after recording ends |
| **Streaming (real-time)** | Text appears word-by-word as user speaks | Feels magical, like the computer understands you in real-time | Higher -- requires streaming ASR, may need corrections as later context arrives |
| **Hybrid streaming** | Stream interim results to the floating pill preview, then batch-inject final result | Preview feels real-time, final result is clean | Medium -- stream to UI, batch to target app |

**Recommendation:** Start with **batch delivery** (simpler, more reliable) but show **real-time streaming preview in the floating pill**. This gives the user immediate feedback that their speech is being captured while ensuring clean final output. Evolve to full streaming injection as the product matures.

### 4.6 Settings & Configuration UX

Based on the reference apps, the settings for a dictation app should be:

#### Essential Settings (MVP)
- **Hotkey configuration:** Which key(s) activate dictation
- **Activation mode:** Toggle vs. push-to-hold
- **Language:** Primary dictation language
- **Transcription model:** Local vs. cloud, model size/speed tradeoff
- **Output formatting:** Punctuation auto-insertion, capitalization behavior
- **Launch at startup:** Auto-start with Windows

#### Important Settings (Post-MVP)
- **Floating indicator position:** Where the pill appears (near cursor, top-center, custom)
- **Sound effects:** Audio feedback on start/stop recording (optional beep)
- **AI formatting mode:** Raw transcription vs. AI-cleaned output
- **Custom vocabulary:** Domain-specific words, names, technical terms
- **Silence detection:** Auto-stop after N seconds of silence
- **Clipboard behavior:** Whether to save/restore clipboard during injection

#### Settings UX Pattern
- Accessible from system tray right-click menu or keyboard shortcut
- Single-page settings with logical grouping (not tabbed -- too few settings to warrant tabs initially)
- Use the existing design system's card/panel components
- Inline toggles and dropdowns, no modal dialogs
- Changes apply immediately (no "Save" button needed)
- Keyboard shortcut recording widget for hotkey configuration

```
Settings Layout:
┌──────────────────────────────────────────────┐
│  Settings                              ✕     │
│                                              │
│  ACTIVATION                                  │
│  ────────────────────────────────────────    │
│  Hotkey             [Ctrl+Shift+Space] ⟲     │
│  Mode               ○ Toggle  ● Hold         │
│                                              │
│  TRANSCRIPTION                               │
│  ────────────────────────────────────────    │
│  Language            [English (US)    ▾]     │
│  Model               ○ Fast  ● Balanced      │
│  AI Formatting       [━━━━●] On              │
│                                              │
│  APPEARANCE                                  │
│  ────────────────────────────────────────    │
│  Indicator Position  [Near cursor     ▾]     │
│  Sound Effects       [━━●━━] Off             │
│                                              │
│  SYSTEM                                      │
│  ────────────────────────────────────────    │
│  Launch at Startup   [━━━━●] On              │
│  Check for Updates   [━━━━●] On              │
│                                              │
└──────────────────────────────────────────────┘
```

### 4.7 Onboarding Flow for Voice Apps

Voice apps have unique onboarding challenges because:
1. Users must grant microphone permissions (OS-level prompt)
2. Users must learn the activation hotkey (muscle memory)
3. Users must trust the app with their voice data (privacy)
4. Users need to experience the "magic moment" quickly

**Recommended Onboarding Sequence:**

```
Step 1: Welcome + Value Prop (1 screen)
"Dictate anywhere on your computer. 3x faster than typing."
[Get Started]

Step 2: Microphone Permission (1 screen)
"We need microphone access to hear you."
[Grant Access] -- triggers OS permission dialog
Audio privacy statement: "Audio is processed on-device. We never store recordings."

Step 3: Choose Your Hotkey (1 screen)
"Pick how you'll activate dictation."
[Record shortcut] or [Use default: Ctrl+Shift+Space]
Show the hotkey prominently with visual emphasis

Step 4: Try It Now (1 screen -- the magic moment)
"Press [your hotkey] and say something."
Show the floating pill with live waveform
After successful dictation, show the transcribed text with celebration
"You just dictated your first text! It's that simple."

Step 5: Where to Find Us (1 screen)
Show the system tray icon location
"We'll live here, ready when you need us."
[Start Using]
```

**Key onboarding principles:**
- Maximum 5 screens, completable in under 60 seconds
- Get to the "magic moment" (first successful dictation) as fast as possible
- Do not overwhelm with settings -- sane defaults first
- Make the hotkey the hero of onboarding -- if users forget the hotkey, the app is useless
- Show the hotkey in a persistent tooltip from the system tray for the first few days

---

## 5. External Research: Minimal Desktop Utility Design

### 5.1 System Tray / Menu Bar App Patterns

The system tray (Windows) / menu bar (macOS) is the natural home for utility apps that run in the background. Key patterns:

**The Tray Icon as Status Indicator:**
- The icon itself communicates state (idle, listening, processing, error)
- Use distinct icon variants: default (outline), active (filled/colored), error (red dot badge)
- On Windows, the system tray supports tooltip on hover -- use this for status text

**The Tray Menu (Right-Click):**
```
┌───────────────────────────┐
│  Dictation is ready       │
│  ─────────────────────    │
│  Start Dictation   Ctrl+… │
│  ─────────────────────    │
│  Settings          Ctrl+, │
│  History                  │
│  ─────────────────────    │
│  Check for Updates        │
│  About                    │
│  ─────────────────────    │
│  Quit                     │
└───────────────────────────┘
```

**Design considerations for Windows system tray:**
- Windows 11 has a redesigned system tray ("notification area") with a popover for overflow icons
- Users must "pin" tray icons to keep them visible -- the app should prompt or guide users to pin it
- Tray icons are 16x16 or 32x32px -- must be crisp and recognizable at small sizes
- The tray context menu should follow Windows 11 design language (rounded corners, Segoe UI font) but can be custom-rendered for brand consistency

### 5.2 Floating Overlay / Widget Patterns

**Raycast Pattern (Command Palette Overlay):**
- Centered on screen, overlays everything
- Backdrop dimming/blur
- Single search input, results below
- Keyboard-driven (arrow keys to navigate, Enter to select, Escape to dismiss)
- Appears/disappears with a hotkey (fast, no animation lag)
- This pattern is ideal for a "dictation command mode" but too heavy for the recording indicator

**CleanShot X Pattern (Floating Utility):**
- Small floating window that can be repositioned
- Stays on top of other windows ("always on top")
- Minimal chrome -- no title bar, just content
- Can be dragged by clicking anywhere on it
- Disappears automatically after use or on Escape
- This pattern is the right model for the dictation recording indicator

**Spotlight / Alfred Pattern:**
- Centered input bar
- Results appear below as you type/speak
- Purely ephemeral -- appears on demand, vanishes when done
- No "window" feel -- more like a HUD element

**Recommended Floating Widget Design (Dictation Pill):**
- **Position:** Default top-center of screen, 60px from top edge. User-configurable.
- **Size:** 240-320px wide, 48-56px tall (compact enough not to obscure content)
- **Behavior:** Always-on-top, click-through except for the pill itself, no taskbar entry
- **Window style:** No title bar, no window chrome. Transparent background with the pill rendered on a transparent canvas.
- **Shadow:** Subtle shadow for depth (elevated feel without heavy chrome)
- **Drag:** Click-and-drag to reposition. Position persists across sessions.
- **Dismiss:** Escape key or clicking outside. Auto-dismiss 1.5 seconds after text delivery.

### 5.3 Making a Utility App Feel Premium

Based on analysis of what makes Linear, Raycast, CleanShot X, and Arc feel premium:

1. **Speed is the feature.** Every interaction must feel instant. Cold start under 2 seconds. Hotkey response under 100ms. No loading spinners for common operations.

2. **Restraint over features.** Expose only what the user needs right now. Progressive disclosure for power features. Settings should be simple by default, powerful on demand.

3. **Attention to micro-details:**
   - Smooth, consistent animations (not flashy -- purposeful)
   - Proper hover states on every interactive element
   - Subtle transitions between states (idle -> recording -> processing -> done)
   - Sound design (optional, tasteful audio feedback)
   - Correct DPI/scaling behavior on all monitor configurations

4. **Native feel:** On Windows, this means:
   - Respecting the system accent color (or offering it as an option)
   - Proper multi-monitor support (floating pill appears on the active monitor)
   - Correct behavior with virtual desktops
   - Following Windows 11 rounded corner conventions (8px border radius)
   - Respecting system dark/light mode preference

5. **Typography as a design element:**
   - Use the Inter font (or system font stack) as specified in the design standards
   - Proper font rendering (ClearType on Windows)
   - Generous line height and letter spacing
   - Text should feel "comfortable" even in tiny UI elements

### 5.4 Dark Mode Design for Small Utility Windows

Small windows on dark backgrounds have specific challenges:

- **Elevation is critical:** Without borders and backgrounds to delineate, small windows can feel "lost" against dark desktop wallpapers. Use subtle shadows and slight background elevation.
- **Avoid pure black (#000000):** As specified in the design tokens (bg-base: `#0A0A0B`), use very dark gray instead. Pure black creates harsh contrast with text and looks flat.
- **Border treatment:** A 1px border in `border-subtle` (#27272A) helps the window "pop" from the desktop without looking heavy.
- **Glow effects shine in dark mode:** The accent glow (DESIGN-12) on the recording indicator creates a beautiful, eye-catching effect that feels high-tech and premium. A soft indigo glow around the pill during recording is the signature visual.
- **Text weight matters more:** As noted in DESIGN-7, use medium weight (500) for body text on dark backgrounds. Light text on dark backgrounds needs slightly heavier weight to maintain legibility.
- **Test against various wallpapers:** The floating pill must look good against light wallpapers, dark wallpapers, and busy/photographic wallpapers. The backdrop blur + border combination handles this well.

---

## 6. External Research: Accessibility Considerations

### 6.1 Voice-to-Text as an Accessibility Tool

A dictation app is inherently an assistive technology. This carries specific responsibilities:

- **It may be a user's primary or only input method.** If a user has RSI, paralysis, or motor impairment, they cannot "just type" if the dictation app fails. Reliability is not a feature -- it is an accessibility requirement.
- **Error recovery must not require fine motor control.** If a user dictates incorrectly, the correction mechanism must be usable without precise mouse targeting.
- **The app must be fully operable via keyboard and, ideally, via voice commands.** Settings, corrections, and all functions should have keyboard paths.

### 6.2 Visual Indicators for Audio State

Users who rely on dictation need unmistakable clarity about what the app is doing:

| State | Visual Indicator | Audio Indicator (optional) | Color |
|-------|-----------------|---------------------------|-------|
| **Idle** | Static tray icon, pill hidden | None | Default |
| **Listening** | Animated waveform in pill, pulsing tray icon | Subtle "start" tone | `accent-primary` (#6366F1) with glow |
| **Processing** | Spinner in pill, tray icon shows processing | None | `text-secondary` (#A1A1AA) |
| **Success** | Checkmark in pill (brief), tray icon returns to idle | Subtle "success" tone | `accent-success` (#22C55E) |
| **Error** | Error icon in pill with message, tray icon shows error badge | Error tone | `accent-error` (#EF4444) |
| **No audio detected** | Waveform flatlines, "Can't hear you" message | None | `accent-warning` (#F59E0B) |

Key accessibility requirements for these indicators:
- States must be distinguishable by shape/icon, not just color (colorblind users)
- States must be communicable via screen reader announcements (ARIA live regions)
- The floating pill should have sufficient contrast against any desktop background
- Optional audio feedback allows low-vision users to confirm state without looking

### 6.3 Keyboard Navigation Requirements

Per ACCESS-2 (all interactive elements must be keyboard accessible):

- **Activation/deactivation:** Global hotkey (the primary interaction is already keyboard-driven)
- **Settings navigation:** Full Tab/Shift+Tab navigation, Enter to activate, Escape to close
- **Tray menu:** Keyboard-navigable context menu (standard on Windows)
- **Floating pill:** Escape to cancel recording, no other keyboard interaction needed (it is a display element)
- **Future voice commands:** "Cancel," "undo," "stop" as voice-based controls during dictation

### 6.4 Customization Needs

Accessibility users often need customization beyond what typical users expect:

- **Font size:** Allow scaling the floating pill text (at minimum, respect system DPI settings)
- **Pill position:** Must be movable -- it could obstruct critical UI elements in accessibility tools
- **Colors:** Respect Windows high contrast mode; offer a high-contrast variant
- **Timing:** Some users need longer auto-dismiss timers; some need the pill to persist until manually dismissed
- **Audio feedback:** Make all audio cues optional and configurable (volume, on/off per event)

---

## 7. External Research: Key UX Decisions

### 7.1 Always-On vs. On-Demand

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **On-demand (recommended)** | User presses hotkey to start dictation | No accidental recording, privacy-safe, simple | User must learn and remember the hotkey |
| **Always-on with wake word** | App continuously listens for a wake word | Fully hands-free | Privacy concern, battery drain, false activations, higher CPU usage |
| **Hybrid** | On-demand by default, always-on as opt-in setting | Flexibility | Complexity in UI/settings |

**Recommendation:** Start **on-demand only**. Always-on is a privacy minefield and technically complex. The hotkey-based approach is proven by WisprFlow and is simpler to implement, test, and trust.

### 7.2 Transcription Preview Location

| Location | Description | Pros | Cons |
|----------|-------------|------|------|
| **In the floating pill** | Text streams into the pill as the user speaks | Immediate feedback, no context switching | Pill must expand, can obscure content |
| **Inline in target app** | Text appears directly in the text field as the user speaks | Most natural, like typing | Technically hard, streaming required, errors visible in target app |
| **No preview (batch)** | Text appears only after dictation ends | Simplest, cleanest | User has no feedback during recording |
| **Expandable pill with preview** | Pill shows waveform during recording, expands to show text during processing | Clean during recording, informative during processing | More complex animation |

**Recommendation:** Use the **expandable pill** approach:
1. During recording: Compact pill with waveform (user focuses on speaking, not reading)
2. During processing: Pill expands slightly to show "Processing..." or interim text
3. After processing: Brief flash of the final text in the pill, then inject into target app
4. The pill auto-dismisses after injection

This avoids the complexity of real-time inline injection while giving users confidence their speech was captured.

### 7.3 Handling Corrections and Editing

Corrections are the Achilles' heel of dictation apps. Options:

**Option A: No correction UI (inject and let user edit in target app)**
- Simplest implementation
- User makes corrections using their target app's native editing tools
- Works well when accuracy is high
- WisprFlow's current approach

**Option B: Preview-and-edit before injection**
- After transcription, show the text in a small editor overlay
- User can edit before pressing Enter to inject
- Adds friction to every dictation
- Useful for high-stakes dictation (emails to CEO, legal text)

**Option C: Quick-redo voice command**
- User says "redo" or presses a hotkey to re-record the last dictation
- Deletes the previously injected text and starts a new recording
- Low friction, handles the "that came out wrong" case

**Recommendation:** Start with **Option A** (no correction UI) combined with **Option C** (quick redo). High accuracy + easy redo covers 95% of cases. Add Option B as an optional "review mode" in settings for users who want it.

### 7.4 Multi-Language Support UX

- **Primary language setting:** Set once in settings, used for all dictation
- **Language switching:** Could support a hotkey to cycle languages or a modifier key (e.g., `Ctrl+Shift+Space` for English, `Ctrl+Alt+Space` for Spanish)
- **Auto-detection:** Some Whisper models can auto-detect language. If used, show detected language in the pill as confirmation.
- **Important:** Do not overwhelm MVP. Ship with English first, add languages incrementally. The Whisper model already supports 90+ languages -- this is primarily a UX/settings problem, not a technical one.

### 7.5 Noise and Silence Detection Feedback

- **Silence detection:** If the user activates dictation but does not speak for 3-5 seconds, show a gentle prompt: "Waiting for you to speak..." with the waveform flatlined
- **Auto-stop on silence:** After extended silence (configurable, default 5-8 seconds), automatically end recording and process whatever was captured
- **Background noise:** If the audio contains high noise levels, show a warning: "Noisy environment detected" with suggestions (move to quieter space, check microphone)
- **No audio input:** If no microphone is detected or it is muted, show an error state immediately: "No microphone detected" with a link to system audio settings

### 7.6 Confidence Indicators

- **Not recommended for MVP.** Confidence indicators (underlining uncertain words, showing percentage scores) add visual noise and anxiety. If the transcription is wrong, the user will see it.
- **Future consideration:** Highlight low-confidence words with a subtle underline in the preview-and-edit mode (Option B above). This is a power-user feature.

---

## 8. External Research: Design Inspiration Deep Dives

### 8.1 Linear.app -- Design Principles to Apply

Linear's design excellence comes from several principles directly applicable to a dictation app:

1. **Speed as brand identity:** Linear is obsessed with perceived speed. Every interaction feels instant. For the dictation app: hotkey response must be sub-100ms, the pill must appear instantly, text injection must feel instantaneous.

2. **Muted palette with strategic color:** Linear uses a largely monochromatic dark palette with color reserved for status and interactive elements. For the dictation app: the pill should be mostly gray/dark with the indigo accent appearing only during active recording (the glow effect).

3. **Subtle depth through layering:** Linear uses subtle background color shifts to create visual hierarchy without borders. For the dictation app: the pill floats above the desktop using elevation (shadow + slightly lighter background + blur).

4. **Typography is information design:** Linear uses typographic hierarchy (weight, size, color) instead of visual chrome (borders, icons, labels). For the dictation app: the pill's state can be communicated through text weight and color changes rather than adding icons for every state.

5. **Keyboard shortcuts everywhere:** Linear shows keyboard shortcuts inline and makes them discoverable through the command palette. For the dictation app: the system tray menu should show keyboard shortcuts, and the onboarding should make the primary hotkey unforgettable.

### 8.2 Raycast -- Command Palette and Floating UI

Relevant patterns from Raycast:

1. **The "bar" as the primary interface:** Raycast's entire product is a floating bar. No main window. This maps directly to the dictation pill.

2. **Instant appearance:** Raycast appears in under 50ms when the hotkey is pressed. No fade-in, no slide-down. It just appears. The dictation pill should follow this pattern -- instant appearance, no animation delay.

3. **Contextual expansion:** Raycast's bar expands downward to show results. The dictation pill could expand to show transcription preview in a similar way.

4. **Minimal footprint:** Raycast avoids having a Dock icon (macOS). The dictation app should avoid having a taskbar entry (Windows equivalent). It lives in the system tray only.

### 8.3 CleanShot X -- Floating Utility Overlay

Relevant patterns from CleanShot X:

1. **Floating overlay that "just works":** CleanShot's annotation overlay appears over everything, handles multi-monitor correctly, and is draggable. The dictation pill needs the same behavior.

2. **Auto-dismiss with grace:** After capturing a screenshot, CleanShot shows a floating preview that auto-dismisses. Users can interact with it if they need to, but it gets out of the way automatically. The dictation pill should auto-dismiss after text delivery with a similar grace period.

3. **State transitions:** CleanShot moves smoothly through states (capturing -> editing -> saved) with clear visual transitions. The dictation pill moves through (idle -> recording -> processing -> delivered) and should be equally clear.

### 8.4 Arc Browser -- Innovative UI Patterns

Relevant patterns from Arc:

1. **Split personality:** Arc has a full browser window AND a floating "Little Arc" mini-window for quick tasks. The dictation app has a similar split: the settings/configuration window is the "full" experience, the floating pill is the "little" experience for actual use.

2. **Design as differentiator in a commodity market:** Browsers are a commodity, but Arc carved out a market through design alone. Dictation apps are heading toward commodity (Whisper is open-source, the tech is available to everyone). Design and polish are the differentiator.

3. **Opinionated defaults:** Arc makes bold default choices (no visible URL bar, no bookmarks bar) that force users to adapt but result in a better experience. The dictation app should make opinionated defaults (toggle mode, auto-format on, indicator near cursor) rather than making users configure everything.

---

## 9. Constraints, Risks, and Dependencies

### 9.1 Technical Constraints (Windows-Specific)

- **System tray behavior:** Windows 11 collapses tray icons into an overflow area by default. The app needs to guide users to pin the icon, or rely purely on the hotkey (no tray icon visibility needed for core functionality).
- **Always-on-top windows:** Windows allows always-on-top windows via the `HWND_TOPMOST` flag, but some fullscreen apps (games, presentations) may occlude it. Need to handle fullscreen detection.
- **Clipboard management:** The Windows clipboard API is well-documented but has quirks with rich text formats. Must handle clipboard save/restore cleanly.
- **Global hotkeys:** Windows supports global hotkeys via `RegisterHotKey` API. Must handle conflicts gracefully (show error if hotkey is already taken).
- **DPI scaling:** Windows supports multiple DPI scales across monitors. The floating pill must render correctly on all monitors and handle moves between monitors with different DPI scales.
- **Audio capture:** Windows Audio Session API (WASAPI) or Windows.Media.Capture for microphone access. Need to handle default device changes, device disconnection, and permissions.

### 9.2 Risks

- **Risk: Clipboard disruption annoys users.** Mitigation: Fast save/restore of clipboard, and explore UI Automation API as an alternative text delivery mechanism.
- **Risk: Hotkey conflicts with other apps.** Mitigation: Detect conflicts, offer alternative suggestions, allow full customization.
- **Risk: Floating pill positioned poorly.** Mitigation: Smart default positioning (center-top), easy drag-to-reposition, remember position per monitor.
- **Risk: Perceived slowness if transcription takes >2 seconds.** Mitigation: Use streaming preview in pill to show progress, optimize model loading (keep warm in background).
- **Risk: Accessibility users find the UI insufficient.** Mitigation: Engage accessibility testers early, support high contrast mode, ensure screen reader compatibility.

### 9.3 Dependencies

- **Whisper model (or equivalent):** Need to decide on local vs. cloud transcription. Local requires bundling the model (~200MB-1.5GB depending on size). Cloud requires API keys and internet.
- **LLM for formatting:** If using AI-enhanced formatting (like WisprFlow), need an LLM API or local model.
- **Windows app framework:** Electron/Tauri/WPF/.NET MAUI for the application shell. Affects performance, native feel, and development speed.

---

## 10. Opportunities & Ideas

### 10.1 Reuse Opportunities
- The existing design token system (`design-ui.md`) provides a complete visual language -- no need to design a color system or typography scale from scratch
- The accessibility standards (`accessibility.md`) provide clear compliance targets

### 10.2 Quick Wins
- **Windows market gap:** WisprFlow and Superwhisper are macOS-only. Being the first polished dictation app on Windows is a significant first-mover advantage.
- **System tray + hotkey + floating pill** is a simple architecture that can ship fast with high polish

### 10.3 Differentiation Ideas
- **"Edit with voice" mode:** After dictation, allow voice commands to edit the text ("replace X with Y," "capitalize that," "add a period"). This goes beyond transcription into voice-controlled editing.
- **Context-aware formatting:** Detect the target app (email client, code editor, chat app) and format accordingly (formal tone for email, markdown for docs, no punctuation for search bars).
- **Dictation history:** A searchable history of everything you have dictated, accessible via hotkey. Useful for finding that thing you said earlier.
- **Snippets/templates:** Voice-triggered templates ("boilerplate email response," "meeting notes header"). Activated by a command prefix during dictation.
- **Per-app profiles:** Different settings (language, formatting, AI mode) depending on which app is focused when dictation starts.

### 10.4 Future Extensions
- macOS version (after Windows is proven)
- Browser extension for web-app-specific integration
- API for third-party integrations
- Team/enterprise features (shared vocabulary, compliance logging)
- Real-time translation (dictate in one language, output in another)

---

## 11. Key Findings Summary

### 11.1 Product / Feature Findings
1. **WisprFlow's "invisible" UX is the gold standard:** No main window, no editor, just hotkey -> speak -> text appears. This should be the core interaction model.
2. **Clipboard injection is the pragmatic text delivery method for Windows**, with UI Automation as a future differentiator.
3. **On-demand activation via global hotkey is the right default.** Always-on voice activation is a privacy and complexity trap for MVP.
4. **Batch transcription with streaming preview** balances simplicity and user confidence.
5. **The Windows market is wide open** -- no polished, modern, WisprFlow-quality dictation app exists for Windows.

### 11.2 Design Findings
1. **The floating pill is the product's face** -- it must be beautiful, minimal, and unmistakably clear in its state communication.
2. **The Linear design system already in `/standards/`** maps perfectly to this app. Dark-first, glassmorphism for the pill, indigo accent glow during recording, muted colors for everything else.
3. **Speed of appearance is critical.** The pill must appear in under 100ms. No slide-in animation -- instant appear with a subtle fade or scale-up (150ms max).
4. **The system tray is the app's home base**, but the hotkey is the primary interaction. Many users may never interact with the tray icon directly.
5. **Premium feel comes from restraint, speed, and micro-details** -- not from features or visual complexity.

### 11.3 Accessibility Findings
1. **The app is an accessibility tool** whether the user identifies as disabled or not. Design accordingly from day one.
2. **State communication must be multi-modal:** visual (color + shape + animation), optional audio (tones), and screen reader (ARIA announcements).
3. **Keyboard-first design aligns with accessibility requirements** -- the hotkey-driven interaction model is inherently accessible.
4. **Customization of pill position, size, and timing** is an accessibility need, not just a preference.

---

## 12. Recommendations for the Create Phase

### 12.1 Recommended Requirements Documents
- **Create next:** PRD (Product Requirements Document) for the dictation app MVP
- **Suggested filename:** `prd-dictation-mvp-v1.md`
- **Follow-up:** DRD (Design Requirements Document) for the floating pill and settings UI
- **Suggested filename:** `drd-dictation-ui-v1.md`

### 12.2 Scope Recommendations

**MVP Scope (Must Have):**
1. System tray application that runs in background on Windows
2. Global hotkey activation (configurable) with toggle and hold modes
3. Floating pill indicator with recording/processing/done states
4. Audio recording and transcription (Whisper-based, local or cloud)
5. Clipboard-based text injection into any focused text field
6. Basic settings: hotkey, activation mode, language, launch at startup
7. Onboarding flow (5 screens, microphone permission, hotkey setup, first dictation)
8. Dark mode UI following existing design tokens

**Post-MVP / Stretch:**
1. AI-enhanced formatting (LLM post-processing for punctuation, capitalization, context-aware cleanup)
2. Streaming transcription preview in the floating pill
3. Quick-redo via hotkey (delete last dictation and re-record)
4. Dictation history with search
5. Custom vocabulary / domain-specific terms
6. Multiple language support
7. Context-aware formatting (detect target app type)
8. Light mode theme
9. Sound effects for recording start/stop
10. Voice command corrections ("replace X with Y")

### 12.3 Key Questions the Requirements Doc Should Answer
1. **What app framework for Windows?** Electron (fast to build, heavy) vs. Tauri (lighter, Rust backend) vs. native WPF/.NET (most native feel, slowest to build)?
2. **Local vs. cloud transcription?** Local Whisper (privacy, offline, larger download) vs. cloud API (lighter, faster for large models, requires internet)?
3. **Business model?** Free tier with cloud limits + paid? One-time purchase? Subscription?
4. **How to handle the clipboard save/restore?** What edge cases exist (rich text, images, large clipboard contents)?
5. **What is the minimum viable onboarding?** Can we ship with just a hotkey setup and skip the full 5-step flow initially?

### 12.4 Suggested Decisions to Lock In Now
- **Decision 1:** Target Windows 10/11 only (no Windows 7/8 support)
- **Decision 2:** Dark mode first, light mode deferred to post-MVP
- **Decision 3:** Global hotkey (Ctrl+Shift+Space default) as the only activation method for MVP (no wake word, no tray click-to-record)
- **Decision 4:** Clipboard injection as the text delivery method for MVP
- **Decision 5:** Use the existing Linear-inspired design system from `standards/domains/design-ui.md` as the visual foundation

---

## 13. Open Questions & Gaps

- **User testing data:** No actual user testing has been conducted. The onboarding flow and pill design should be validated with real users before finalizing.
- **Windows-specific technical spikes needed:** Clipboard save/restore reliability, global hotkey registration edge cases, always-on-top behavior in fullscreen apps, multi-monitor DPI handling.
- **Competitive pricing analysis:** What do users pay for WisprFlow, Superwhisper, and Dragon? What is the willingness to pay for a Windows-specific tool?
- **Audio processing latency benchmarks:** How fast can local Whisper process a typical 10-second utterance on mid-range Windows hardware? This directly affects UX.
- **Accessibility testing plan:** Need to define how and when to test with screen readers (NVDA, JAWS) and with actual accessibility users.

---

## 14. Sources & References

- WisprFlow (https://wisprflow.com) -- Primary competitor, macOS dictation app
- Superwhisper (https://superwhisper.com) -- macOS dictation app with AI modes
- Talon Voice (https://talonvoice.com) -- Cross-platform voice control system
- Linear (https://linear.app) -- Design aesthetic inspiration
- Raycast (https://raycast.com) -- Command palette and floating UI patterns
- CleanShot X (https://cleanshot.com) -- Floating overlay utility pattern
- Arc Browser (https://arc.net) -- Innovative UI patterns
- OpenAI Whisper (https://github.com/openai/whisper) -- Open-source speech recognition model
- Windows UI Automation (https://learn.microsoft.com/en-us/windows/win32/winauto/entry-uiauto-win32) -- Windows accessibility/automation API
- WCAG 2.1 Guidelines (https://www.w3.org/WAI/WCAG21/quickref/) -- Accessibility standards
- Internal: `standards/domains/design-ui.md` -- Existing Linear-inspired design system
- Internal: `standards/global/accessibility.md` -- Existing accessibility standards
- Internal: `standards/global/principles.md` -- Core design principles
