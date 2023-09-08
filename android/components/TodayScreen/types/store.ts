import type { CheckList } from "./check_list";
import type { DateString } from "./date_string";
import type { Item, ItemId } from "./item";

export type Store = {
  // 選択した項目の一覧
  checkLists: Record<DateString, CheckList>;
  // 選択可能な項目の一覧
  items: {
    allIds: ItemId[];
    byId: Record<ItemId, Item>;
  };
};
