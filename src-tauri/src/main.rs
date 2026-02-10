// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// NO-TELEMETRY COMPLIANCE (FR-35):
// This application does not include any analytics, crash reporting, telemetry,
// or tracking of any kind. All speech processing happens locally on the user's
// device. Cloud LLM calls (when enabled by the user) send only the transcribed
// text to the selected provider's API — no metadata, device info, or usage
// stats are transmitted. The auto-updater checks for new versions only and does
// not report any user data. This is a deliberate design decision.

fn main() {
    dictation_app_lib::run()
}
