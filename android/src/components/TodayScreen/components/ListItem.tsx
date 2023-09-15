import { StyleSheet, Text, View } from "react-native";
import { Checkbox, List } from "react-native-paper";
import type { Item } from "@/types/item";

type Props = {
  checked: boolean;
  days: number | null;
  item: Item;
  onCheckboxPress: () => void;
  onItemPress: () => void;
};

export function ListItem({
  checked,
  days,
  item,
  onCheckboxPress,
  onItemPress,
}: Props): JSX.Element {
  return (
    <List.Item
      left={(props) => (
        <Checkbox
          onPress={onCheckboxPress}
          status={checked ? "checked" : "unchecked"}
          {...props}
        />
      )}
      right={(_) => (
        <View style={styles.right}>
          <Text>{days !== null ? `${days} days ago` : "(none)"}</Text>
        </View>
      )}
      onPress={onItemPress}
      title={item.name}
    />
  );
}

const styles = StyleSheet.create({
  right: {
    alignItems: "center",
    flex: 1,
    flexDirection: "row",
    height: "100%",
    justifyContent: "flex-end",
    width: "100%",
  },
});
