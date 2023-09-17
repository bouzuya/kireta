import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckListId } from "@/types/check_list";
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
  const checkList = findCheckList(store, checkListId);
  const itemIds = findCheckedItemIdsByCheckListId(store, checkListId);
  const items = itemIds
    .map((itemId): Item | null => findItem(store, itemId))
    .filter((item): item is Item => item !== null);

  const handleListItemOnPress = useCallback(
    (item: Item) => () => {
      navigation.dispatch(StackActions.push("Item", { itemId: item.id }));
    },
    [navigation]
  );

  useEffect(() => {
    navigation.setOptions({
      headerTitle: checkList === null ? "List" : `List ${checkList.date}`,
    });
  }, [checkList, navigation]);

  return {
    handleListItemOnPress,
    items,
  };
}
