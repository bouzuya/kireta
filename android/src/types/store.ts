import {
  newCheckList,
  type CheckList,
  type CheckListId,
} from "@/types/check_list";
import type { DateString } from "@/types/date_string";
import { newItem, type Item, type ItemId } from "@/types/item";

export type Command =
  | {
      payload: {
        checkList: CheckList;
      };
      type: "addCheckList";
    }
  | {
      payload: {
        item: Item;
      };
      type: "addItem";
    }
  | {
      payload: {
        checkListId: DateString;
        checked: boolean;
        itemId: ItemId;
      };
      type: "setChecked";
    };

export type Store = {
  checkLists: {
    allIds: CheckListId[];
    byDate: Record<DateString, CheckListId>;
    byId: Record<CheckListId, CheckList>;
  };
  checked: {
    byCheckListId: Record<CheckListId, Record<ItemId, boolean>>;
    byItemId: Record<ItemId, Record<CheckListId, boolean>>;
  };
  items: {
    allIds: ItemId[];
    byId: Record<ItemId, Item>;
  };
};

export function findAllCheckListDates(self: Store): DateString[] {
  return findAllCheckListIds(self)
    .map((id): DateString | undefined => self.checkLists.byId[id]?.date)
    .filter((date): date is DateString => date !== undefined);
}

export function findAllCheckListIds(self: Store): CheckListId[] {
  return [...self.checkLists.allIds];
}

export function findAllCheckLists(self: Store): CheckList[] {
  return findAllCheckListIds(self)
    .map((id) => findCheckList(self, id))
    .filter((checkList): checkList is CheckList => checkList !== null)
    .sort((a, b) => a.date < b.date ? 1 : a.date === b.date ? 0 : -1);
}

export function findAllItemIds(self: Store): ItemId[] {
  return [...self.items.allIds];
}

export function findAllItems(self: Store): Item[] {
  return findAllItemIds(self)
    .map((id) => findItem(self, id))
    .filter((item): item is Item => item !== null);
}

export function findCheckListByDate(
  self: Store,
  date: DateString
): CheckList | null {
  const id = self.checkLists.byDate[date];
  if (id === undefined) return null;
  return findCheckList(self, id);
}

export function findCheckList(self: Store, id: CheckListId): CheckList | null {
  return self.checkLists.byId[id] ?? null;
}

export function findCheckedCheckListIdsByItemId(
  self: Store,
  itemId: ItemId
): CheckListId[] {
  const byItemId = self.checked.byItemId[itemId] ?? {};
  return Object.entries(byItemId)
    .filter(([_, checked]: [CheckListId, boolean]): boolean => checked)
    .map(([id, _]: [CheckListId, boolean]): CheckListId => id);
}

export function findCheckedItemIdsByCheckListId(
  self: Store,
  checkListId: CheckListId
): ItemId[] {
  const byCheckListId = self.checked.byCheckListId[checkListId] ?? {};
  return Object.entries(byCheckListId)
    .filter(([_, checked]: [ItemId, boolean]): boolean => checked)
    .map(([id, _]: [ItemId, boolean]): ItemId => id);
}

export function findChecked(
  self: Store,
  checkListId: CheckListId,
  itemId: ItemId
): boolean {
  return self.checked.byCheckListId[checkListId]?.[itemId] ?? false;
}

export function findItem(self: Store, id: ItemId): Item | null {
  return self.items.byId[id] ?? null;
}

export function handle(mutSelf: Store, command: Command): void {
  switch (command.type) {
    case "addCheckList":
      storeCheckList(mutSelf, command.payload.checkList);
      break;
    case "addItem":
      storeItem(mutSelf, command.payload.item);
      break;
    case "setChecked":
      storeChecked(
        mutSelf,
        command.payload.checkListId,
        command.payload.itemId,
        command.payload.checked
      );
      break;
  }
}

export function newStore(): Store {
  const store = {
    checkLists: {
      allIds: [],
      byDate: {},
      byId: {},
    },
    checked: {
      byCheckListId: {},
      byItemId: {},
    },
    items: {
      allIds: [],
      byId: {},
    },
  };

  // DEBUG
  const item1 = newItem({ name: "項目1" });
  const item2 = newItem({ name: "項目2" });
  const checkList1 = newCheckList({ date: "2023-09-07" });
  const checkList2 = newCheckList({ date: "2023-09-08" });
  const checkList3 = newCheckList({ date: "2023-09-09" });
  storeItem(store, item1);
  storeItem(store, item2);
  storeCheckList(store, checkList1);
  storeCheckList(store, checkList2);
  storeCheckList(store, checkList3);
  storeChecked(store, checkList1.id, item1.id, true);
  storeChecked(store, checkList2.id, item1.id, true);
  storeChecked(store, checkList3.id, item1.id, true);
  storeChecked(store, checkList3.id, item2.id, true);

  return store;
}

function storeCheckList(self: Store, checkList: CheckList): void {
  if (self.checkLists.byId[checkList.id] !== undefined)
    throw new Error("The checkListId already exists");
  if (self.checkLists.byDate[checkList.date] !== undefined)
    throw new Error("The checkListDate already exists");
  self.checkLists.allIds.push(checkList.id);
  self.checkLists.byId[checkList.id] = checkList;
  self.checkLists.byDate[checkList.date] = checkList.id;
}

function storeChecked(
  mutSelf: Store,
  checkListId: CheckListId,
  itemId: ItemId,
  checked: boolean
): void {
  // checkListId が checkLists に存在しない可能性はあるが、検査しない
  const byCheckListId = mutSelf.checked.byCheckListId[checkListId] ?? {};
  byCheckListId[itemId] = checked;
  mutSelf.checked.byCheckListId[checkListId] = byCheckListId;
  // itemId が items に存在しない可能性はあるが、検査しない
  const byItemId = mutSelf.checked.byItemId[itemId] ?? {};
  byItemId[checkListId] = checked;
  mutSelf.checked.byItemId[itemId] = byItemId;
}

function storeItem(self: Store, item: Item): void {
  if (self.items.byId[item.id] !== undefined)
    throw new Error("The itemId already exists");
  self.items.allIds.push(item.id);
  self.items.byId[item.id] = item;
}
