import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckList } from "@/types/check_list";
import { newCheckList } from "@/types/check_list";
import { newItem, type Item, type ItemId } from "@/types/item";
import {
  findAllItems,
  findCheckListByDate,
  findCheckedItemIdsByCheckListId,
  handle,
} from "@/types/store";

export function useTodayScreen(): {
  checked: Record<ItemId, boolean>;
  handleButtonOnPress: () => void;
  handleFABOnPress: () => void;
  handleListItemOnPress: (item: Item) => () => void;
  items: Item[] | null;
} {
  const { store } = useStore();
  const navigation = useNavigation();
  const [checkList, setCheckList] = useState<CheckList | null>(null);
  const [checked, setChecked] = useState<Record<ItemId, boolean> | null>(null);
  const [items, setItems] = useState<Item[] | null>(null);

  useEffect(() => {
    if (checkList !== null) return;
    const today = new Date().toISOString().slice(0, 10);
    const foundOrCreated = (() => {
      const found = findCheckListByDate(store, today);
      if (found !== null) {
        return found;
      } else {
        const created = newCheckList({ date: today });
        handle(store, {
          payload: {
            checkList: created,
          },
          type: "addCheckList",
        });
        return created;
      }
    })();
    setCheckList(foundOrCreated);
  }, [checkList, store]);

  useEffect(() => {
    if (items !== null) return;
    setItems(findAllItems(store));
  }, [items, store]);

  useEffect(() => {
    if (checked !== null) return;
    if (checkList === null) return;
    const itemIds = findCheckedItemIdsByCheckListId(store, checkList.id);
    setChecked(Object.fromEntries(itemIds.map((id) => [id, true])));
  }, [checkList, checked, store]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      if (checked === null) return;
      if (checkList === null) return;
      handle(store, {
        payload: {
          checked: !checked[item.id],
          checkListId: checkList.id,
          itemId: item.id,
        },
        type: "setChecked",
      });
      checked[item.id] = !checked[item.id];
      setChecked({ ...checked });
    },
    [checkList, checked, store]
  );

  const handleFABOnPress = useCallback(() => {
    if (items === null) return;
    const item = newItem({ name: `Item ${items.length}` });
    handle(store, {
      payload: {
        item,
      },
      type: "addItem",
    });
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
