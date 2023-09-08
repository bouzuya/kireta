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

export function newStore(): Store {
  return {
    checkLists: {},
    items: {
      allIds: [],
      byId: {},
    },
  };
}

export function addCheckList(mutSelf: Store, checkList: CheckList): void {
  if (mutSelf.checkLists[checkList.date] !== undefined) return;
  mutSelf.checkLists[checkList.date] = checkList;
}

export function addItem(mutSelf: Store, item: Item): void {
  mutSelf.items.allIds.push(item.id);
  mutSelf.items.byId[item.id] = item;
}

// TODO: Extract updateCheckList to check_list mod
export function updateCheckList(
  mutSelf: Store,
  date: DateString,
  itemId: ItemId,
  checked: boolean
): void {
  const checkList = mutSelf.checkLists[date];
  if (checkList === undefined) return;
  checkList.checked[itemId] = checked;
}
