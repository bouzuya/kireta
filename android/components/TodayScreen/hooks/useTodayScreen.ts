import { StackActions, useNavigation } from "@react-navigation/native";
import { randomUUID } from "expo-crypto";
import { useCallback, useEffect, useState } from "react";
import type { Item, ItemId } from "../types/item";

const store: {
  items: { allIds: ItemId[]; byId: Record<ItemId, Item> };
} = {
  items: {
    allIds: [],
    byId: {},
  },
};

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
  const [checked, setChecked] = useState<Record<ItemId, boolean>>({});

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
    const item = newItem({ name: `Item ${items.length}` });
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
