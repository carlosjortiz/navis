import { TitleBar } from "@/components/title-bar"

export default function App() {
  return (
    <div className="flex flex-col h-screen bg-background text-foreground font-sans">
      <TitleBar />
      <main className="flex-1 overflow-auto flex items-center justify-center p-8">
        <div className="bg-card rounded-lg shadow-lg p-8 max-w-md w-full space-y-3 text-center">
          <h1 className="text-3xl font-bold">Workspace Selector</h1>
          <p className="text-muted-foreground">Pick a workspace to open.</p>
        </div>
      </main>
    </div>
  )
}
