import { useEffect } from "react"
import { useTranslation } from "react-i18next"
import { Loader2 } from "lucide-react"
import { TitleBar } from "@/components/title-bar"
import { commands } from "@/bindings"
import { notifyError } from "@/lib/notify"
import { useWorkspaces } from "./hooks/use-workspaces"
import { EmptyState } from "./components/empty-state"
import { WorkspaceList } from "./components/workspace-list"

export default function App() {
  const { t } = useTranslation()
  const { state } = useWorkspaces()

  // Surface errors via toast while letting the list keep showing previously-known workspaces.
  useEffect(() => {
    if (state.kind === "error") {
      notifyError(state.error, t)
    }
  }, [state, t])

  const handleOpen = async (name: string) => {
    const result = await commands.openWorkspace(name)
    if (result.status === "error") {
      notifyError(result.error, t)
    }
  }

  return (
    <div className="flex flex-col h-screen bg-background/80 backdrop-blur-md text-foreground font-sans">
      <TitleBar />
      <main className="flex-1 overflow-auto flex items-center justify-center p-8">
        {state.kind === "loading" && (
          <Loader2 className="size-8 animate-spin text-muted-foreground" />
        )}
        {state.kind === "empty" && (
          <EmptyState />
        )}
        {state.kind === "populated" && (
          <WorkspaceList workspaces={state.workspaces} onOpen={handleOpen} />
        )}
        {state.kind === "error" && state.previous !== undefined && (
          <WorkspaceList workspaces={state.previous} onOpen={handleOpen} />
        )}
        {state.kind === "error" && state.previous === undefined && (
          <EmptyState />
        )}
      </main>
    </div>
  )
}
