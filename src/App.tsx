import { Button } from "@/components/ui/button"

export default function App() {
  return (
    <div className="min-h-screen bg-primary-50 font-sans flex items-center justify-center p-8">
      <div className="bg-white rounded-lg shadow-lg p-8 max-w-md w-full space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-primary-900 mb-2">Hello Navis</h1>
          <p className="text-primary-500">
            Tailwind 4 + theme custom + shadcn/ui funcionando.
          </p>
        </div>

        <div className="space-y-3">
          <p className="text-sm font-medium text-foreground">shadcn Button variants</p>
          <div className="flex flex-wrap gap-2">
            <Button variant="default">Default</Button>
            <Button variant="outline">Outline</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="ghost">Ghost</Button>
            <Button variant="destructive">Destructive</Button>
            <Button variant="link">Link</Button>
          </div>
        </div>
      </div>
    </div>
  )
}
