import { useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { DateString } from "@/components/TodayScreen/types/date_string";
import { getAllDates } from "@/components/TodayScreen/types/store";

export function useHistoryScreen(): {
  dates: DateString[] | null;
} {
  const [dates, setDates] = useState<DateString[] | null>(null);
  const { store } = useStore();

  useEffect(() => {
    if (dates !== null) return;
    setDates(getAllDates(store));
  }, [dates, store]);

  return {
    dates,
  };
}
