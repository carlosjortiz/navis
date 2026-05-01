import { invoke } from "@tauri-apps/api/core"
import { useTranslation } from "react-i18next"
import { TitleBar } from "@/components/title-bar"
import { Button } from "@/components/ui/button"
import { formatAppError } from "@/lib/app-error"

export default function App() {
  const { t } = useTranslation()

  const handleOpenDemo = async () => {
    try {
      await invoke("open_workspace", { slug: "demo" })
    } catch (err) {
      // Placeholder until proper toast UI lands; alert is sufficient for the smoke path.
      alert(formatAppError(err, t))
    }
  }

  return (
    <div className="flex flex-col h-screen bg-background/80 backdrop-blur-md text-foreground font-sans">
      <TitleBar />
      <main className="flex-1 overflow-auto flex items-center justify-center p-8">
        <div className="bg-card rounded-lg shadow-lg p-8 max-w-md w-full space-y-3 text-center">
          <h1 className="text-3xl font-bold">Workspace Selector</h1>
          <p className="text-muted-foreground">Pick a workspace to open.</p>
          <Button onClick={handleOpenDemo}>Open demo workspace</Button>
        </div>
      </main>
    </div>
  )
}
