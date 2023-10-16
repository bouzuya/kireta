import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { IconButton } from "react-native-paper";
import { useStore } from "@/components/StoreContext";
import type { CheckList } from "@/types/check_list";
import type { Item, ItemId } from "@/types/item";
import {
  findCheckedCheckListIdsByItemId,
  findCheckList,
  findItem,
  handle,
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
    }
  | {
      checkLists: CheckList[];
      days: number | null;
      item: Item;
      name: string;
      type: "editing";
    };

export function useItemScreen(itemId: ItemId): {
  data: {
    checkLists: CheckList[];
    days: number | null;
    item: Item;
  } | null;
  editing: { name: string } | null;
  handleListItemOnPress: (checkList: CheckList) => () => void;
  handleNameChangeText: (text: string) => void;
} {
  const [screenState, setScreenState] = useState<ScreenState>({
    itemId,
    type: "initial",
  });
  const { store } = useStore();
  const navigation = useNavigation();

  const handleIconButtonOnPress = useCallback(() => {
    switch (screenState.type) {
      case "initial":
        return;
      case "loaded": {
        setScreenState({
          ...screenState,
          name: screenState.item.name,
          type: "editing",
        });
        return;
      }
      case "editing": {
        handle(store, {
          payload: {
            item: {
              ...screenState.item,
              name: screenState.name,
            },
          },
          type: "setItem",
        });
        setScreenState({
          itemId: screenState.item.id,
          type: "initial",
        });
        return;
      }
    }
  }, [screenState, store]);

  const handleListItemOnPress = useCallback(
    (checkList: CheckList) => () => {
      navigation.dispatch(
        StackActions.push("List", { checkListId: checkList.id }),
      );
    },
    [navigation],
  );

  const handleNameChangeText = useCallback(
    (text: string) => {
      if (screenState.type !== "editing") return;
      setScreenState({ ...screenState, name: text });
    },
    [screenState],
  );

  useEffect(() => {
    navigation.setOptions({
      headerRight: () => (
        <IconButton
          icon={screenState.type === "editing" ? "check" : "pencil"}
          onPress={handleIconButtonOnPress}
          style={{ marginRight: -4 }}
        />
      ),
      headerTitle: getHeaderTitle(screenState),
    });
  }, [handleIconButtonOnPress, navigation, screenState]);

  useEffect(() => {
    void (async () => {
      setScreenState(await handleScreenState(store, screenState));
    })();
  }, [store, screenState]);

  return {
    data:
      screenState.type === "loaded" || screenState.type === "editing"
        ? {
            checkLists: screenState.checkLists,
            days: screenState.days,
            item: screenState.item,
          }
        : null,
    editing: screenState.type === "editing" ? { name: screenState.name } : null,
    handleListItemOnPress,
    handleNameChangeText,
  };
}

function getHeaderTitle(screenState: ScreenState): string {
  return screenState.type === "initial"
    ? "Item"
    : `Item ${screenState.item.name}`;
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
    case "editing":
      return screenState;
    default:
      throw new Error("assert unknown screenState.type");
  }
}
