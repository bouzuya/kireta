import { useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { DateString } from "@/types/date_string";
import { findAllCheckListDates } from "@/types/store";

export function useHistoryScreen(): {
  dates: DateString[] | null;
} {
  const [dates, setDates] = useState<DateString[] | null>(null);
  const { store } = useStore();

  useEffect(() => {
    if (dates !== null) return;
    setDates(findAllCheckListDates(store));
  }, [dates, store]);

  return {
    dates,
  };
}
