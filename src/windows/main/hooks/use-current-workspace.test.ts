import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";

import "@/i18n";
import { useCurrentWorkspace } from "./use-current-workspace";

vi.mock("@/lib/notify", () => ({
  notifyError: vi.fn(),
}));

const { notifyError } = await import("@/lib/notify");

const setSearch = (search: string) => {
  window.history.pushState({}, "", search ? `/?${search}` : "/");
};

beforeEach(() => {
  setSearch("");
  clearMocks();
  vi.mocked(notifyError).mockReset();
});

afterEach(() => {
  clearMocks();
});

describe("useCurrentWorkspace", () => {
  test("returns the workspace when get_workspace succeeds", async () => {
    setSearch("workspace=foo");
    mockIPC((cmd, args) => {
      if (cmd === "get_workspace") {
        expect(args).toEqual({ name: "foo" });
        return { name: "foo", description: "hello" };
      }
      throw new Error(`unexpected command: ${cmd}`);
    });

    const { result } = renderHook(() => useCurrentWorkspace());

    await waitFor(() => {
      expect(result.current).toEqual({ name: "foo", description: "hello" });
    });
    expect(notifyError).not.toHaveBeenCalled();
  });

  test("returns null and notifies when get_workspace errors", async () => {
    setSearch("workspace=bar");
    const appError = { kind: "notFound", entity: "workspace", name: "bar" };
    mockIPC((cmd) => {
      if (cmd === "get_workspace") {
        throw appError;
      }
      throw new Error(`unexpected command: ${cmd}`);
    });

    const { result } = renderHook(() => useCurrentWorkspace());

    await waitFor(() => {
      expect(notifyError).toHaveBeenCalledWith(appError, expect.any(Function));
    });
    expect(result.current).toBeNull();
  });

  test("returns null and skips the IPC call when the workspace query param is missing", async () => {
    setSearch("");
    const handler = vi.fn();
    mockIPC((cmd, args) => {
      handler(cmd, args);
      throw new Error("no command should run");
    });

    const { result } = renderHook(() => useCurrentWorkspace());

    await new Promise((resolve) => setTimeout(resolve, 30));
    expect(result.current).toBeNull();
    expect(handler).not.toHaveBeenCalled();
    expect(notifyError).not.toHaveBeenCalled();
  });
});
