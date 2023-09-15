import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import React from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { FAB } from "react-native-paper";
import { ListItem } from "@/components/TodayScreen/components/ListItem";
import { useTodayScreen } from "@/components/TodayScreen/hooks/useTodayScreen";
import type { BottomTabParamList } from "@/types/navigation";

type Props = BottomTabScreenProps<BottomTabParamList, "Today">;

export function TodayScreen(_: Props): JSX.Element {
  const {
    handleFABOnPress,
    handleListItemOnCheckboxPress,
    handleListItemOnItemPress,
    items,
  } = useTodayScreen();
  return (
    <View style={styles.container}>
      <FlatList
        data={items}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({ item }) => (
          <ListItem
            checked={item.checked}
            days={item.days}
            item={item}
            onCheckboxPress={handleListItemOnCheckboxPress(item)}
            onItemPress={handleListItemOnItemPress(item)}
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
