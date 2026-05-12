import { useTranslation } from "react-i18next"
import type { Workspace } from "@/bindings"
import { Button } from "@/components/ui/button"

export interface WorkspaceCardProps {
  workspace: Workspace
  onOpen: (name: string) => void
}

export function WorkspaceCard({ workspace, onOpen }: WorkspaceCardProps) {
  const { t } = useTranslation()
  return (
    <div className="bg-card rounded-lg shadow-md p-4 flex items-center justify-between gap-4">
      <div className="flex-1 min-w-0">
        <h3 className="font-semibold truncate">{workspace.name}</h3>
        <p className="text-sm text-muted-foreground truncate">
          {workspace.description ?? t("workspaceSelector.card.noDescription")}
        </p>
      </div>
      <Button onClick={() => onOpen(workspace.name)}>
        {t("workspaceSelector.card.openButton")}
      </Button>
    </div>
  )
}
