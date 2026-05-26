import { useState } from "react"
import { ChevronDown, Circle, Plus } from "lucide-react"
import { useTranslation } from "react-i18next"

import { commands } from "@/bindings"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { notifyError } from "@/lib/notify"
import { useOpenWorkspaces } from "./use-open-workspaces"
import { useWorkspaces } from "./use-workspaces"
import { WorkspaceFormDialog } from "./workspace-form-dialog"

interface WorkspaceSwitcherProps {
  currentWorkspaceName: string
}

export function WorkspaceSwitcher({ currentWorkspaceName }: WorkspaceSwitcherProps) {
  const { t } = useTranslation()
  const { state, refresh } = useWorkspaces()
  const openNames = useOpenWorkspaces()
  const [createOpen, setCreateOpen] = useState(false)

  const workspaces =
    state.kind === "populated"
      ? state.workspaces
      : state.kind === "error" && state.previous
        ? state.previous
        : []

  const openSet = new Set(openNames)
  const openList = workspaces.filter((w) => openSet.has(w.name))
  const closedList = workspaces.filter((w) => !openSet.has(w.name))

  const handleOpen = async (name: string) => {
    const result = await commands.openWorkspace(name)
    if (result.status === "error") notifyError(result.error, t)
  }

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            type="button"
            aria-label={t("workspaceSwitcher.triggerLabel")}
            className="inline-flex items-center gap-1 rounded px-2 py-0.5 text-sm text-foreground transition-colors hover:bg-muted/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          >
            <span className="truncate max-w-[200px]">{currentWorkspaceName}</span>
            <ChevronDown className="size-3.5 text-muted-foreground" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-[220px]">
          <DropdownMenuItem onSelect={() => setCreateOpen(true)}>
            <Plus className="size-4" />
            {t("workspaceSwitcher.newWorkspace")}
          </DropdownMenuItem>

          {openList.length > 0 && (
            <>
              <DropdownMenuSeparator />
              <DropdownMenuLabel className="text-xs font-normal text-muted-foreground">
                {t("workspaceSwitcher.openLabel")}
              </DropdownMenuLabel>
              <DropdownMenuGroup>
                {openList.map((w) => (
                  <DropdownMenuItem key={w.name} onSelect={() => handleOpen(w.name)}>
                    <Circle className="size-2 fill-primary text-primary" />
                    <span className="truncate">{w.name}</span>
                  </DropdownMenuItem>
                ))}
              </DropdownMenuGroup>
            </>
          )}

          {closedList.length > 0 && (
            <>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                {closedList.map((w) => (
                  <DropdownMenuItem key={w.name} onSelect={() => handleOpen(w.name)}>
                    <span aria-hidden className="size-2" />
                    <span className="truncate">{w.name}</span>
                  </DropdownMenuItem>
                ))}
              </DropdownMenuGroup>
            </>
          )}
        </DropdownMenuContent>
      </DropdownMenu>

      <WorkspaceFormDialog
        mode="create"
        open={createOpen}
        onOpenChange={setCreateOpen}
        onSuccess={refresh}
      />
    </>
  )
}
