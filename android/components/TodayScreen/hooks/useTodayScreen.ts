import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "../../StoreContext";
import { newItem, type Item, type ItemId } from "../types/item";
import {
  addItem,
  getCheckedItemIdsByDate,
  getItems as storeGetItems,
  setChecked as storeSetChecked,
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
  const [checked, setChecked] = useState<Record<ItemId, boolean> | null>(null);

  useEffect(() => {
    if (items !== null) return;
    setItems(storeGetItems(store));
  }, [items, store]);

  useEffect(() => {
    if (checked !== null) return;
    const today = new Date().toISOString().slice(0, 10);
    const itemIds = getCheckedItemIdsByDate(store, today);
    setChecked(Object.fromEntries(itemIds.map((id) => [id, true])));
  }, [checked, store]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      if (checked === null) return;
      // TODO: today
      const today = new Date().toISOString().slice(0, 10);
      storeSetChecked(store, today, item.id, !checked[item.id]);
      checked[item.id] = !checked[item.id];
      setChecked({ ...checked });
    },
    [checked, store]
  );

  const handleFABOnPress = useCallback(() => {
    if (items === null) return;
    const item = newItem({ name: `Item ${items.length}` });
    addItem(store, item);
    setItems([...items, item]);
  }, [items, store]);

  return {
    checked: checked ?? {},
    handleButtonOnPress,
    handleFABOnPress,
    handleListItemOnPress,
    items,
  };
}
