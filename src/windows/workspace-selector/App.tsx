import { useEffect, useState } from "react"
import { useTranslation } from "react-i18next"
import { Loader2 } from "lucide-react"
import { TitleBar } from "@/components/title-bar"
import { commands, type Workspace } from "@/bindings"
import { notifyError } from "@/lib/notify"
import { useWorkspaces } from "./hooks/use-workspaces"
import { EmptyState } from "./components/empty-state"
import { WorkspaceList } from "./components/workspace-list"
import { WorkspaceHeader } from "./components/workspace-header"
import { WorkspaceFormDialog } from "./components/workspace-form-dialog"
import { DeleteWorkspaceAlert } from "./components/delete-workspace-alert"

export default function App() {
  const { t } = useTranslation()
  const { state, refresh } = useWorkspaces()

  const [createOpen, setCreateOpen] = useState(false)
  const [renameTarget, setRenameTarget] = useState<Workspace | null>(null)
  const [deleteTarget, setDeleteTarget] = useState<Workspace | null>(null)

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

  const openCreate = () => setCreateOpen(true)

  const renderBody = () => {
    if (state.kind === "loading") {
      return (
        <div className="flex-1 flex items-center justify-center p-8">
          <Loader2 className="size-8 animate-spin text-muted-foreground" />
        </div>
      )
    }
    if (state.kind === "empty") {
      return (
        <div className="flex-1 flex items-center justify-center p-8">
          <EmptyState onCreate={openCreate} />
        </div>
      )
    }
    const workspaces =
      state.kind === "populated"
        ? state.workspaces
        : state.kind === "error"
          ? state.previous
          : undefined

    if (!workspaces || workspaces.length === 0) {
      return (
        <div className="flex-1 flex items-center justify-center p-8">
          <EmptyState onCreate={openCreate} />
        </div>
      )
    }

    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <WorkspaceHeader onCreate={openCreate} />
        <div className="flex-1 overflow-auto px-6 py-4 flex justify-center">
          <WorkspaceList
            workspaces={workspaces}
            onOpen={handleOpen}
            onRename={(ws) => setRenameTarget(ws)}
            onDelete={(ws) => setDeleteTarget(ws)}
          />
        </div>
      </div>
    )
  }

  return (
    <div className="flex flex-col h-screen bg-background/80 backdrop-blur-md text-foreground font-sans">
      <TitleBar />
      <main className="flex-1 flex flex-col overflow-hidden">
        {renderBody()}
      </main>

      <WorkspaceFormDialog
        mode="create"
        open={createOpen}
        onOpenChange={setCreateOpen}
        onSuccess={refresh}
      />

      {renameTarget && (
        <WorkspaceFormDialog
          mode="rename"
          open
          onOpenChange={(open) => {
            if (!open) setRenameTarget(null)
          }}
          oldName={renameTarget.name}
          onSuccess={refresh}
        />
      )}

      {deleteTarget && (
        <DeleteWorkspaceAlert
          open
          onOpenChange={(open) => {
            if (!open) setDeleteTarget(null)
          }}
          workspaceName={deleteTarget.name}
          onSuccess={refresh}
        />
      )}
    </div>
  )
}
