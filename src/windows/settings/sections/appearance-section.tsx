import { useTranslation } from "react-i18next";

import type { Settings, Theme } from "@/bindings";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Slider } from "@/components/ui/slider";

const THEME_OPTIONS: readonly Theme[] = ["system", "light", "dark"] as const;

type Props = {
  settings: Settings;
  onThemeChange: (theme: Theme) => void;
  onOpacityChange: (opacity: number) => void;
};

export function AppearanceSection({ settings, onThemeChange, onOpacityChange }: Props) {
  const { t } = useTranslation();

  return (
    <section className="space-y-6">
      <h2 className="text-lg font-semibold">{t("settings.appearance.title")}</h2>

      <div className="space-y-3">
        <Label>{t("settings.appearance.theme.label")}</Label>
        <RadioGroup
          value={settings.theme}
          onValueChange={(value) => onThemeChange(value as Theme)}
          className="flex gap-4"
        >
          {THEME_OPTIONS.map((option) => (
            <div key={option} className="flex items-center gap-2">
              <RadioGroupItem id={`theme-${option}`} value={option} />
              <Label htmlFor={`theme-${option}`} className="font-normal">
                {t(`settings.appearance.theme.options.${option}`)}
              </Label>
            </div>
          ))}
        </RadioGroup>
      </div>

      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <Label>{t("settings.appearance.opacity.label")}</Label>
          <span className="text-sm tabular-nums text-muted-foreground">
            {Math.round(settings.opacity * 100)}%
          </span>
        </div>
        <Slider
          min={0.3}
          max={1}
          step={0.05}
          value={[settings.opacity]}
          onValueChange={(values) => {
            const next = values[0];
            if (next !== undefined) onOpacityChange(next);
          }}
        />
      </div>
    </section>
  );
}
