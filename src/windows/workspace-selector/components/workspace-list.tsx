import type { Workspace } from "@/bindings"
import { WorkspaceCard } from "./workspace-card"

export interface WorkspaceListProps {
  workspaces: Workspace[]
  onOpen: (name: string) => void
}

export function WorkspaceList({ workspaces, onOpen }: WorkspaceListProps) {
  return (
    <div className="flex flex-col gap-3 w-full max-w-md">
      {workspaces.map(ws => (
        // workspace names are unique by directory — guaranteed by the backend
        <WorkspaceCard key={ws.name} workspace={ws} onOpen={onOpen} />
      ))}
    </div>
  )
}
