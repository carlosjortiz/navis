import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

import type { Settings } from "@/bindings";

const EVENT = "settings-preview";

export const emitSettingsPreview = (settings: Settings): Promise<void> =>
  emit(EVENT, settings);

export const onSettingsPreview = (
  cb: (settings: Settings) => void,
): Promise<UnlistenFn> => listen<Settings>(EVENT, (event) => cb(event.payload));
