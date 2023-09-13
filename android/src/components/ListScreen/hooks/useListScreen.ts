import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckListId } from "@/types/check_list";
import type { Item } from "@/types/item";
import { findCheckedItemIdsByCheckListId, findItem } from "@/types/store";

export function useListScreen(checkListId: CheckListId): {
  handleListItemOnPress: (item: Item) => () => void;
  items: Item[];
} {
  const navigation = useNavigation();
  const { store } = useStore();
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
  return {
    handleListItemOnPress,
    items,
  };
}
