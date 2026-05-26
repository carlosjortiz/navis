import { afterEach, beforeEach, describe, expect, test, vi } from "vitest"
import { renderHook, waitFor } from "@testing-library/react"
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks"

import { useOpenWorkspaces } from "./use-open-workspaces"

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}))

beforeEach(() => {
  clearMocks()
})

afterEach(() => {
  clearMocks()
})

describe("useOpenWorkspaces", () => {
  test("bootstraps the snapshot from get_open_workspaces on first subscribe", async () => {
    mockIPC((cmd) => {
      if (cmd === "get_open_workspaces") {
        return ["beta", "alpha"]
      }
      throw new Error(`unexpected command: ${cmd}`)
    })

    const { result } = renderHook(() => useOpenWorkspaces())

    await waitFor(() => {
      expect(result.current).toEqual(["alpha", "beta"])
    })
  })
})
