import { useTranslation } from "react-i18next"
import { TitleBar } from "@/components/title-bar"
import { Button } from "@/components/ui/button"
import { commands } from "@/bindings"
import { notifyError } from "@/lib/notify"

export default function App() {
  const { t } = useTranslation()

  const handleOpenDemo = async () => {
    const result = await commands.openWorkspace("demo")
    if (result.status === "error") {
      notifyError(result.error, t)
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
