import { useSyncExternalStore } from "react";

import { commands } from "@/bindings";
import type { EffectiveSettings } from "@/lib/settings-defaults";
import { onSettingsPreview } from "@/lib/settings-events";

const subscribers = new Set<() => void>();
let snapshot: EffectiveSettings | null = null;

const notify = () => {
  for (const cb of subscribers) cb();
};

void (async () => {
  const result = await commands.getSettings();
  // Skip if a preview event already populated the snapshot — it carries fresher
  // state (the Settings window started editing before getSettings resolved).
  if (result.status === "ok" && snapshot === null) {
    snapshot = result.data as EffectiveSettings;
    notify();
  }
})();

void onSettingsPreview((next) => {
  snapshot = next as EffectiveSettings;
  notify();
});

const subscribe = (cb: () => void) => {
  subscribers.add(cb);
  return () => {
    subscribers.delete(cb);
  };
};

const getSnapshot = () => snapshot;

export function useSettings(): EffectiveSettings | null {
  return useSyncExternalStore(subscribe, getSnapshot);
}
