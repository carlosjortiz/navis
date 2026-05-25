import { useTranslation } from "react-i18next"
import { Button } from "@/components/ui/button"

export interface EmptyStateProps {
  onCreate: () => void
}

export function EmptyState({ onCreate }: EmptyStateProps) {
  const { t } = useTranslation()
  return (
    <div className="bg-card rounded-lg shadow-lg p-8 max-w-md w-full space-y-3 text-center">
      <h1 className="text-3xl font-bold">{t("workspaceSelector.empty.heading")}</h1>
      <p className="text-muted-foreground">{t("workspaceSelector.empty.body")}</p>
      <Button onClick={onCreate}>
        {t("workspaceSelector.empty.createCta")}
      </Button>
    </div>
  )
}
