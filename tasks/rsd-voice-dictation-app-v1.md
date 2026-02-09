# Research Summary Document (RSD): Voice Dictation App v1

## 1. Project Overview

- **User brief:** Create a cross-platform voice-to-text dictation app that competes with WisprFlow. Beautiful, simple, accurate, and efficient. Windows first, then macOS, then Linux.
- **Project type(s):** Product + Design
- **Research depth:** Moderate
- **Primary research focus:** Balanced (external landscape + technical architecture)

---

## 2. Existing Context & Assets (Internal)

### 2.1 Related Requirements & Docs
- No existing requirements docs, task files, or prior work in `/tasks/` -- this is an entirely new project.

### 2.2 Codebase / System Context
- No existing codebase. Starting from scratch.
- The team's established standards (`/standards/`) define a **Next.js + TypeScript + Tailwind + shadcn/ui + Radix** stack, which informs the frontend framework choice for the desktop app.
- Design standards (`/standards/domains/design-ui.md`) define a **Linear-inspired, dark-first, keyboard-first** aesthetic with specific color tokens, typography scales, spacing, and component patterns that should carry directly into this product.

### 2.3 Detailed Research Sub-Documents
The following detailed research files were generated during this research phase and are available for deep dives:
- `tasks/rsd-dictation-app-v1.md` -- Competitor analysis, market gaps, pricing strategy
- `tasks/rsd-desktop-framework-v1.md` -- Framework comparison (7 frameworks, scored)
- `tasks/rsd-dictation-ui-ux-v1.md` -- UX patterns, interaction design, onboarding

---

## 3. User & Business Context

### Target Users
- **Primary:** Knowledge workers who write frequently (emails, documents, messages, code comments) and want to dictate instead of type for speed and comfort.
- **Secondary:** Users with RSI, carpal tunnel, or motor impairments who need voice as a primary input method.
- **Tertiary:** Multilingual users who want accurate dictation across languages.

### User Goals & Pain Points
- **Speed:** Dictation is 3x faster than typing for most people.
- **Comfort:** Reduce repetitive strain from extended typing sessions.
- **Accuracy:** Transcription must be good enough that corrections are rare (>95% accuracy baseline).
- **Invisibility:** The tool should stay out of the way -- no complex UI to manage, no workflow disruption.
- **Privacy:** Many users want dictation processed locally, never sent to the cloud.

### Business Goals
- Capture the **Windows dictation market** where no premium AI-enhanced solution exists.
- Build cross-platform to expand TAM (Windows > Mac > Linux).
- Differentiate on design quality, local-first privacy, and AI-enhanced formatting.

### Success Signals
- Users adopt the hotkey as a natural part of their workflow within the first week.
- Transcription accuracy is perceived as "good enough" that corrections are infrequent.
- The app feels lightweight and premium -- users recommend it for the experience, not just the function.

---

## 4. External Research: Competitive Landscape

### 4.1 WisprFlow (Primary Competitor)

| Attribute | Detail |
|-----------|--------|
| **Platforms** | macOS only |
| **STT approach** | Cloud-based (proprietary models) |
| **Key feature** | LLM post-processing for grammar, formatting, and context-aware cleanup |
| **Pricing** | ~$8-10/month subscription |
| **UX model** | Menu bar app, floating pill indicator, global hotkey activation |
| **Text delivery** | Clipboard injection (save clipboard, paste, restore) |
| **What users love** | AI text cleanup, system-wide integration, speed, simplicity |
| **What users complain about** | No Windows, cloud dependency, occasional over-correction, no offline mode |

**Key insight:** WisprFlow's moat is not raw transcription quality -- it's the LLM formatting layer that turns messy speech into polished text, plus the invisible, frictionless UX.

### 4.2 Competitor Matrix

| Product | Platforms | STT Location | AI Cleanup | Offline | Price | System-Wide |
|---------|-----------|-------------|------------|---------|-------|-------------|
| **WisprFlow** | macOS | Cloud | Yes (LLM) | No | ~$8-10/mo | Yes |
| **Superwhisper** | macOS | Local (Whisper) | Yes (GPT modes) | Partial | ~$8/mo | Yes |
| **Talon Voice** | Win/Mac/Linux | Local | No | Yes | Free | Yes (commands) |
| **Dragon** | Windows | Local | No | Yes | $200+ one-time | Yes |
| **Windows Voice Typing** | Windows | Cloud | No | No | Free | Partial |
| **macOS Dictation** | macOS | Hybrid | No | Partial | Free | Yes |
| **Otter.ai** | Web/Mobile | Cloud | No | No | Freemium | No |

### 4.3 Market Gaps -- The Opportunity

**The #1 finding: No premium AI-enhanced dictation tool exists for Windows.** Users on Windows are stuck with either:
- Windows Voice Typing (basic, cloud-only, no AI cleanup)
- Dragon (expensive, legacy UX, being sunset by Microsoft)
- Talon Voice (powerful but targets developers/power users with a steep learning curve)

**Consistently requested features that nobody delivers well:**
1. Beautiful, modern UI on Windows (every existing option looks dated or minimal)
2. Local/offline processing with cloud-grade accuracy
3. AI-powered text cleanup (WisprFlow has this, but macOS only)
4. Cross-platform with a single purchase/subscription
5. Lifetime license option (vs. recurring subscription fatigue)
6. Context-aware formatting (adapt output to target app)
7. Custom vocabulary for domain-specific terms
8. Quick redo/correction via voice without touching keyboard
9. Dictation history with search
10. Low resource usage for a background utility

---

## 5. External Research: Speech-to-Text Engines

### 5.1 Local STT Engine Comparison

| Engine | Accuracy (English) | Real-time Streaming | Latency | Min RAM | GPU Required | Cross-Platform | License |
|--------|-------------------|-------------------|---------|---------|-------------|---------------|---------|
| **whisper.cpp** | Excellent (WER ~4-8%) | Pseudo (1-2s chunks) | 1-3 sec | ~200 MB (tiny) | No | Excellent (incl. Apple Silicon) | MIT |
| **faster-whisper** | Excellent (WER ~4-8%) | Pseudo (1-2s chunks) | 1-3 sec | ~500 MB | Recommended | Windows/Linux (no Mac GPU) | MIT |
| **Vosk** | Good (WER ~10-15%) | Native (<200ms) | <200 ms | ~200 MB | No | Excellent | Apache 2.0 |
| **Silero STT** | Moderate (WER ~8-12%) | Yes | <500 ms | ~300 MB | No | Good | MIT |
| **NeMo Parakeet** | Excellent (WER ~4-6%) | Native (RNNT) | <500 ms | ~2 GB | Yes (NVIDIA) | Limited | Apache 2.0 |

**Key findings:**
- **whisper.cpp is the strongest foundation for local STT.** Zero runtime dependencies (pure C/C++), MIT licensed, runs on all platforms, Apple Silicon optimized via Metal/ANE, and all Whisper model sizes supported. The `base.en` model provides the best latency/accuracy trade-off for English dictation.
- **Vosk offers the only true real-time streaming** with sub-200ms partial results, but accuracy is notably lower than Whisper-class models.
- **Whisper is architecturally batch-oriented** -- true word-by-word streaming is not possible. Pseudo-streaming via chunked processing (1-2 second segments) with VAD is the practical approach.
- **Silero VAD** (voice activity detection) is the de facto standard for detecting when speech starts/stops and should be used regardless of which STT engine is chosen.

### 5.2 Cloud STT Engine Comparison

| Service | Accuracy | Streaming | Latency | Price/min | Dictation-Ready |
|---------|----------|-----------|---------|-----------|----------------|
| **Deepgram** | Excellent | Yes (WebSocket) | <300 ms | $0.0043 | Yes (smart formatting) |
| **Azure Speech** | Very Good | Yes (SDK) | <200 ms | $0.0167 | Yes (built-in dictation mode) |
| **Google STT** | Very Good | Yes (gRPC) | <500 ms | $0.024 | Partial |
| **AssemblyAI** | Excellent | Yes (WebSocket) | <350 ms | $0.005 | Yes |
| **OpenAI Whisper API** | Excellent | **No** (batch only) | 3-8 sec | $0.006 | No |
| **AWS Transcribe** | Good | Yes (HTTP/2) | <500 ms | $0.024 | Partial |

**Key findings:**
- **Deepgram** is the best cloud option: best streaming accuracy, lowest price, clean WebSocket API.
- **Azure Speech** has the most "dictation-ready" SDK with continuous recognition mode, making it the easiest to integrate.
- **OpenAI Whisper API does NOT support streaming** -- unsuitable as a primary engine for real-time dictation.

### 5.3 Recommended STT Architecture

A **hybrid, tiered approach**:

```
Microphone → Silero VAD → whisper.cpp (local, base.en) → Display
                                    |
                          [optional, user-configured]
                                    |
                          Cloud STT (Deepgram/Azure) → Higher-accuracy correction
```

- **Tier 1 (Default, offline-capable):** whisper.cpp with `base.en` model, chunked streaming (1-2s segments), Silero VAD for speech detection. Runs on any modern machine without GPU.
- **Tier 2 (Cloud enhancement, optional):** Deepgram or Azure for users who want maximum accuracy and have internet. User provides their own API key or subscribes.
- **Tier 3 (Post-processing):** LLM-based formatting to clean up raw transcription (punctuation, capitalization, grammar). Tiered: rule-based (free), local LLM (mid-tier), cloud LLM (premium).

---

## 6. External Research: Desktop App Framework

### 6.1 Framework Comparison (Weighted Scores)

| Framework | Score /55 | Verdict |
|-----------|-----------|---------|
| **Tauri v2** | **48.5** | Primary recommendation |
| **Electron** | **48** | Runner-up (fastest to ship) |
| Wails | 41 | Honorable mention |
| Neutralinojs | 37.5 | Too limited |
| Qt | 37.5 | Wrong paradigm for web-tech team |
| Flutter Desktop | 32 | Not recommended |
| .NET MAUI | 32 | No Linux support -- disqualified |

### 6.2 Why Tauri v2 Wins (Narrowly)

The scores are nearly tied, but the tie-breakers favor Tauri for this specific app:

| Factor | Tauri v2 | Electron |
|--------|----------|----------|
| **App size** | 2-8 MB installer | 80-150 MB installer |
| **Idle memory** | 20-50 MB | 100-300 MB |
| **ML integration** | Rust links directly to whisper.cpp via `whisper-rs` crate | Requires N-API addons (works but indirect) |
| **Frontend** | React/TypeScript/Tailwind/shadcn/ui (identical to Electron) | React/TypeScript/Tailwind/shadcn/ui |
| **Backend language** | Rust (learning curve) | JavaScript/Node.js (familiar) |
| **Ecosystem maturity** | Newer, smaller ecosystem | Mature, large ecosystem |
| **Competitive validation** | No major dictation app uses Electron | No major dictation app uses Electron |

**The trade-off:** Tauri requires Rust for the backend (~30-40% of the codebase). The frontend (60-70%) remains entirely React/TypeScript/Tailwind/shadcn/ui. Existing Rust crates handle most heavy lifting:
- `whisper-rs` -- whisper.cpp bindings
- `cpal` -- cross-platform audio capture
- `enigo` -- keyboard simulation for text injection
- `tauri-plugin-global-shortcut` -- global hotkeys
- `tauri-plugin-autostart` -- launch on boot
- `tauri-plugin-updater` -- auto-updates

**Open question for the PRD:** Is the Rust learning curve acceptable, or should Electron be chosen for faster initial delivery? Both produce the same user experience.

### 6.3 Text Injection Strategy

How dictation apps insert text into other applications:

| Method | How It Works | Pros | Cons | Used By |
|--------|-------------|------|------|---------|
| **Clipboard injection** (recommended primary) | Save clipboard → set text → Ctrl+V → restore clipboard | Works in any app | Brief clipboard disruption | WisprFlow |
| **Keyboard simulation** (fallback) | Simulate keystrokes via OS API | No clipboard interference | Slow for long text, can trigger autocomplete | Talon Voice |
| **Accessibility API** (future) | Set text field value via UI Automation | Clean, no side effects | Not universally supported | Superwhisper |
| **Input Method Framework** (complex) | Register as OS input method | Most native | Extremely complex to implement | Windows Voice Typing |

**Recommendation:** Clipboard injection for MVP, keyboard simulation as fallback for short text, accessibility API as a future differentiator.

---

## 7. External Research: UI/UX Patterns

### 7.1 The Proven Dictation UX Model

The dominant pattern among modern dictation apps (WisprFlow, Superwhisper, macOS Dictation) is:

1. **No main window.** The app lives in the system tray. The hotkey is the entire interaction.
2. **Global hotkey activation** -- press to start, press to stop (toggle mode). Push-to-hold as alternative.
3. **Floating pill indicator** -- small (240-320px wide, 48-56px tall), no title bar, always-on-top, no taskbar entry. Shows recording/processing/done/error states.
4. **Invisible text delivery** -- text appears where the cursor is, via clipboard injection. The user never leaves their current app.
5. **Minimal settings** -- accessible from tray menu. No "Save" button, changes apply immediately.

### 7.2 Floating Pill Design (Mapped to Our Design Tokens)

| State | Visual | Color Token | Animation |
|-------|--------|-------------|-----------|
| Recording | Waveform bars + "Listening" | `accent-primary` (#6366F1) with pulsing glow | Bars animate with audio levels |
| Processing | Spinner + "Processing..." | `text-secondary` (#A1A1AA) | Spinner rotation |
| Success | Checkmark + "Done" | `accent-success` (#22C55E) | 1.5s display, fade out |
| Error | Error icon + message | `accent-error` (#EF4444) | Persists until dismissed |
| No audio | Flat waveform + "Can't hear you" | `accent-warning` (#F59E0B) | Gentle pulse |

- Background: `bg-elevated` (#18181B) with backdrop-blur (glassmorphism per DESIGN-11)
- Border: 1px `border-subtle` (#27272A)
- Signature detail: Soft indigo glow around the pill during recording (per DESIGN-12)
- Must appear in <100ms when hotkey is pressed

### 7.3 Onboarding (5 Steps, Under 60 Seconds)

1. **Welcome + value prop:** "Dictate anywhere. 3x faster than typing."
2. **Microphone permission:** Trigger OS dialog. Privacy statement: "Audio processed on-device."
3. **Hotkey setup:** Accept default (`Ctrl+Shift+Space`) or record custom shortcut.
4. **First dictation (magic moment):** User presses hotkey, speaks, sees text appear.
5. **System tray location:** Show where the icon lives. "We'll be here when you need us."

### 7.4 Accessibility Considerations

This app is inherently an assistive technology. Key implications:
- **It may be a user's primary input method.** Reliability is an accessibility requirement.
- **Error recovery must not require fine motor control.** Quick-redo via hotkey (delete last dictation and re-record).
- **State must be communicated multi-modally:** Visual (color + shape + animation) + optional audio tones + screen reader announcements.
- **Windows High Contrast mode must be supported.**
- **Respect `prefers-reduced-motion`** per DESIGN animation guidelines.

---

## 8. Constraints, Risks, and Dependencies

### 8.1 Constraints

**Technical:**
- whisper.cpp on CPU with `base.en` model introduces 1-3 second transcription latency (acceptable but noticeable vs. cloud streaming at <300ms).
- Windows system tray icons are collapsed by default on Windows 11; hotkey must be the primary interaction, not tray icon.
- Text injection via clipboard has edge cases with rich text formats and large clipboard contents.
- Global hotkeys can conflict with other apps; must detect and handle gracefully.

**Organizational:**
- If Tauri is chosen, Rust backend requires learning Rust (team currently knows TypeScript/React).
- Solo developer or small team -- must scope aggressively for MVP.

### 8.2 Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| **Microsoft builds premium dictation into Windows** | High (eliminates market gap) | Medium | Ship fast; differentiate on AI cleanup and design quality |
| **WisprFlow ships a Windows version** | High (direct competition) | Medium | First-mover advantage on Windows; local-first and lifetime license as differentiators |
| **Whisper accuracy insufficient for dictation without GPU** | Medium | Low | `base.en` on modern CPU is sufficient; offer cloud fallback |
| **Text injection unreliable across Windows apps** | High | Medium | Test extensively; clipboard injection is proven; accessibility API as backup |
| **Rust learning curve slows development** | Medium | Medium | Could switch to Electron for faster MVP; Rust backend is scoped and well-served by existing crates |

### 8.3 Dependencies & Assumptions

- **Assumes** modern Windows 10/11 as minimum target (no Windows 7/8).
- **Assumes** users have a microphone (built-in or external).
- **Depends on** whisper.cpp continuing to be maintained (very active, 50K+ GitHub stars).
- **Depends on** Tauri v2 stability for production desktop apps (GA released, growing adoption).
- **Assumes** `base.en` model provides sufficient accuracy for English dictation (~95%+ in quiet environments).

---

## 9. Opportunities & Ideas

### Reuse Opportunities
- Existing design token system (DESIGN standards) maps directly to the floating pill and settings UI.
- shadcn/ui components for the settings window.
- The team's React/TypeScript/Tailwind expertise covers the entire frontend.

### Quick Wins
- Windows Voice Typing (Win+H) is so mediocre that even a basic whisper.cpp dictation tool with a polished UI would be compelling.
- Dark mode + system tray app + floating pill can be built quickly with Tauri + existing design system.

### Differentiation Ideas
1. **AI text cleanup** (tiered: rule-based free, local LLM mid, cloud LLM premium) -- the WisprFlow killer feature, brought to Windows.
2. **Local-first privacy** -- all processing on-device by default, no cloud account required.
3. **Lifetime license option** -- fight subscription fatigue in a market dominated by monthly pricing.
4. **Context-aware formatting** -- detect target app and adjust output (formal for email, markdown for docs, code comments for IDE).
5. **"Edit with voice" mode** -- voice commands to correct and modify text after dictation.
6. **Dictation history** -- searchable archive of everything you've dictated.
7. **Per-app profiles** -- different settings per application.

### Future Extensions (Post-MVP)
- macOS support (Tauri v2 supports it; whisper.cpp has Apple Silicon optimization).
- Linux support (Tauri v2 supports it; whisper.cpp runs natively).
- Real-time streaming preview in the pill.
- Custom vocabulary / domain-specific terms.
- Multi-language support (Whisper supports 99+ languages).
- Voice commands ("new line", "select all", "delete that").

---

## 10. Key Findings by Track

### 10.1 Product / Feature Findings
1. **Windows is the correct beachhead.** No premium AI-enhanced dictation tool exists for Windows. The competitive landscape is weak (Dragon is being sunset, Windows Voice Typing is mediocre, Talon targets power users).
2. **whisper.cpp is the right STT foundation.** Zero dependencies, MIT licensed, all platforms, all model sizes. The `base.en` model offers the best latency/accuracy trade-off for English. Hybrid architecture (local default + optional cloud) gives users choice.
3. **AI text cleanup is the killer feature, not raw transcription.** WisprFlow proved that LLM post-processing (fixing grammar, formatting, punctuation) is what makes dictation magical. Raw Whisper output is good; LLM-cleaned output is great.
4. **Tauri v2 is the recommended framework** (narrowly over Electron). Lighter weight, better ML integration via Rust, same React/TypeScript frontend. The Rust learning curve is the main trade-off.

### 10.2 Design Findings
1. **The floating pill is the entire product.** No main window, no dedicated editor. Hotkey → pill → text appears. This is the proven pattern (WisprFlow, Superwhisper, macOS Dictation).
2. **The indigo glow during recording is the signature visual.** Dark background + glassmorphism pill + soft indigo glow = premium feel that aligns with existing DESIGN standards.
3. **Speed is the brand.** <100ms hotkey response, <2s cold start. Every millisecond of perceived latency degrades trust. The app must feel instant.
4. **Onboarding must deliver the magic moment in under 60 seconds.** The user must press the hotkey, speak, and see text appear in their first session. The hotkey is the hero.

---

## 11. Recommendations for the Create Phase

### 11.1 Recommended Requirements Documents
- **Create next:** PRD (Product Requirements Document) for the Windows MVP.
- **Then:** DRD (Design Requirements Document) for the floating pill, settings UI, onboarding, and system tray behavior.
- **Suggested filenames:** `prd-dictation-mvp-v1.md`, `drd-dictation-ui-v1.md`

### 11.2 Scope Recommendations

**MVP scope (must have):**
1. System tray background app on Windows (Tauri v2 or Electron)
2. Global hotkey activation (toggle + push-to-hold modes, configurable)
3. Floating pill indicator with recording/processing/done/error states
4. Local STT via whisper.cpp (`base.en` model)
5. Clipboard-based text injection into any focused text field
6. Basic AI text cleanup (rule-based punctuation, capitalization)
7. Settings: hotkey, activation mode, model size, launch at startup
8. 5-step onboarding flow (permission, hotkey, first dictation)
9. Dark mode UI using existing design token system

**Stretch / Deferred:**
1. Cloud STT option (Deepgram/Azure)
2. LLM-powered AI text cleanup (local or cloud)
3. Streaming transcription preview
4. Quick-redo via hotkey
5. Dictation history
6. Custom vocabulary
7. macOS and Linux support
8. Multi-language support
9. Context-aware formatting
10. Voice commands for editing

### 11.3 Key Questions the Requirements Doc Should Answer
1. **Framework decision: Tauri v2 or Electron?** Is the Rust learning curve acceptable for the trade-off of lighter weight and better ML integration?
2. **AI cleanup scope for MVP:** Rule-based only, or include a local LLM option?
3. **Pricing model:** Freemium with paid tiers, one-time purchase, subscription, or hybrid (free core + paid AI features)?
4. **App name and brand identity?**
5. **Minimum Windows version:** Windows 10 21H2+ or Windows 11 only?
6. **Open source or proprietary?** Local-first + open-source whisper.cpp invites an open-source ethos, but commercial viability matters.

### 11.4 Suggested Decisions to Lock In Now
1. **Windows first** -- confirmed by competitive analysis. No premium AI dictation on Windows.
2. **Dark mode first** -- aligns with design standards (DESIGN-1), target audience (developers/knowledge workers), and Linear-inspired aesthetic.
3. **Global hotkey as primary interaction** -- the entire UX revolves around the hotkey. No main window for MVP.
4. **Clipboard injection for text delivery** -- proven by WisprFlow, works in any app.
5. **whisper.cpp for local STT** -- best cross-platform, zero-dependency, MIT-licensed option.
6. **Existing design token system** -- Linear-inspired dark palette, typography, spacing, and component standards carry directly into this product.

---

## 12. Open Questions & Gaps

- **Whisper accuracy in noisy environments:** Research was moderate-depth; real-world testing needed to validate `base.en` accuracy with background noise, accents, and fast speech.
- **Text injection edge cases on Windows:** Clipboard injection behavior in Electron apps, UWP apps, and browser-based editors (Google Docs, Notion) needs testing.
- **Tauri v2 maturity for production:** Tauri v2 is GA but newer than Electron. Edge cases in system tray behavior, global hotkeys, and auto-updates on Windows need validation.
- **Local LLM feasibility:** Can a local LLM (e.g., Phi-3, Llama 3) run alongside whisper.cpp without excessive resource usage? Or is cloud LLM the only practical option for AI cleanup?
- **Revenue viability:** Market research on willingness-to-pay for a Windows dictation tool was not conducted. Need user interviews or a landing page test.
- **Accessibility API text injection on Windows:** IUIAutomation support varies by application. Need to survey which major apps (VS Code, Word, Outlook, Chrome, Slack) support it.

---

## 13. Sources & References

### Competitors & Market
- WisprFlow / Wispr: https://www.wispr.com/
- Superwhisper: https://superwhisper.com/
- Talon Voice: https://talonvoice.com/
- Dragon NaturallySpeaking: https://www.nuance.com/dragon.html

### STT Engines
- OpenAI Whisper: https://github.com/openai/whisper
- whisper.cpp: https://github.com/ggerganov/whisper.cpp
- faster-whisper: https://github.com/SYSTRAN/faster-whisper
- Vosk: https://alphacephei.com/vosk/
- Silero Models (VAD): https://github.com/snakers4/silero-models
- Distil-Whisper: https://huggingface.co/distil-whisper
- Moonshine: https://github.com/usefulsensors/moonshine
- whisper_streaming: https://github.com/ufal/whisper_streaming

### Cloud STT Services
- Deepgram: https://deepgram.com/ and https://developers.deepgram.com/
- Azure Speech Services: https://azure.microsoft.com/en-us/products/ai-services/speech-to-text
- Google Cloud Speech-to-Text: https://cloud.google.com/speech-to-text
- AssemblyAI: https://www.assemblyai.com/
- AWS Transcribe: https://aws.amazon.com/transcribe/

### Desktop Frameworks
- Tauri v2: https://v2.tauri.app/
- Electron: https://www.electronjs.org/
- Wails: https://wails.io/

### Design Inspiration
- Linear: https://linear.app/
- Raycast: https://raycast.com/
- CleanShot X: https://cleanshot.com/
- Arc Browser: https://arc.net/

### Rust Crates (for Tauri backend)
- whisper-rs: https://crates.io/crates/whisper-rs
- cpal (audio capture): https://crates.io/crates/cpal
- enigo (keyboard simulation): https://crates.io/crates/enigo

---

## Standards Compliance

```
standards_version: 1.0.0
applied_standards:
  - global/principles.md
  - global/security-privacy.md
  - domains/code-internal-architecture.md
  - domains/content-voice.md
  - domains/design-ui.md
  - phases/research.md
```

### Applied Rules
- [PRIN-1] User-First: Research prioritized user needs (accuracy, speed, privacy, simplicity).
- [PRIN-5] Incremental Delivery: MVP scope is tightly scoped; stretch items deferred.
- [PRIN-10] Simplicity: Recommended the simplest viable architecture (local whisper.cpp + clipboard injection).
- [PRIN-14] Reuse Before Build: Identified existing design tokens, shadcn/ui components, and open-source STT engines.
- [SEC-2] PII Protection: Architecture defaults to local processing; no audio sent to cloud without explicit user opt-in.
- [R-1] Research goal clearly stated.
- [R-3] Sources cited throughout.
- [R-4] Uncertainties flagged in Section 12.
- [R-7] Actionable recommendations provided in Section 11.
- [R-8] Scope and limitations documented.

### Deviations
- None.
