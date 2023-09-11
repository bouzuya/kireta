import { Checkbox, List } from "react-native-paper";
import type { Item } from "@/types/item";

type Props = {
  checked: boolean;
  item: Item;
  onCheckboxPress: () => void;
  onItemPress: () => void;
};

export function ListItem({
  checked,
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
      onPress={onItemPress}
      title={item.name}
    />
  );
}
