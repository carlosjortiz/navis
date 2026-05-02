import { useSyncExternalStore } from "react";

import { commands, events, type Settings } from "@/bindings";

const subscribers = new Set<() => void>();
let snapshot: Settings | null = null;

const notify = () => {
  for (const cb of subscribers) cb();
};

void (async () => {
  const result = await commands.getSettings();
  if (result.status === "ok") {
    snapshot = result.data;
    notify();
  }
})();

void events.settingsChanged.listen((evt) => {
  snapshot = evt.payload;
  notify();
});

const subscribe = (cb: () => void) => {
  subscribers.add(cb);
  return () => {
    subscribers.delete(cb);
  };
};

const getSnapshot = () => snapshot;

export function useSettings(): Settings | null {
  return useSyncExternalStore(subscribe, getSnapshot);
}
