import { useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckList } from "@/types/check_list";
import { findAllCheckLists } from "@/types/store";

export function useHistoryScreen(): {
  checkLists: CheckList[] | null;
} {
  const [checkLists, setCheckLists] = useState<CheckList[] | null>(null);
  const { store } = useStore();

  useEffect(() => {
    if (checkLists !== null) return;
    void (async () => {
      setCheckLists(await findAllCheckLists(store));
    })();
  }, [checkLists, store]);

  return {
    checkLists,
  };
}
