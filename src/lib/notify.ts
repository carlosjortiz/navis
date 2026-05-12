import { toast } from "sonner"
import type { TFunction } from "i18next"
import { formatAppError } from "@/lib/app-error"

export function notifyError(err: unknown, t: TFunction): void {
  toast.error(formatAppError(err, t))
}
