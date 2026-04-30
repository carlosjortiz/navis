import { Minus, Square, X } from "lucide-react"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { Button } from "@/components/ui/button"

export function TitleBar() {
  const handleMinimize = () => getCurrentWindow().minimize()
  const handleMaximize = () => getCurrentWindow().toggleMaximize()
  const handleClose = () => getCurrentWindow().close()

  return (
    <div
      data-tauri-drag-region
      className="flex h-9 shrink-0 items-center justify-end border-b border-border bg-background select-none"
    >
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
          onClick={handleMaximize}
          aria-label="Maximize"
          className="rounded-none"
        >
          <Square />
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
