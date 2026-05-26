import { afterEach, beforeEach, describe, expect, test, vi } from "vitest"
import { render, screen, waitFor, within } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks"

import "@/i18n"
import { WorkspaceSwitcher } from "./workspace-switcher"

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}))

vi.mock("@/lib/notify", () => ({
  notifyError: vi.fn(),
}))

beforeEach(() => {
  clearMocks()
})

afterEach(() => {
  clearMocks()
})

function setupCommands(options: {
  workspaces: Array<{ name: string; description?: string | null }>
  open: string[]
  onOpenWorkspace?: (name: string) => void
}) {
  mockIPC((cmd, args) => {
    switch (cmd) {
      case "list_workspaces":
        return options.workspaces
      case "get_open_workspaces":
        return options.open
      case "open_workspace":
        options.onOpenWorkspace?.((args as { name: string }).name)
        return null
      default:
        throw new Error(`unexpected command: ${cmd}`)
    }
  })
}

describe("WorkspaceSwitcher", () => {
  test("renders open workspaces under the 'Open' label and closed ones below", async () => {
    setupCommands({
      workspaces: [
        { name: "alpha", description: null },
        { name: "beta", description: null },
        { name: "gamma", description: null },
      ],
      open: ["alpha", "gamma"],
    })

    const user = userEvent.setup()
    render(<WorkspaceSwitcher currentWorkspaceName="alpha" />)

    await user.click(screen.getByRole("button", { name: /switch workspace/i }))

    const menu = await screen.findByRole("menu")
    await waitFor(() => {
      expect(within(menu).getByText("Open")).toBeInTheDocument()
    })

    const items = within(menu).getAllByRole("menuitem")
    const labels = items.map((el) => el.textContent?.trim())
    expect(labels).toEqual(["New workspace", "alpha", "gamma", "beta"])
  })

  test("invokes open_workspace when clicking a workspace item", async () => {
    const onOpen = vi.fn()
    setupCommands({
      workspaces: [
        { name: "alpha", description: null },
        { name: "beta", description: null },
      ],
      open: ["alpha"],
      onOpenWorkspace: onOpen,
    })

    const user = userEvent.setup()
    render(<WorkspaceSwitcher currentWorkspaceName="alpha" />)

    await user.click(screen.getByRole("button", { name: /switch workspace/i }))
    const menu = await screen.findByRole("menu")
    await within(menu).findByText("beta")

    await user.click(within(menu).getByRole("menuitem", { name: /^beta$/ }))

    await waitFor(() => {
      expect(onOpen).toHaveBeenCalledWith("beta")
    })
  })

  test("opens the create dialog when clicking + New workspace", async () => {
    setupCommands({ workspaces: [], open: [] })

    const user = userEvent.setup()
    render(<WorkspaceSwitcher currentWorkspaceName="alpha" />)

    await user.click(screen.getByRole("button", { name: /switch workspace/i }))
    const menu = await screen.findByRole("menu")
    await user.click(within(menu).getByRole("menuitem", { name: /new workspace/i }))

    expect(await screen.findByRole("dialog")).toBeInTheDocument()
    expect(screen.getByRole("heading", { name: /create workspace/i })).toBeInTheDocument()
  })
})
