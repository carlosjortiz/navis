import { Copy, Minus, Square, X } from "lucide-react"
import { useEffect, useState } from "react"
import { useTranslation } from "react-i18next"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { Button } from "@/components/ui/button"

type TitleBarProps = {
  workspaceName?: string
}

export function TitleBar({ workspaceName }: TitleBarProps = {}) {
  const { t } = useTranslation()
  const [isMaximized, setIsMaximized] = useState(false)

  useEffect(() => {
    const appWindow = getCurrentWindow()
    let unlisten: (() => void) | undefined

    void appWindow.isMaximized().then(setIsMaximized)

    void appWindow
      .onResized(async () => {
        setIsMaximized(await appWindow.isMaximized())
      })
      .then((fn) => {
        unlisten = fn
      })

    return () => {
      unlisten?.()
    }
  }, [])

  const handleMinimize = () => getCurrentWindow().minimize()
  const handleToggleMaximize = () => getCurrentWindow().toggleMaximize()
  const handleClose = () => getCurrentWindow().close()

  const titleText = workspaceName ? `Navis — ${workspaceName}` : t("app.titleBarFallback")

  return (
    <div
      data-tauri-drag-region
      className="flex h-9 shrink-0 items-center justify-between border-b border-border bg-background select-none"
    >
      <div data-tauri-drag-region className="min-w-0 px-3 text-sm text-muted-foreground truncate">
        {titleText}
      </div>
      <div className="flex">
        <Button
          size="icon-sm"
          variant="ghost"
          onClick={handleMinimize}
          aria-label="Minimize"
          className="rounded-none"
        >
          <Minus />
        </Button>
        <Button
          size="icon-sm"
          variant="ghost"
          onClick={handleToggleMaximize}
          aria-label={isMaximized ? "Restore" : "Maximize"}
          className="rounded-none"
        >
          {isMaximized ? <Copy className="rotate-90" /> : <Square />}
        </Button>
        <Button
          size="icon-sm"
          variant="ghost"
          onClick={handleClose}
          aria-label="Close"
          className="rounded-none hover:bg-destructive/20 hover:text-destructive"
        >
          <X />
        </Button>
      </div>
    </div>
  )
}
