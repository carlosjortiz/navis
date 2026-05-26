import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { useTranslation } from "react-i18next"
import { toast } from "sonner"

import { commands } from "@/bindings"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { notifyError } from "@/lib/notify"
import {
  workspaceCreateSchema,
  workspaceRenameSchema,
  type WorkspaceCreateValues,
  type WorkspaceRenameValues,
} from "./workspace-form.schema"

export type WorkspaceFormDialogProps =
  | {
      mode: "create"
      open: boolean
      onOpenChange: (open: boolean) => void
      onSuccess: () => void
    }
  | {
      mode: "rename"
      open: boolean
      onOpenChange: (open: boolean) => void
      onSuccess: () => void
      oldName: string
    }

export function WorkspaceFormDialog(props: WorkspaceFormDialogProps) {
  const { t } = useTranslation()

  return (
    <Dialog open={props.open} onOpenChange={props.onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t(`workspaceSelector.dialog.${props.mode}.title`)}</DialogTitle>
          <DialogDescription className="sr-only">
            {t(`workspaceSelector.dialog.${props.mode}.title`)}
          </DialogDescription>
        </DialogHeader>
        {props.open && (
          props.mode === "create" ? (
            <CreateForm
              onClose={() => props.onOpenChange(false)}
              onSuccess={props.onSuccess}
              t={t}
            />
          ) : (
            <RenameForm
              oldName={props.oldName}
              onClose={() => props.onOpenChange(false)}
              onSuccess={props.onSuccess}
              t={t}
            />
          )
        )}
      </DialogContent>
    </Dialog>
  )
}

type TFn = ReturnType<typeof useTranslation>["t"]

interface CreateFormProps {
  onClose: () => void
  onSuccess: () => void
  t: TFn
}

function CreateForm({ onClose, onSuccess, t }: CreateFormProps) {
  const form = useForm<WorkspaceCreateValues>({
    resolver: zodResolver(workspaceCreateSchema),
    defaultValues: { name: "", description: "" },
  })

  const onSubmit = async (values: WorkspaceCreateValues) => {
    const description = values.description?.trim() ? values.description.trim() : null
    const result = await commands.createWorkspace(values.name, description)
    if (result.status === "error") {
      notifyError(result.error, t)
      return
    }
    toast.success(t("workspaceSelector.toast.created"))
    onClose()
    onSuccess()
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-4">
        <FormField
          control={form.control}
          name="name"
          render={({ field, fieldState }) => (
            <FormItem>
              <FormLabel>{t("workspaceSelector.form.name.label")}</FormLabel>
              <FormControl>
                <Input
                  placeholder={t("workspaceSelector.form.name.placeholder")}
                  autoFocus
                  {...field}
                />
              </FormControl>
              <FormMessage>
                {fieldState.error?.message ? t(fieldState.error.message) : null}
              </FormMessage>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="description"
          render={({ field, fieldState }) => (
            <FormItem>
              <FormLabel>{t("workspaceSelector.form.description.label")}</FormLabel>
              <FormControl>
                <Textarea
                  placeholder={t("workspaceSelector.form.description.placeholder")}
                  rows={3}
                  {...field}
                />
              </FormControl>
              <FormDescription>
                {t("workspaceSelector.form.description.optional")}
              </FormDescription>
              <FormMessage>
                {fieldState.error?.message ? t(fieldState.error.message) : null}
              </FormMessage>
            </FormItem>
          )}
        />
        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>
            {t("workspaceSelector.dialog.cancel")}
          </Button>
          <Button type="submit" disabled={form.formState.isSubmitting}>
            {t("workspaceSelector.dialog.create.submit")}
          </Button>
        </DialogFooter>
      </form>
    </Form>
  )
}

interface RenameFormProps {
  oldName: string
  onClose: () => void
  onSuccess: () => void
  t: TFn
}

function RenameForm({ oldName, onClose, onSuccess, t }: RenameFormProps) {
  const form = useForm<WorkspaceRenameValues>({
    resolver: zodResolver(workspaceRenameSchema),
    defaultValues: { name: oldName },
  })

  const onSubmit = async (values: WorkspaceRenameValues) => {
    if (values.name === oldName) {
      onClose()
      return
    }
    const result = await commands.renameWorkspace(oldName, values.name)
    if (result.status === "error") {
      notifyError(result.error, t)
      return
    }
    toast.success(t("workspaceSelector.toast.renamed"))
    onClose()
    onSuccess()
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-4">
        <FormField
          control={form.control}
          name="name"
          render={({ field, fieldState }) => (
            <FormItem>
              <FormLabel>{t("workspaceSelector.form.name.label")}</FormLabel>
              <FormControl>
                <Input
                  placeholder={t("workspaceSelector.form.name.placeholder")}
                  autoFocus
                  {...field}
                />
              </FormControl>
              <FormMessage>
                {fieldState.error?.message ? t(fieldState.error.message) : null}
              </FormMessage>
            </FormItem>
          )}
        />
        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>
            {t("workspaceSelector.dialog.cancel")}
          </Button>
          <Button type="submit" disabled={form.formState.isSubmitting}>
            {t("workspaceSelector.dialog.rename.submit")}
          </Button>
        </DialogFooter>
      </form>
    </Form>
  )
}
