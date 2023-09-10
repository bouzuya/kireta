import type { DateString } from "./date_string";
import type { Item, ItemId } from "./item";

export type Command =
  | {
      payload: {
        item: Item;
      };
      type: "addItem";
    }
  | {
      payload: {
        checked: boolean;
        date: DateString;
        itemId: ItemId;
      };
      type: "setChecked";
    };

export type Store = {
  // 選択した項目の一覧
  checked: {
    // 日別
    byDate: Record<DateString, Record<ItemId, boolean>>;
    // 項目別
    byItem: Record<ItemId, Record<DateString, boolean>>;
  };
  // 選択可能な項目の一覧
  items: {
    allIds: ItemId[];
    byId: Record<ItemId, Item>;
  };
};

export function getChecked(
  self: Store,
  date: DateString,
  itemId: ItemId
): boolean {
  return self.checked.byDate[date]?.[itemId] ?? false;
}

export function getCheckedItemIdsByDate(
  self: Store,
  date: DateString
): ItemId[] {
  const byDate = self.checked.byDate[date] ?? {};
  return Object.entries(byDate)
    .filter(([_, checked]: [ItemId, boolean]): boolean => checked)
    .map(([id, _]: [ItemId, boolean]): ItemId => id);
}

export function getCheckedDatesByItemId(
  self: Store,
  itemId: ItemId
): DateString[] {
  const byItem = self.checked.byItem[itemId] ?? {};
  return Object.entries(byItem)
    .filter(([_, checked]: [DateString, boolean]): boolean => checked)
    .map(([date, _]: [DateString, boolean]): DateString => date);
}

export function getItem(self: Store, itemId: ItemId): Item | null {
  return self.items.byId[itemId] ?? null;
}

export function getItems(self: Store): Item[] {
  return self.items.allIds
    .map((id: ItemId): Item | undefined => self.items.byId[id])
    .filter((item: Item | undefined): item is Item => item !== undefined);
}

export function handle(mutSelf: Store, command: Command): void {
  switch (command.type) {
    case "addItem":
      addItem(mutSelf, command.payload.item);
      break;
    case "setChecked":
      setChecked(
        mutSelf,
        command.payload.date,
        command.payload.itemId,
        command.payload.checked
      );
      break;
  }
}

export function newStore(): Store {
  return {
    checked: {
      byDate: {},
      byItem: {},
    },
    items: {
      allIds: [],
      byId: {},
    },
  };
}

function addItem(mutSelf: Store, item: Item): void {
  if (mutSelf.items.byId[item.id] !== undefined)
    throw new Error("already exists");
  mutSelf.items.allIds.push(item.id);
  mutSelf.items.byId[item.id] = item;
}

function setChecked(
  mutSelf: Store,
  date: DateString,
  itemId: ItemId,
  checked: boolean
): void {
  // itemId が items に存在しない可能性はあるが、検査しない
  const byDate = mutSelf.checked.byDate[date] ?? {};
  byDate[itemId] = checked;
  mutSelf.checked.byDate[date] = byDate;
  const byItem = mutSelf.checked.byItem[itemId] ?? {};
  byItem[date] = checked;
  mutSelf.checked.byItem[itemId] = byItem;
}
