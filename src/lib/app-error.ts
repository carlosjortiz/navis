import type { TFunction } from "i18next"

export type AppError =
  | { kind: "notFound"; entity: string; name: string }
  | { kind: "internal" }

// The `in` operator check is the only reliable way to narrow `unknown` to an
// object with a specific key without casting.
export function isAppError(value: unknown): value is AppError {
  return (
    typeof value === "object" &&
    value !== null &&
    "kind" in value &&
    (value.kind === "notFound" || value.kind === "internal")
  )
}

export function formatAppError(err: unknown, t: TFunction): string {
  if (!isAppError(err)) {
    return t("errors.internal")
  }

  switch (err.kind) {
    case "notFound":
      return t([`errors.notFound.${err.entity}`, "errors.notFound.default"], {
        entity: err.entity,
        name: err.name,
      })
    case "internal":
      return t("errors.internal")
  }
}
