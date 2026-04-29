export default function App() {
  return (
    <div className="min-h-screen bg-primary-50 font-sans flex items-center justify-center p-8">
      <div className="bg-white rounded-lg shadow-lg p-8 max-w-md">
        <h1 className="text-3xl font-bold text-primary-900 mb-2">Hello Navis</h1>
        <p className="text-primary-500 mb-6">Tailwind 4 + custom theme funcionando.</p>
        <button className="px-4 py-2 bg-primary-500 text-white rounded hover:bg-primary-900 transition-colors">
          Smoke test button
        </button>
      </div>
    </div>
  )
}
