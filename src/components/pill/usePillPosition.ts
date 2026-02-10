import { useEffect, useRef } from "react";
import { getCurrentWindow, PhysicalPosition } from "@tauri-apps/api/window";

const POSITION_KEY = "pill-position";

interface SavedPosition {
  x: number;
  y: number;
}

/**
 * Hook to persist and restore the pill window position.
 * Uses localStorage for simplicity. On first launch, the window is
 * centered (Tauri config default). After the user drags it, the
 * new position is saved and restored on next show.
 */
export function usePillPosition() {
  const restoredRef = useRef(false);

  useEffect(() => {
    const win = getCurrentWindow();

    // Restore saved position on first mount
    const restore = async () => {
      if (restoredRef.current) return;
      restoredRef.current = true;

      const saved = localStorage.getItem(POSITION_KEY);
      if (!saved) return;

      try {
        const pos: SavedPosition = JSON.parse(saved);
        await win.setPosition(new PhysicalPosition(pos.x, pos.y));
      } catch {
        // Ignore invalid saved positions
      }
    };

    restore();

    // Save position when the window is moved (debounced)
    let saveTimer: ReturnType<typeof setTimeout> | null = null;
    const unlisten = win.onMoved((event) => {
      if (saveTimer) clearTimeout(saveTimer);
      saveTimer = setTimeout(() => {
        const pos: SavedPosition = { x: event.payload.x, y: event.payload.y };
        localStorage.setItem(POSITION_KEY, JSON.stringify(pos));
      }, 300);
    });

    return () => {
      if (saveTimer) clearTimeout(saveTimer);
      unlisten.then((fn) => fn());
    };
  }, []);
}
