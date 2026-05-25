import { Plus } from "lucide-react"
import { useTranslation } from "react-i18next"
import { Button } from "@/components/ui/button"

export interface WorkspaceHeaderProps {
  onCreate: () => void
}

export function WorkspaceHeader({ onCreate }: WorkspaceHeaderProps) {
  const { t } = useTranslation()
  return (
    <div className="sticky top-0 z-10 flex items-center justify-between gap-4 border-b bg-background/80 px-6 py-3 backdrop-blur-md">
      <h2 className="text-base font-semibold">
        {t("workspaceSelector.header.title")}
      </h2>
      <Button size="sm" onClick={onCreate}>
        <Plus className="size-4" />
        {t("workspaceSelector.actions.create")}
      </Button>
    </div>
  )
}
