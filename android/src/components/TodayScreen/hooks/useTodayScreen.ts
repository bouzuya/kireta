import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckList, CheckListId } from "@/types/check_list";
import { newCheckList } from "@/types/check_list";
import type { DateString } from "@/types/date_string";
import { newItem } from "@/types/item";
import type { Item, ItemId } from "@/types/item";
import type { Store } from "@/types/store";
import {
  findAllItems,
  findCheckList,
  findCheckListByDate,
  findCheckedCheckListIdsByItemId,
  findCheckedItemIdsByCheckListId,
  handle,
} from "@/types/store";

type ItemForToday = Item & { checked: boolean } & { days: number | null };

type ScreenState =
  | {
      type: "initial";
    }
  | {
      date: DateString;
      type: "initialized";
    }
  | {
      checkListId: CheckListId;
      type: "checkListIdLoaded";
    }
  | {
      checkListId: CheckListId;
      items: Item[];
      type: "itemsLoaded";
    }
  | {
      checkListId: CheckListId;
      itemWithCheckeds: (Item & { checked: boolean })[];
      type: "itemWithCheckedsLoaded";
    }
  | {
      checkListId: CheckListId;
      items: ItemForToday[];
      type: "itemForTodayLoaded";
    };

async function getDays(store: Store, itemId: ItemId): Promise<number | null> {
  const checkListIds = await findCheckedCheckListIdsByItemId(store, itemId);
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

  return days;
}

async function handleScreenState(
  store: Store,
  screenState: ScreenState,
): Promise<ScreenState> {
  const type = screenState.type;
  switch (type) {
    case "initial": {
      return {
        date: new Date().toISOString().slice(0, 10),
        type: "initialized",
      };
    }
    case "initialized": {
      const date = screenState.date;
      const foundOrCreated = await (async () => {
        const found = await findCheckListByDate(store, date);
        if (found !== null) {
          return found;
        } else {
          const created = newCheckList({ date });
          handle(store, {
            payload: {
              checkList: created,
            },
            type: "addCheckList",
          });
          return created;
        }
      })();
      return {
        checkListId: foundOrCreated.id,
        type: "checkListIdLoaded",
      };
    }
    case "checkListIdLoaded": {
      return {
        checkListId: screenState.checkListId,
        items: await findAllItems(store),
        type: "itemsLoaded",
      };
    }
    case "itemsLoaded": {
      const itemIds = await findCheckedItemIdsByCheckListId(
        store,
        screenState.checkListId,
      );
      return {
        checkListId: screenState.checkListId,
        itemWithCheckeds: itemIds.reduce(
          (acc, checkedItemId) => {
            return acc.map((item) => {
              return item.id === checkedItemId
                ? { ...item, checked: true }
                : item;
            });
          },
          screenState.items.map((item) => ({ ...item, checked: false })),
        ),
        type: "itemWithCheckedsLoaded",
      };
    }
    case "itemWithCheckedsLoaded": {
      // TODO: too slow
      const items = await Promise.all(
        screenState.itemWithCheckeds.map(async (item) => {
          const days = await getDays(store, item.id);
          return { ...item, days };
        }),
      );
      return {
        checkListId: screenState.checkListId,
        items,
        type: "itemForTodayLoaded",
      };
    }
    case "itemForTodayLoaded": {
      // do nothing
      return screenState;
    }
    default:
      throw new Error();
  }
}

export function useTodayScreen(): {
  handleFABOnPress: () => void;
  handleFlatListOnRefresh: () => void;
  handleListItemOnCheckboxPress: (item: Item) => () => void;
  handleListItemOnItemPress: (item: Item) => () => void;
  items: ItemForToday[] | null;
  refreshing: boolean;
} {
  const [screenState, setScreenState] = useState<ScreenState>({
    type: "initial",
  });
  const { store } = useStore();
  const navigation = useNavigation();
  const [refreshing, setRefreshing] = useState<boolean>(false);

  const handleFABOnPress = useCallback(() => {
    if (screenState.type !== "itemForTodayLoaded") return;
    const items = screenState.items;
    const item = newItem({ name: `Item ${items.length}` });

    // update store
    handle(store, {
      payload: {
        item,
      },
      type: "addItem",
    });

    // update state
    setScreenState({
      ...screenState,
      items: [...items, { ...item, checked: false, days: null }],
    });
  }, [screenState, store]);

  const handleFlatListOnRefresh = useCallback(() => {
    setRefreshing(true);
    try {
      setScreenState({ type: "initial" });
    } finally {
      setRefreshing(false);
    }
  }, []);

  const handleListItemOnCheckboxPress = useCallback(
    (item: Item) => () => {
      if (screenState.type !== "itemForTodayLoaded") return;
      const itemId = item.id;
      const items = screenState.items;
      const checked = !(items.find((i) => i.id === itemId)?.checked ?? false);
      const checkListId = screenState.checkListId;

      // update store
      handle(store, {
        payload: { checked, checkListId, itemId },
        type: "setChecked",
      });

      // update state
      setScreenState({
        ...screenState,
        items: items.map((item) => {
          if (item.id !== itemId) return item;
          return { ...item, checked };
        }),
      });
    },
    [screenState, store],
  );

  const handleListItemOnItemPress = useCallback(
    (item: Item) => () => {
      if (screenState.type !== "itemForTodayLoaded") return;
      navigation.dispatch(StackActions.push("Item", { itemId: item.id }));
    },
    [navigation, screenState.type],
  );

  useEffect(() => {
    void (async () => {
      setScreenState(await handleScreenState(store, screenState));
    })();
  }, [screenState, store]);

  useEffect(() => {
    if (screenState.type !== "initialized") return;
    navigation.setOptions({
      headerTitle: `Today ${screenState.date}`,
    });
  }, [navigation, screenState]);

  return {
    handleFABOnPress,
    handleFlatListOnRefresh,
    handleListItemOnCheckboxPress,
    handleListItemOnItemPress,
    items: screenState.type === "itemForTodayLoaded" ? screenState.items : null,
    refreshing,
  };
}
