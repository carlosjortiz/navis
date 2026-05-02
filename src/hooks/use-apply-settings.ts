import { useEffect } from "react";

import type { Settings } from "@/bindings";
import i18n from "@/i18n";

const setDark = (isDark: boolean) => {
  document.documentElement.classList.toggle("dark", isDark);
};

export function useApplySettings(settings: Settings | null): void {
  useEffect(() => {
    if (settings === null) return;
    if (settings.theme === "dark") {
      setDark(true);
      return;
    }
    if (settings.theme === "light") {
      setDark(false);
      return;
    }
    // theme === "system": apply the current OS preference and stay subscribed
    // to its changes so flipping dark mode in the OS updates the running app.
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    setDark(mq.matches);
    const onChange = (event: MediaQueryListEvent) => {
      setDark(event.matches);
    };
    mq.addEventListener("change", onChange);
    return () => {
      mq.removeEventListener("change", onChange);
    };
  }, [settings?.theme]);

  useEffect(() => {
    if (settings === null) return;
    void i18n.changeLanguage(settings.language);
  }, [settings?.language]);
}
