import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { commands, type Workspace } from "@/bindings";
import { notifyError } from "@/lib/notify";

export function useCurrentWorkspace(): Workspace | null {
  const { t } = useTranslation();
  const [workspace, setWorkspace] = useState<Workspace | null>(null);

  useEffect(() => {
    const name = new URLSearchParams(window.location.search).get("workspace");
    if (!name) return;

    void commands.getWorkspace(name).then((result) => {
      if (result.status === "ok") {
        setWorkspace(result.data);
      } else {
        notifyError(result.error, t);
      }
    });
  }, [t]);

  return workspace;
}
