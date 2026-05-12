import { useCallback, useEffect, useState } from "react"
import { commands, type AppError, type Workspace } from "@/bindings"

export type WorkspaceListState =
  | { kind: "loading" }
  | { kind: "empty" }
  | { kind: "populated"; workspaces: Workspace[] }
  | { kind: "error"; error: AppError; previous?: Workspace[] }

export interface UseWorkspacesResult {
  state: WorkspaceListState
  refresh: () => Promise<void>
}

export function useWorkspaces(): UseWorkspacesResult {
  const [state, setState] = useState<WorkspaceListState>({ kind: "loading" })

  // Using setState(prev => ...) lets refresh read prior state without closing
  // over it, so refresh has empty deps and stays stable across renders.
  const refresh = useCallback(async () => {
    const result = await commands.listWorkspaces()
    setState(prev => {
      if (result.status === "ok") {
        return result.data.length === 0
          ? { kind: "empty" }
          : { kind: "populated", workspaces: result.data }
      }
      const previous =
        prev.kind === "populated" ? prev.workspaces :
        prev.kind === "error" ? prev.previous :
        undefined
      return { kind: "error", error: result.error, previous }
    })
  }, [])

  useEffect(() => {
    void refresh()
  }, [refresh])

  return { state, refresh }
}
