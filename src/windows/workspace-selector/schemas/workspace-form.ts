import { z } from "zod"

const FORBIDDEN_CHARS = /[<>:"/\\|?*\x00]/
const RESERVED_NAMES = new Set([
  "CON", "PRN", "AUX", "NUL",
  "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
  "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
])

const nameSchema = z
  .string()
  .min(1, { message: "workspaceSelector.validation.empty" })
  .max(255, { message: "workspaceSelector.validation.tooLong" })
  .refine((n) => !FORBIDDEN_CHARS.test(n), {
    message: "workspaceSelector.validation.forbiddenChar",
  })
  .refine((n) => !(n.startsWith(" ") || n.endsWith(" ")), {
    message: "workspaceSelector.validation.leadingTrailingSpace",
  })
  .refine((n) => !(n.startsWith(".") || n.endsWith(".")), {
    message: "workspaceSelector.validation.leadingTrailingDot",
  })
  .refine((n) => n.trim().length > 0, {
    message: "workspaceSelector.validation.whitespaceOnly",
  })
  .refine(
    (n) => {
      const stem = (n.split(".")[0] ?? "").toUpperCase()
      return !RESERVED_NAMES.has(stem)
    },
    { message: "workspaceSelector.validation.reservedName" },
  )

// Soft FE-only limit until the future edit-workspace-fields command lands
// and the backend defines a real bound for description length.
const descriptionSchema = z.string().max(2000).optional().or(z.literal(""))

export const workspaceCreateSchema = z.object({
  name: nameSchema,
  description: descriptionSchema,
})

export const workspaceRenameSchema = z.object({
  name: nameSchema,
})

export type WorkspaceCreateValues = z.infer<typeof workspaceCreateSchema>
export type WorkspaceRenameValues = z.infer<typeof workspaceRenameSchema>
