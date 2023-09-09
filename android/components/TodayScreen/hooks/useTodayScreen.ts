import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "../../StoreContext";
import { newCheckList, type CheckList } from "../types/check_list";
import { newItem, type Item, type ItemId } from "../types/item";
import {
  addCheckList,
  addItem,
  getItems,
  updateCheckList,
} from "../types/store";

export function useTodayScreen(): {
  checked: Record<ItemId, boolean>;
  handleButtonOnPress: () => void;
  handleFABOnPress: () => void;
  handleListItemOnPress: (item: Item) => () => void;
  items: Item[] | null;
} {
  const { store } = useStore();
  const navigation = useNavigation();
  const [items, setItems] = useState<Item[] | null>(null);
  const [checkList, setCheckList] = useState<CheckList | null>(null);

  useEffect(() => {
    if (items !== null) return;
    setItems(getItems(store));
  }, [items, store]);

  useEffect(() => {
    if (checkList !== null) return;
    const today = new Date().toISOString().slice(0, 10);
    const created = store.checkLists[today] ?? newCheckList({ date: today });
    addCheckList(store, created);
    setCheckList({ ...created });
  }, [checkList, store]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      if (checkList === null) return;
      updateCheckList(
        store,
        checkList.date,
        item.id,
        !checkList.checked[item.id]
      );
      setCheckList({ ...checkList });
    },
    [checkList, store]
  );

  const handleFABOnPress = useCallback(() => {
    if (items === null) return;
    const item = newItem({ name: `Item ${items.length}` });
    addItem(store, item);
    setItems([...items, item]);
  }, [items, store]);

  return {
    checked: checkList?.checked ?? {},
    handleButtonOnPress,
    handleFABOnPress,
    handleListItemOnPress,
    items,
  };
}
