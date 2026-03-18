export type Theme = "dark" | "light" | "system";

const STORAGE_KEY = "theme";

function createTheme() {
  let current = $state<Theme>("system");
  let mediaQuery: MediaQueryList | null = null;
  let mediaListener: ((e: MediaQueryListEvent) => void) | null = null;

  function applyDark(isDark: boolean) {
    document.documentElement.classList.toggle("dark", isDark);
  }

  function resolveAndApply(t: Theme) {
    if (t === "system") {
      applyDark(mediaQuery?.matches ?? false);
    } else {
      applyDark(t === "dark");
    }
  }

  return {
    get current() {
      return current;
    },

    get isDark() {
      if (current === "system") return mediaQuery?.matches ?? false;
      return current === "dark";
    },

    init() {
      mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      const stored = localStorage.getItem(STORAGE_KEY) as Theme | null;
      current = stored ?? "system";
      resolveAndApply(current);

      mediaListener = (e) => {
        if (current === "system") applyDark(e.matches);
      };
      mediaQuery.addEventListener("change", mediaListener);
    },

    set(t: Theme) {
      current = t;
      localStorage.setItem(STORAGE_KEY, t);
      resolveAndApply(t);
    },

    cleanup() {
      if (mediaQuery && mediaListener) {
        mediaQuery.removeEventListener("change", mediaListener);
        mediaListener = null;
      }
    },
  };
}

export const theme = createTheme();
