import type { Workspace } from "@/bindings"
import { WorkspaceCard } from "./workspace-card"

export interface WorkspaceListProps {
  workspaces: Workspace[]
  onOpen: (name: string) => void
  onRename: (workspace: Workspace) => void
  onDelete: (workspace: Workspace) => void
}

export function WorkspaceList({
  workspaces,
  onOpen,
  onRename,
  onDelete,
}: WorkspaceListProps) {
  return (
    <div className="flex flex-col gap-3 w-full max-w-md">
      {workspaces.map(ws => (
        // workspace names are unique by directory — guaranteed by the backend
        <WorkspaceCard
          key={ws.name}
          workspace={ws}
          onOpen={onOpen}
          onRename={onRename}
          onDelete={onDelete}
        />
      ))}
    </div>
  )
}
