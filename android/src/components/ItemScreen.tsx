import type { NativeStackScreenProps } from "@react-navigation/native-stack";
import React from "react";
import { FlatList, StyleSheet, Text, View } from "react-native";
import { Button, TextInput } from "react-native-paper";
import { useItemScreen } from "@/components/ItemScreen/hooks/useItemScreen";
import type { NativeStackParamList } from "@/types/navigation";

type Props = NativeStackScreenProps<NativeStackParamList, "Item">;

export function ItemScreen({
  route: {
    params: { itemId },
  },
}: Props): JSX.Element {
  const { data, editing, handleListItemOnPress, handleNameChangeText } =
    useItemScreen(itemId);
  if (data === null) {
    return <Text>Loading...</Text>;
  }
  const { checkLists, days, item } = data;
  return (
    <View style={styles.container}>
      <View>
        <Text>Item ID</Text>
        <Text>{item.id}</Text>
      </View>
      <View style={{ padding: 16 }}>
        <Text>Item name</Text>
        {editing !== null ? (
          <TextInput value={editing.name} onChangeText={handleNameChangeText} />
        ) : (
          <Text
            style={{
              borderBottomColor: "transparent",
              borderBottomWidth: 2,
              fontSize: 16,
              padding: 16,
            }}
          >
            {item.name}
          </Text>
        )}
      </View>
      <View>
        <Text>Last purchased</Text>
        <Text>{days !== null ? `${days} days ago` : "(none)"}</Text>
      </View>
      <View>
        <Text>Number of purchases</Text>
        <Text>{`${checkLists.length} times`}</Text>
      </View>
      <View>
        <Text>Purchase history</Text>
        <FlatList
          data={checkLists}
          keyExtractor={(checkList) => checkList.id}
          renderItem={({ item: checkList }) => (
            <Button onPress={handleListItemOnPress(checkList)}>
              {checkList.date}
            </Button>
          )}
        />
      </View>
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
