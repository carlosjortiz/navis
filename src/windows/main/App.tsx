import { Globe, Minus, Plus, RotateCcw, Search, Send, Settings } from "lucide-react"
import { useState } from "react"
import { useTranslation } from "react-i18next"
import { Button } from "@/components/ui/button"
import { CodeMirrorEditor } from "@/components/code-mirror-editor"
import { SettingsDialog } from "@/components/settings-dialog"
import { TitleBar } from "@/components/title-bar"
import { useCounterStore } from "@/stores/counter-store"

const SAMPLE_JSON = `{
  "name": "demo-workspace",
  "version": 1,
  "apis": [
    { "slug": "users-api", "baseUrl": "https://api.example.com" },
    { "slug": "billing-api", "baseUrl": "https://billing.example.com" }
  ]
}`

export default function App() {
  const { t, i18n } = useTranslation()

  const count = useCounterStore((state) => state.count)
  const increment = useCounterStore((state) => state.increment)
  const decrement = useCounterStore((state) => state.decrement)
  const reset = useCounterStore((state) => state.reset)

  const [settingsOpen, setSettingsOpen] = useState(false)
  const [editorValue, setEditorValue] = useState(SAMPLE_JSON)

  const currentLang = i18n.resolvedLanguage ?? i18n.language

  return (
    <div className="flex flex-col h-screen bg-background/80 backdrop-blur-md font-sans">
      <TitleBar />
      <main className="flex-1 overflow-auto flex items-start justify-center p-8">
        <div className="bg-white rounded-lg shadow-lg p-8 max-w-md w-full space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-primary-900 mb-2">{t("app.title")}</h1>
          <p className="text-primary-500">{t("app.subtitle")}</p>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">{t("sections.language")}</p>
          <div className="flex gap-2">
            <Button
              variant={currentLang === "en" ? "default" : "outline"}
              onClick={() => i18n.changeLanguage("en")}
            >
              EN
            </Button>
            <Button
              variant={currentLang === "es" ? "default" : "outline"}
              onClick={() => i18n.changeLanguage("es")}
            >
              ES
            </Button>
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">{t("sections.standaloneIcons")}</p>
          <div className="flex items-center gap-4 text-foreground">
            <Globe className="size-5" />
            <Search className="size-5" />
            <Send className="size-5" />
            <Settings className="size-5" />
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">{t("sections.iconsInsideButton")}</p>
          <div className="flex flex-wrap gap-2">
            <Button variant="default">
              <Send />
              {t("actions.sendRequest")}
            </Button>
            <Button variant="outline">
              <Search />
              {t("actions.search")}
            </Button>
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">{t("sections.iconOnlyButtons")}</p>
          <div className="flex gap-2">
            <Button size="icon" variant="default" aria-label={t("actions.sendRequest")}>
              <Send />
            </Button>
            <Button size="icon" variant="outline" aria-label={t("settings.title")} onClick={() => setSettingsOpen(true)}>
              <Settings />
            </Button>
            <Button size="icon" variant="ghost" aria-label={t("actions.search")}>
              <Search />
            </Button>
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">{t("sections.zustandCounter")}</p>
          <div className="flex items-center gap-3">
            <span className="text-2xl font-bold text-primary-900 tabular-nums w-12 text-center">
              {count}
            </span>
            <Button size="icon" variant="default" onClick={increment} aria-label={t("actions.increment")}>
              <Plus />
            </Button>
            <Button size="icon" variant="outline" onClick={decrement} aria-label={t("actions.decrement")}>
              <Minus />
            </Button>
            <Button size="icon" variant="ghost" onClick={reset} aria-label={t("actions.reset")}>
              <RotateCcw />
            </Button>
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">CodeMirror editor (JSON)</p>
          <div className="border border-border rounded-md overflow-hidden">
            <CodeMirrorEditor value={editorValue} onChange={setEditorValue} />
          </div>
        </div>
        </div>
      </main>
      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
    </div>
  )
}
