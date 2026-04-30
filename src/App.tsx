import { Globe, Search, Send, Settings } from "lucide-react"
import { Button } from "@/components/ui/button"

export default function App() {
  return (
    <div className="min-h-screen bg-primary-50 font-sans flex items-center justify-center p-8">
      <div className="bg-white rounded-lg shadow-lg p-8 max-w-md w-full space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-primary-900 mb-2">Hello Navis</h1>
          <p className="text-primary-500">
            Tailwind 4 + theme custom + shadcn/ui + Lucide icons funcionando.
          </p>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">Standalone icons</p>
          <div className="flex items-center gap-4 text-foreground">
            <Globe className="size-5" />
            <Search className="size-5" />
            <Send className="size-5" />
            <Settings className="size-5" />
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">Icons inside Button</p>
          <div className="flex flex-wrap gap-2">
            <Button variant="default">
              <Send />
              Send request
            </Button>
            <Button variant="outline">
              <Search />
              Search
            </Button>
          </div>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">Icon-only buttons</p>
          <div className="flex gap-2">
            <Button size="icon" variant="default" aria-label="Send">
              <Send />
            </Button>
            <Button size="icon" variant="outline" aria-label="Settings">
              <Settings />
            </Button>
            <Button size="icon" variant="ghost" aria-label="Search">
              <Search />
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}
