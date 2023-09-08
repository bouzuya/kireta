import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import { StackActions } from "@react-navigation/native";
import React, { useCallback, useEffect, useState } from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Button, Checkbox, FAB, List } from "react-native-paper";
import type { BottomTabParamList } from "../types/navigation";

type Item = { id: number; name: string };

const store: {
  items: { allIds: number[]; byId: Record<number, Item> };
} = {
  items: {
    allIds: [],
    byId: {},
  },
};

function addItem(item: Item): void {
  store.items.allIds.push(item.id);
  store.items.byId[item.id] = item;
}

function useTodayScreen(navigation: Props["navigation"]): {
  checked: Record<number, boolean>;
  handleCheckboxOnPress: (item: Item) => () => void;
  handleFABOnPress: () => void;
  handleButtonOnPress: () => void;
  items: Item[] | null;
} {
  const [items, setItems] = useState<Item[] | null>(null);
  const [checked, setChecked] = useState<Record<number, boolean>>({});

  useEffect(() => {
    if (items !== null) return;
    setItems(store.items.allIds.map((id) => store.items.byId[id]));
  }, [items]);

  const handleButtonOnPress = useCallback(() => {
    navigation.dispatch(StackActions.push("Item"));
  }, [navigation]);

  const handleCheckboxOnPress = useCallback(
    (item: Item) => () => {
      setChecked((prev) => ({
        ...prev,
        [item.id]: !prev[item.id],
      }));
    },
    []
  );

  const handleFABOnPress = useCallback(() => {
    if (items === null) return;
    const item = { id: items.length, name: `Item ${items.length}` };
    addItem(item);
    setItems([...items, item]);
  }, [items]);

  return {
    checked,
    handleButtonOnPress,
    handleCheckboxOnPress,
    handleFABOnPress,
    items,
  };
}

type Props = BottomTabScreenProps<BottomTabParamList, "Today">;

export function TodayScreen({ navigation }: Props): JSX.Element {
  const {
    checked,
    handleButtonOnPress,
    handleCheckboxOnPress,
    handleFABOnPress,
    items,
  } = useTodayScreen(navigation);
  return (
    <View style={styles.container}>
      <Button onPress={handleButtonOnPress}>Item</Button>
      <FlatList
        data={items}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({ item }) => (
          <List.Item
            left={(props) => (
              <Checkbox
                onPress={handleCheckboxOnPress(item)}
                status={checked[item.id] ? "checked" : "unchecked"}
                {...props}
              />
            )}
            title={item.name}
          />
        )}
      />
      <FAB
        icon="plus"
        onPress={handleFABOnPress}
        style={{ position: "absolute", right: 16, bottom: 16 }}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignContent: "flex-start",
    alignItems: "stretch",
    display: "flex",
    flexDirection: "column",
    height: "100%",
    justifyContent: "flex-start",
    margin: 0,
    padding: 0,
    width: "100%",
  },
});
