import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckList, CheckListId } from "@/types/check_list";
import type { Item } from "@/types/item";
import {
  findCheckList,
  findCheckedItemIdsByCheckListId,
  findItem,
} from "@/types/store";

export function useListScreen(checkListId: CheckListId): {
  handleListItemOnPress: (item: Item) => () => void;
  items: Item[];
} {
  const navigation = useNavigation();
  const { store } = useStore();
  const [checkList, setCheckList] = useState<CheckList | null>(null);
  const [items, setItems] = useState<Item[] | null>(null);

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
      const itemIds = await findCheckedItemIdsByCheckListId(store, checkListId);
      setItems(
        (
          await Promise.all(
            itemIds.map(
              (itemId): Promise<Item | null> => findItem(store, itemId),
            ),
          )
        ).filter((item): item is Item => item !== null),
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
