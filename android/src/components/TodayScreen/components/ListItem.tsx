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
        <View style={{ paddingLeft: 8 }}>
          <Checkbox
            onPress={onCheckboxPress}
            status={checked ? "checked" : "unchecked"}
            {...props}
          />
        </View>
      )}
      right={(_) => (
        <View style={styles.right}>
          <Text>{days !== null ? `${days} days ago` : "(none)"}</Text>
        </View>
      )}
      onPress={onItemPress}
      title={item.name}
      titleStyle={styles.title}
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
  title: {
    marginTop: -3,
  },
});
