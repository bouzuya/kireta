import { StackActions, useNavigation } from "@react-navigation/native";
import { useCallback, useEffect, useState } from "react";
import { useStore } from "@/components/StoreContext";
import type { CheckListId } from "@/types/check_list";
import { newCheckList } from "@/types/check_list";
import type { DateString } from "@/types/date_string";
import { newItem, type Item } from "@/types/item";
import type { Store } from "@/types/store";
import {
  findAllItems,
  findCheckListByDate,
  findCheckedItemIdsByCheckListId,
  handle,
} from "@/types/store";

type ItemWithChecked = Item & { checked: boolean };

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
      itemWithCheckeds: ItemWithChecked[];
      type: "itemWithCheckedsLoaded";
    };

function handleScreenState(
  store: Store,
  screenState: ScreenState
): ScreenState {
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
      const foundOrCreated = (() => {
        const found = findCheckListByDate(store, date);
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
        items: findAllItems(store),
        type: "itemsLoaded",
      };
    }
    case "itemsLoaded": {
      const itemIds = findCheckedItemIdsByCheckListId(
        store,
        screenState.checkListId
      );
      return {
        checkListId: screenState.checkListId,
        itemWithCheckeds: itemIds.reduce(
          (acc, checkedItemId) => {
            const item = acc.find((i) => i.id === checkedItemId);
            if (item === undefined) return acc;
            return {
              ...acc,
              [checkedItemId]: {
                ...item,
                checked: true,
              },
            };
          },
          screenState.items.map((item) => ({ ...item, checked: false }))
        ),
        type: "itemWithCheckedsLoaded",
      };
    }
    case "itemWithCheckedsLoaded": {
      // do nothing
      return screenState;
    }
    default:
      throw new Error();
  }
}

export function useTodayScreen(): {
  handleButtonOnPress: () => void;
  handleFABOnPress: () => void;
  handleListItemOnCheckboxPress: (item: Item) => () => void;
  handleListItemOnItemPress: (item: Item) => () => void;
  items: ItemWithChecked[] | null;
} {
  const [screenState, setScreenState] = useState<ScreenState>({
    type: "initial",
  });
  const { store } = useStore();
  const navigation = useNavigation();

  useEffect(() => {
    setScreenState(handleScreenState(store, screenState));
  }, [screenState, store]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleFABOnPress = useCallback(() => {
    if (screenState.type !== "itemWithCheckedsLoaded") return;
    const items = screenState.itemWithCheckeds;
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
      itemWithCheckeds: [...items, { ...item, checked: false }],
    });
  }, [screenState, store]);

  const handleListItemOnCheckboxPress = useCallback(
    (item: Item) => () => {
      if (screenState.type !== "itemWithCheckedsLoaded") return;
      const itemId = item.id;
      const items = screenState.itemWithCheckeds;
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
        itemWithCheckeds: items.map((item) => {
          if (item.id !== itemId) return item;
          return { ...item, checked };
        }),
      });
    },
    [screenState, store]
  );

  const handleListItemOnItemPress = useCallback(
    (item: Item) => () => {
      if (screenState.type !== "itemWithCheckedsLoaded") return;
      navigation.dispatch(StackActions.push("Item", { itemId: item.id }));
    },
    [navigation, screenState.type]
  );

  return {
    handleButtonOnPress,
    handleFABOnPress,
    handleListItemOnCheckboxPress,
    handleListItemOnItemPress,
    items:
      screenState.type === "itemWithCheckedsLoaded"
        ? screenState.itemWithCheckeds
        : null,
  };
}
