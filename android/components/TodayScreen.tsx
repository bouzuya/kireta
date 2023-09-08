import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import React from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Button, Checkbox, FAB, List } from "react-native-paper";
import type { BottomTabParamList } from "../types/navigation";
import { useTodayScreen } from "./TodayScreen/hooks";

type Props = BottomTabScreenProps<BottomTabParamList, "Today">;

export function TodayScreen(_: Props): JSX.Element {
  const {
    checked,
    handleButtonOnPress,
    handleCheckboxOnPress,
    handleFABOnPress,
    items,
  } = useTodayScreen();
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
