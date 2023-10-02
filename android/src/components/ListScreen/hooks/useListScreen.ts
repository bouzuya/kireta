import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckList, CheckListId } from "@/types/check_list";
import type { Item } from "@/types/item";
import {
  findAllItems,
  findCheckList,
  findCheckedItemIdsByCheckListId,
} from "@/types/store";

type ItemWithChecked = Item & { checked: boolean };

export function useListScreen(checkListId: CheckListId): {
  handleListItemOnPress: (item: Item) => () => void;
  items: ItemWithChecked[];
} {
  const navigation = useNavigation();
  const { store } = useStore();
  const [checkList, setCheckList] = useState<CheckList | null>(null);
  const [items, setItems] = useState<ItemWithChecked[] | null>(null);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      navigation.dispatch(StackActions.push("Item", { itemId: item.id }));
    },
    [navigation],
  );

  useEffect(() => {
    void (async () => {
      setCheckList(await findCheckList(store, checkListId));
    })();
  }, [checkListId, store]);

  useEffect(() => {
    void (async () => {
      const items = await findAllItems(store);
      const itemIds = await findCheckedItemIdsByCheckListId(store, checkListId);
      setItems(
        items.map((item) => {
          return { ...item, checked: itemIds.includes(item.id) };
        }),
      );
    })();
  }, [checkListId, store]);

  useEffect(() => {
    navigation.setOptions({
      headerTitle: checkList === null ? "List" : `List ${checkList.date}`,
    });
  }, [checkList, navigation]);

  return {
    handleListItemOnPress,
    // TODO
    items: items ?? [],
  };
}
