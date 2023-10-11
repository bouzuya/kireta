import type { NativeStackScreenProps } from "@react-navigation/native-stack";
import React from "react";
import { FlatList, StyleSheet, Text, View } from "react-native";
import { List, TextInput } from "react-native-paper";
import { LabeledValue } from "@/components/ItemScreen/components/LabeledValue";
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
  return (
    <View style={styles.container}>
      <LabeledValue
        label="Item ID"
        value={data === null ? " " : data.item.id}
      />
      <View style={styles.labeledValueContainer}>
        <Text>Item name</Text>
        {editing !== null ? (
          <TextInput value={editing.name} onChangeText={handleNameChangeText} />
        ) : (
          <Text style={styles.valueText}>
            {data === null ? " " : data.item.name}
          </Text>
        )}
      </View>
      <LabeledValue
        label="Last purchased"
        value={
          data === null
            ? " "
            : data.days === null
            ? "(none)"
            : `${data.days} days ago`
        }
      />
      <LabeledValue
        label="Number of purchases"
        value={data === null ? " " : `${data.checkLists.length} times`}
      />
      <View style={styles.labeledValueContainer}>
        <Text>Purchase history</Text>
        <FlatList
          data={data?.checkLists ?? []}
          keyExtractor={(checkList) => checkList.id}
          renderItem={({ item: checkList }) => (
            <List.Item
              onPress={handleListItemOnPress(checkList)}
              title={checkList.date}
            />
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
    paddingHorizontal: 0,
    paddingVertical: 16,
    width: "100%",
  },
  labeledValueContainer: {
    paddingHorizontal: 16,
  },
  valueText: {
    borderBottomColor: "transparent",
    borderBottomWidth: 2,
    fontSize: 16,
    padding: 16,
  },
});
