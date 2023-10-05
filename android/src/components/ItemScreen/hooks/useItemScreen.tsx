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
  // TODO: editing -> screenState.type === "editing"
  const [editing, setEditing] = useState<{ name: string } | null>(null);

  useEffect(() => {
    void (async () => {
      setScreenState(await handleScreenState(store, screenState));
    })();
  }, [store, screenState]);

  const handleListItemOnPress = useCallback(
    (checkList: CheckList) => () => {
      navigation.dispatch(
        StackActions.push("List", { checkListId: checkList.id })
      );
    },
    [navigation]
  );

  const handleNameChangeText = useCallback(
    (text: string) => {
      if (editing === null) return;
      setEditing({ ...editing, name: text });
    },
    [editing]
  );

  useEffect(() => {
    navigation.setOptions({
      headerRight: () => (
        <IconButton
          icon={editing === null ? "pencil" : "check"}
          onPress={() => {
            if (screenState.type !== "loaded") return;
            if (editing === null) {
              setEditing({ name: screenState.item.name });
            } else {
              setEditing(null);
              handle(store, {
                payload: {
                  item: {
                    ...screenState.item,
                    name: editing.name,
                  },
                },
                type: "setItem",
              });
              setScreenState({
                itemId: screenState.item.id,
                type: "initial",
              });
            }
          }}
          style={{ marginRight: -4 }}
        />
      ),
      headerTitle:
        screenState.type !== "loaded"
          ? "Item"
          : `Item ${screenState.item.name}`,
    });
  }, [editing, navigation, screenState, store]);

  return {
    data:
      screenState.type === "loaded"
        ? {
            checkLists: screenState.checkLists,
            days: screenState.days,
            item: screenState.item,
          }
        : null,
    editing,
    handleListItemOnPress,
    handleNameChangeText,
  };
}

async function handleScreenState(
  store: Store,
  screenState: ScreenState
): Promise<ScreenState> {
  switch (screenState.type) {
    case "initial": {
      const item = await findItem(store, screenState.itemId);
      if (item === null) throw new Error("FIXME");
      const checkListIds = await findCheckedCheckListIdsByItemId(
        store,
        screenState.itemId
      );
      const checkLists = (
        await Promise.all(
          checkListIds.map(
            (id): Promise<CheckList | null> => findCheckList(store, id)
          )
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
