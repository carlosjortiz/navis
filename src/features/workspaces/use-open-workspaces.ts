import { useSyncExternalStore } from "react"
import { listen } from "@tauri-apps/api/event"
import { commands } from "@/bindings"

const OPEN_WORKSPACES_EVENT = "open-workspaces-changed"

const subscribers = new Set<() => void>()
let snapshot: readonly string[] = Object.freeze([])
let bootstrapped = false

function publish(next: readonly string[]) {
  snapshot = Object.freeze([...next].sort())
  for (const cb of subscribers) cb()
}

function ensureBootstrapped() {
  if (bootstrapped) return
  bootstrapped = true

  void commands.getOpenWorkspaces().then((result) => {
    if (result.status === "ok") publish(result.data)
  })

  // Subscribe once at module scope so all consumers share a single listener;
  // the Promise<UnlistenFn> never resolves to cleanup because the listener
  // lives for the lifetime of the window.
  void listen<string[]>(OPEN_WORKSPACES_EVENT, (event) => {
    publish(event.payload)
  })
}

function subscribe(cb: () => void): () => void {
  ensureBootstrapped()
  subscribers.add(cb)
  return () => {
    subscribers.delete(cb)
  }
}

function getSnapshot(): readonly string[] {
  return snapshot
}

export function useOpenWorkspaces(): readonly string[] {
  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot)
}
