import { MoreVertical } from "lucide-react"
import { useTranslation } from "react-i18next"
import type { Workspace } from "@/bindings"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"

export interface WorkspaceCardProps {
  workspace: Workspace
  onOpen: (name: string) => void
  onRename: (workspace: Workspace) => void
  onDelete: (workspace: Workspace) => void
}

export function WorkspaceCard({
  workspace,
  onOpen,
  onRename,
  onDelete,
}: WorkspaceCardProps) {
  const { t } = useTranslation()

  const open = () => onOpen(workspace.name)

  const handleKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault()
      open()
    }
  }

  return (
    <div
      role="button"
      tabIndex={0}
      onClick={open}
      onKeyDown={handleKeyDown}
      className="bg-card hover:bg-accent transition-colors rounded-lg shadow-md p-4 flex items-center justify-between gap-4 cursor-pointer focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
    >
      <div className="flex-1 min-w-0">
        <h3 className="font-semibold truncate">{workspace.name}</h3>
        <p className="text-sm text-muted-foreground truncate">
          {workspace.description ?? t("workspaceSelector.card.noDescription")}
        </p>
      </div>
      <DropdownMenu>
        <DropdownMenuTrigger asChild onClick={(e) => e.stopPropagation()}>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t("workspaceSelector.actions.rename")}
          >
            <MoreVertical className="size-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          align="end"
          onClick={(e) => e.stopPropagation()}
        >
          <DropdownMenuItem
            onSelect={() => onRename(workspace)}
          >
            {t("workspaceSelector.actions.rename")}
          </DropdownMenuItem>
          <DropdownMenuItem
            variant="destructive"
            onSelect={() => onDelete(workspace)}
          >
            {t("workspaceSelector.actions.delete")}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  )
}
