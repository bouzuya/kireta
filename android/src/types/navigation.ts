import type { ItemId } from "@/types/item";

export type BottomTabParamList = {
  History: undefined;
  Today: undefined;
};

export type NativeStackParamList = {
  Item: { itemId: ItemId };
  List: undefined;
  Tab: undefined;
};
