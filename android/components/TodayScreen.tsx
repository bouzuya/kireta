import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import React from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Button, FAB } from "react-native-paper";
import type { BottomTabParamList } from "../types/navigation";
import { ListItem } from "./TodayScreen/components/ListItem";
import { useTodayScreen } from "./TodayScreen/hooks/useTodayScreen";

type Props = BottomTabScreenProps<BottomTabParamList, "Today">;

export function TodayScreen(_: Props): JSX.Element {
  const {
    checked,
    handleButtonOnPress,
    handleFABOnPress,
    handleListItemOnPress,
    items,
  } = useTodayScreen();
  return (
    <View style={styles.container}>
      <Button onPress={handleButtonOnPress}>Item</Button>
      <FlatList
        data={items}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({ item }) => (
          <ListItem
            checked={checked[item.id]}
            item={item}
            onPress={handleListItemOnPress(item)}
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
