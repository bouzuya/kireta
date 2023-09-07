import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import { StackActions } from "@react-navigation/native";
import React, { useState } from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Button, Checkbox, FAB, List } from "react-native-paper";
import type { BottomTabParamList } from "../types/navigation";

type Props = BottomTabScreenProps<BottomTabParamList, "Today">;

export function TodayScreen({ navigation }: Props): JSX.Element {
  const [items, setItems] = useState<{ id: number; name: string }[]>([]);
  const [checked, setChecked] = useState<Record<number, boolean>>({});
  return (
    <View style={styles.container}>
      <Button
        onPress={() => {
          navigation.dispatch(StackActions.push("Item"));
        }}
      >
        Item
      </Button>
      <FlatList
        data={items}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({ item }) => (
          <List.Item
            left={(props) => (
              <Checkbox
                onPress={() => {
                  setChecked((prev) => ({
                    ...prev,
                    [item.id]: !prev[item.id],
                  }));
                }}
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
        onPress={() => {
          setItems((prev) => {
            const id = (prev[prev.length - 1]?.id ?? 0) + 1;
            return [...prev, { id, name: `Item ${id}` }];
          });
        }}
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
