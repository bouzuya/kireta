import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckList } from "@/types/check_list";
import type { Item, ItemId } from "@/types/item";
import {
  findCheckedCheckListIdsByItemId,
  findCheckList,
  findItem,
  type Store,
} from "@/types/store";

type ScreenState =
  | {
      itemId: ItemId;
      type: "initial";
    }
  | {
      checkLists: CheckList[];
      days: number | null;
      item: Item;
      type: "loaded";
    };

export function useItemScreen(itemId: ItemId): {
  data: {
    checkLists: CheckList[];
    days: number | null;
    item: Item;
  } | null;
  handleListItemOnPress: (checkList: CheckList) => () => void;
} {
  const [screenState, setScreenState] = useState<ScreenState>({
    itemId,
    type: "initial",
  });
  const { store } = useStore();
  const navigation = useNavigation();

  useEffect(() => {
    void (async () => {
      setScreenState(await handleScreenState(store, screenState));
    })();
  }, [store, screenState]);

  const handleListItemOnPress = useCallback(
    (checkList: CheckList) => () => {
      navigation.dispatch(
        StackActions.push("List", { checkListId: checkList.id }),
      );
    },
    [navigation],
  );

  useEffect(() => {
    navigation.setOptions({
      headerTitle:
        screenState.type !== "loaded"
          ? "Item"
          : `Item ${screenState.item.name}`,
    });
  }, [navigation, screenState]);

  return {
    data:
      screenState.type === "loaded"
        ? {
            checkLists: screenState.checkLists,
            days: screenState.days,
            item: screenState.item,
          }
        : null,
    handleListItemOnPress,
  };
}

async function handleScreenState(
  store: Store,
  screenState: ScreenState,
): Promise<ScreenState> {
  switch (screenState.type) {
    case "initial": {
      const item = await findItem(store, screenState.itemId);
      if (item === null) throw new Error("FIXME");
      const checkListIds = await findCheckedCheckListIdsByItemId(
        store,
        screenState.itemId,
      );
      const checkLists = (
        await Promise.all(
          checkListIds.map(
            (id): Promise<CheckList | null> => findCheckList(store, id),
          ),
        )
      )
        .filter((checkList): checkList is CheckList => checkList !== null)
        .sort(({ date: a }, { date: b }) => (a < b ? 1 : a === b ? 0 : -1));
      const days: number | null =
        checkLists[0] === undefined
          ? null
          : ((new Date().getTime() -
              new Date(checkLists[0].date + "T00:00:00Z").getTime()) /
              (86400 * 1000)) |
            0;
      return {
        checkLists,
        days,
        item,
        type: "loaded",
      };
    }
    case "loaded":
      return screenState;
    default:
      throw new Error("assert unknown screenState.type");
  }
}
