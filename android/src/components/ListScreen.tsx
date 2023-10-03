import type { NativeStackScreenProps } from "@react-navigation/native-stack";
import React from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Checkbox, List } from "react-native-paper";
import { useListScreen } from "@/components/ListScreen/hooks/useListScreen";
import type { NativeStackParamList } from "@/types/navigation";

type Props = NativeStackScreenProps<NativeStackParamList, "List">;

export function ListScreen({
  route: {
    params: { checkListId },
  },
}: Props): JSX.Element {
  const { handleListItemOnPress, items } = useListScreen(checkListId);
  return (
    <View style={styles.container}>
      <FlatList
        data={items}
        keyExtractor={(item) => item.id}
        renderItem={({ item }) => (
          <List.Item
            left={(props) => (
              <View style={{ paddingLeft: 8 }}>
                <Checkbox
                  disabled={true}
                  status={item.checked ? "checked" : "unchecked"}
                  {...props}
                />
              </View>
            )}
            onPress={handleListItemOnPress(item)}
            title={item.name}
            titleStyle={{ marginTop: -3 }}
          />
        )}
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
