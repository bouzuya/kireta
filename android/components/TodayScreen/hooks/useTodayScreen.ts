import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import type { Item } from "../types/item";

const store: {
  items: { allIds: number[]; byId: Record<number, Item> };
} = {
  items: {
    allIds: [],
    byId: {},
  },
};

function addItem(item: Item): void {
  store.items.allIds.push(item.id);
  store.items.byId[item.id] = item;
}

export function useTodayScreen(): {
  checked: Record<number, boolean>;
  handleButtonOnPress: () => void;
  handleFABOnPress: () => void;
  handleListItemOnPress: (item: Item) => () => void;
  items: Item[] | null;
} {
  const navigation = useNavigation();
  const [items, setItems] = useState<Item[] | null>(null);
  const [checked, setChecked] = useState<Record<number, boolean>>({});

  useEffect(() => {
    if (items !== null) return;
    setItems(store.items.allIds.map((id) => store.items.byId[id]));
  }, [items]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      setChecked((prev) => ({
        ...prev,
        [item.id]: !prev[item.id],
      }));
    },
    []
  );

  const handleFABOnPress = useCallback(() => {
    if (items === null) return;
    const item = { id: items.length, name: `Item ${items.length}` };
    addItem(item);
    setItems([...items, item]);
  }, [items]);

  return {
    checked,
    handleButtonOnPress,
    handleFABOnPress,
    handleListItemOnPress,
    items,
  };
}
