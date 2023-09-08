import { StackActions, useNavigation } from "@react-navigation/native";
import { randomUUID } from "expo-crypto";
import { useCallback, useEffect, useState } from "react";
import type { CheckList } from "../types/check_list";
import type { DateString } from "../types/date_string";
import type { Item, ItemId } from "../types/item";
import type { Store } from "../types/store";

const store: Store = {
  checkLists: {},
  items: {
    allIds: [],
    byId: {},
  },
};

function addCheckList(checkList: CheckList): void {
  if (store.checkLists[checkList.date] !== undefined) return;
  store.checkLists[checkList.date] = checkList;
}

function newCheckList({ date }: { date: DateString }): CheckList {
  return {
    checked: {},
    date,
  };
}

function updateCheckList(
  date: DateString,
  itemId: ItemId,
  checked: boolean
): void {
  const checkList = store.checkLists[date];
  if (checkList === undefined) return;
  checkList.checked[itemId] = checked;
}

function newItem(props: Omit<Item, "id">): Item {
  const { name } = props;
  return {
    id: randomUUID(),
    name,
  };
}

function addItem(item: Item): void {
  store.items.allIds.push(item.id);
  store.items.byId[item.id] = item;
}

export function useTodayScreen(): {
  checked: Record<ItemId, boolean>;
  handleButtonOnPress: () => void;
  handleFABOnPress: () => void;
  handleListItemOnPress: (item: Item) => () => void;
  items: Item[] | null;
} {
  const navigation = useNavigation();
  const [items, setItems] = useState<Item[] | null>(null);
  const [checkList, setCheckList] = useState<CheckList | null>(null);

  useEffect(() => {
    if (items !== null) return;
    setItems(
      store.items.allIds
        .map((id: ItemId): Item | undefined => store.items.byId[id])
        .filter((item: Item | undefined): item is Item => item !== undefined)
    );
  }, [items]);

  useEffect(() => {
    if (checkList !== null) return;
    const today = new Date().toISOString().slice(0, 10);
    const created = store.checkLists[today] ?? newCheckList({ date: today });
    addCheckList(created);
    setCheckList({ ...created });
  }, [checkList]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      if (checkList === null) return;
      updateCheckList(checkList.date, item.id, !checkList.checked[item.id]);
      setCheckList({ ...checkList });
    },
    [checkList]
  );

  const handleFABOnPress = useCallback(() => {
    if (items === null) return;
    const item = newItem({ name: `Item ${items.length}` });
    addItem(item);
    setItems([...items, item]);
  }, [items]);

  return {
    checked: checkList?.checked ?? {},
    handleButtonOnPress,
    handleFABOnPress,
    handleListItemOnPress,
    items,
  };
}
