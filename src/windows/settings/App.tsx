import { useEffect, useRef, useState, type CSSProperties } from "react";
import { useTranslation } from "react-i18next";

import { commands, type Language, type Settings, type Theme } from "@/bindings";
import { TitleBar } from "@/components/title-bar";
import { Button } from "@/components/ui/button";
import { useApplySettings } from "@/hooks/use-apply-settings";
import { emitSettingsPreview } from "@/lib/settings-events";

import { useDebouncedSave } from "./hooks/use-debounced-save";
import { AppearanceSection } from "./sections/appearance-section";
import { LanguageSection } from "./sections/language-section";

const rootBackgroundStyle = (opacity: number): CSSProperties => ({
  backgroundColor: `color-mix(in oklab, var(--background) ${opacity * 100}%, transparent)`,
});

const settingsEqual = (a: Settings, b: Settings): boolean =>
  a.theme === b.theme &&
  a.language === b.language &&
  a.opacity === b.opacity &&
  a.schema_version === b.schema_version;

export default function App() {
  const { t } = useTranslation();
  const [persisted, setPersisted] = useState<Settings | null>(null);
  const [draft, setDraft] = useState<Settings | null>(null);

  useApplySettings(draft);

  useEffect(() => {
    void (async () => {
      const result = await commands.getSettings();
      if (result.status === "ok") {
        setPersisted(result.data);
        setDraft(result.data);
      }
    })();
  }, []);

  // If the window is closed with unsaved edits, broadcast the persisted
  // value once so any main window that was previewing the draft snaps back.
  // Stored via ref so the cleanup reads the latest state without re-running
  // the effect (which would prematurely revert in-progress edits).
  const revertTargetRef = useRef<Settings | null>(null);
  useEffect(() => {
    revertTargetRef.current =
      persisted !== null && draft !== null && !settingsEqual(persisted, draft)
        ? persisted
        : null;
  });
  useEffect(() => {
    return () => {
      if (revertTargetRef.current !== null) {
        void emitSettingsPreview(revertTargetRef.current);
      }
    };
  }, []);

  const debouncedEmit = useDebouncedSave((next: Settings) => {
    void emitSettingsPreview(next);
  }, 150);

  if (draft === null || persisted === null) {
    return (
      <div
        className="flex flex-col h-screen backdrop-blur-md font-sans"
        style={rootBackgroundStyle(0.8)}
      >
        <TitleBar />
        <main className="flex-1 flex items-center justify-center text-sm text-muted-foreground">
          {t("settings.loading")}
        </main>
      </div>
    );
  }

  const isDirty = !settingsEqual(persisted, draft);

  const onThemeChange = (theme: Theme) => {
    const next = { ...draft, theme };
    setDraft(next);
    void emitSettingsPreview(next);
  };

  const onLanguageChange = (language: Language) => {
    const next = { ...draft, language };
    setDraft(next);
    void emitSettingsPreview(next);
  };

  const onOpacityChange = (opacity: number) => {
    const next = { ...draft, opacity };
    setDraft(next);
    debouncedEmit(next);
  };

  const handleSave = async () => {
    const result = await commands.saveSettings(draft);
    if (result.status === "ok") {
      setPersisted(draft);
    }
  };

  const handleCancel = () => {
    setDraft(persisted);
    void emitSettingsPreview(persisted);
  };

  return (
    <div
      className="flex flex-col h-screen backdrop-blur-md font-sans"
      style={rootBackgroundStyle(draft.opacity)}
    >
      <TitleBar />
      <main className="flex-1 overflow-auto p-8">
        <div className="max-w-md w-full mx-auto space-y-8">
          <h1 className="text-2xl font-bold">{t("settings.title")}</h1>
          <AppearanceSection
            settings={draft}
            onThemeChange={onThemeChange}
            onOpacityChange={onOpacityChange}
          />
          <LanguageSection
            settings={draft}
            onLanguageChange={onLanguageChange}
          />
          <div className="flex justify-end gap-2 pt-4 border-t border-border">
            <Button
              variant="secondary"
              onClick={handleCancel}
              disabled={!isDirty}
            >
              {t("actions.cancel")}
            </Button>
            <Button onClick={handleSave} disabled={!isDirty}>
              {t("actions.save")}
            </Button>
          </div>
        </div>
      </main>
    </div>
  );
}
