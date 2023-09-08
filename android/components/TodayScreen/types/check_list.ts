import type { DateString } from "./date_string";
import type { ItemId } from "./item";

export type CheckList = {
  checked: Record<ItemId, boolean>;
  date: DateString;
};

export function newCheckList({ date }: { date: DateString }): CheckList {
  return {
    checked: {},
    date,
  };
}
