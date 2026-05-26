import { useState } from "react"
import { useTranslation } from "react-i18next"
import { toast } from "sonner"

import { commands } from "@/bindings"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog"
import { notifyError } from "@/lib/notify"

export interface DeleteWorkspaceAlertProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  workspaceName: string
  onSuccess: () => void
}

export function DeleteWorkspaceAlert({
  open,
  onOpenChange,
  workspaceName,
  onSuccess,
}: DeleteWorkspaceAlertProps) {
  const { t } = useTranslation()
  const [busy, setBusy] = useState(false)

  const handleConfirm = async (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault()
    setBusy(true)
    const result = await commands.deleteWorkspace(workspaceName)
    setBusy(false)
    if (result.status === "error") {
      notifyError(result.error, t)
      return
    }
    toast.success(t("workspaceSelector.toast.deleted"))
    onOpenChange(false)
    onSuccess()
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("workspaceSelector.delete.title")}</AlertDialogTitle>
          <AlertDialogDescription>
            {t("workspaceSelector.delete.body", { name: workspaceName })}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={busy}>
            {t("workspaceSelector.delete.cancel")}
          </AlertDialogCancel>
          <AlertDialogAction
            variant="destructive"
            disabled={busy}
            onClick={handleConfirm}
          >
            {t("workspaceSelector.delete.confirm")}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
