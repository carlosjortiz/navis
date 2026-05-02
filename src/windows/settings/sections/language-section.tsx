import { useTranslation } from "react-i18next";

import type { Language, Settings } from "@/bindings";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const LANGUAGE_OPTIONS: readonly Language[] = ["en", "es"] as const;

type Props = {
  settings: Settings;
  onLanguageChange: (language: Language) => void;
};

export function LanguageSection({ settings, onLanguageChange }: Props) {
  const { t } = useTranslation();

  return (
    <section className="space-y-6">
      <h2 className="text-lg font-semibold">{t("settings.language.title")}</h2>

      <div className="space-y-3">
        <Label htmlFor="language-select">{t("settings.language.label")}</Label>
        <Select
          value={settings.language}
          onValueChange={(value) => onLanguageChange(value as Language)}
        >
          <SelectTrigger id="language-select" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {LANGUAGE_OPTIONS.map((code) => (
              <SelectItem key={code} value={code}>
                {t(`settings.language.options.${code}`)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
    </section>
  );
}
