import { useTranslation } from "react-i18next"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

type SettingsDialogProps = {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const { t } = useTranslation()

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t("settings.title")}</DialogTitle>
        </DialogHeader>
        <Tabs defaultValue="theme" className="mt-2">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="theme">{t("settings.tabs.theme")}</TabsTrigger>
            <TabsTrigger value="language">{t("settings.tabs.language")}</TabsTrigger>
          </TabsList>
          <TabsContent value="theme" className="text-sm text-muted-foreground py-4">
            {t("settings.theme.placeholder")}
          </TabsContent>
          <TabsContent value="language" className="text-sm text-muted-foreground py-4">
            {t("settings.language.placeholder")}
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  )
}
